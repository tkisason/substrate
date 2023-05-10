// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Storage map type. Implements StorageMap, StorageIterableMap, StoragePrefixedMap traits and their
//! methods directly.

#![allow(unused_imports)] // FAIL-CI

use crate::{
	defensive,
	storage::{KeyLenOf, StorageAppend, StorageDecodeLength, StoragePrefixedMap, StorageTryAppend},
	traits::{Defensive, Get, GetDefault, StorageInfo, StorageInstance},
	CloneNoBound, DebugNoBound, DefaultNoBound, EqNoBound, PartialEqNoBound, StorageHasher,
	Twox128,
};
use codec::{Decode, Encode, EncodeLike, FullCodec, MaxEncodedLen};
use sp_arithmetic::traits::{SaturatedConversion, Saturating};
use sp_std::prelude::*;

pub type PageIndex = u32;
pub type ValueIndex = u32;

/// A paginated storage list that packs multiple elements per page.
///
/// The map is lined by a storage lookup of `(prefix, page_index)`. See [`StoragePagedListMeta`].
///
/// Supported APIs are iteration and appending.
pub struct StoragePagedList<Prefix, Hasher, Value, ValuesPerPage, MaxPages = GetDefault> {
	_phantom: core::marker::PhantomData<(Prefix, Hasher, Value, ValuesPerPage, MaxPages)>,
}

/// State info for book-keeping. Can be used as append-only iterator.
#[derive(
	Encode, Decode, CloneNoBound, PartialEqNoBound, EqNoBound, DebugNoBound, DefaultNoBound,
)]
// todo ignore scale bounds
pub struct StoragePagedListMeta<Prefix, Value, Hasher, ValuesPerPage> {
	current_page: PageIndex,
	current_value: ValueIndex, // inside `current_page`

	_phantom: core::marker::PhantomData<(Prefix, Value, Hasher, ValuesPerPage)>,
}

impl<Prefix, Value, Hasher, ValuesPerPage>
	StoragePagedListMeta<Prefix, Value, Hasher, ValuesPerPage>
where
	Prefix: StorageInstance,
	Value: FullCodec,
	Hasher: StorageHasher,
	ValuesPerPage: Get<u32>,
{
	pub fn from_storage() -> Option<Self> {
		let key = Self::key();
		let raw = sp_io::storage::get(key.as_ref()).unwrap_or_default();
		Self::decode(&mut &raw[..]).ok()
	}

	pub fn key() -> Hasher::Output {
		(Prefix::STORAGE_PREFIX, b"paged_list_state").using_encoded(Hasher::hash)
	}

	pub fn append<EncodeLikeItem>(&mut self, item: EncodeLikeItem)
	where
		EncodeLikeItem: EncodeLike<Value>,
	{
		if self.current_value == ValuesPerPage::get() {
			self.current_value = 0;
			self.current_page.saturating_inc();
		}
		let key = page_key::<Prefix, Hasher>(self.current_page);
		// Pages are `Vec` and therefore appendable.
		sp_io::storage::append(key.as_ref(), item.encode());
	}
}

// invariant: empty pages do not exist
pub struct Page<V> {
	current: PageIndex,
	values: sp_std::vec::Vec<V>,
}

impl<V: FullCodec> Page<V> {
	fn from_storage<Prefix: StorageInstance, Hasher: StorageHasher>(
		index: PageIndex,
	) -> Option<Self> {
		let key = page_key::<Prefix, Hasher>(index);
		let values = sp_io::storage::get(key.as_ref())
			.map(|raw| sp_std::vec::Vec::<V>::decode(&mut &raw[..]).ok())
			.flatten();
		let values = values.unwrap_or_default();
		if values.is_empty() {
			None
		} else {
			Some(Self { current: 0, values })
		}
	}
}

// Note: does not live under `Page` since it does not require `Value`.
pub fn page_key<Prefix: StorageInstance, Hasher: StorageHasher>(
	index: PageIndex,
) -> Hasher::Output {
	(Prefix::STORAGE_PREFIX, index).using_encoded(Hasher::hash)
}

impl<V: Clone> Iterator for Page<V> {
	type Item = V;

	fn next(&mut self) -> Option<Self::Item> {
		let val = self.values.get(self.current as usize).cloned();
		self.current.saturating_inc();
		val
	}
}

pub struct StoragePagedListIterator<Prefix, Value, Hasher, ValuesPerPage> {
	// Design: we put the Page into the iterator to have fewer storage look-ups. Yes, these
	// look-ups would be cached anyway, but bugging the overlay on each `.next` call still seems
	// like a poor trade-off than caching it in the iterator directly. Note: if Page is empty then
	// the iterator did not find any data upon setup or ran out of pages.
	page: Option<Page<Value>>,
	current_page: PageIndex,
	_phantom: core::marker::PhantomData<(Prefix, Value, Hasher, ValuesPerPage)>,
}

impl<Prefix, Value, Hasher, ValuesPerPage>
	StoragePagedListIterator<Prefix, Value, Hasher, ValuesPerPage>
where
	Prefix: StorageInstance,
	Value: FullCodec + Clone,
	Hasher: StorageHasher,
	ValuesPerPage: Get<u32>,
{
	pub fn from_storage() -> Self {
		let page = Page::<Value>::from_storage::<Prefix, Hasher>(0);

		Self { page, current_page: 0, _phantom: Default::default() }
	}
}

impl<Prefix, Value, Hasher, ValuesPerPage> Iterator
	for StoragePagedListIterator<Prefix, Value, Hasher, ValuesPerPage>
where
	Prefix: StorageInstance,
	Value: FullCodec + Clone,
	Hasher: StorageHasher,
	ValuesPerPage: Get<u32>,
{
	type Item = Value;

	fn next(&mut self) -> Option<Self::Item> {
		let Some(ref mut page) = self.page else {
			return None;
		};
		if let Some(val) = page.next() {
			return Some(val)
		}

		self.current_page.saturating_inc();
		let Some(mut page) = Page::from_storage::<Prefix, Hasher>(self.current_page) else {
			// No more pages, this is fine.
			return None;
		};

		if let Some(val) = page.next() {
			self.page = Some(page);
			return Some(val)
		}

		defensive!("StoragePagedListIterator: empty pages do not exist: storage corrupted");
		self.page = None;
		None
	}
}

impl<Prefix, Hasher, Value, ValuesPerPage, MaxPages> crate::storage::IterableStorageList<Value>
	for StoragePagedList<Prefix, Hasher, Value, ValuesPerPage, MaxPages>
where
	Prefix: StorageInstance,
	Value: FullCodec + Clone,
	Hasher: StorageHasher,
	ValuesPerPage: Get<u32>,
	MaxPages: Get<Option<u32>>,
{
	type Iterator = StoragePagedListIterator<Prefix, Value, Hasher, ValuesPerPage>;

	fn iter() -> Self::Iterator {
		StoragePagedListIterator::from_storage()
	}
}

impl<Prefix, Hasher, Value, ValuesPerPage, MaxPages>
	StoragePagedList<Prefix, Hasher, Value, ValuesPerPage, MaxPages>
where
	Prefix: StorageInstance,
	Value: FullCodec + Clone,
	Hasher: StorageHasher,
	ValuesPerPage: Get<u32>,
	MaxPages: Get<Option<u32>>,
{
	fn read_meta() -> StoragePagedListMeta<Prefix, Value, Hasher, ValuesPerPage> {
		// We unwrap here to not require a setup migration.
		StoragePagedListMeta::from_storage().unwrap_or_default()
	}

	/// Provides a fast append iterator.
	///
	/// The list should not be modified while appending. Also don't call it recursively.
	pub fn appendix() -> StoragePagedListMeta<Prefix, Value, Hasher, ValuesPerPage> {
		Self::read_meta()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		hash::*,
		metadata_ir::{StorageEntryModifierIR, StorageEntryTypeIR, StorageHasherIR},
		parameter_types,
		storage::{types::ValueQuery, IterableStorageList},
	};
	use sp_io::{hashing::twox_128, TestExternalities};

	parameter_types! {
		pub const ValuesPerPage: u32 = 2;
		pub const MaxPages: Option<u32> = Some(20);
	}

	struct Prefix;
	impl StorageInstance for Prefix {
		fn pallet_prefix() -> &'static str {
			"test"
		}
		const STORAGE_PREFIX: &'static str = "foo";
	}

	type List = StoragePagedList<Prefix, Blake2_128Concat, u32, ValuesPerPage, MaxPages>;

	#[test]
	fn basic_works() {
		TestExternalities::default().execute_with(|| {
			let mut append_iter = List::appendix();
			for i in 0..1000 {
				append_iter.append(i);
			}

			let vals = List::iter().collect::<Vec<_>>();
			assert_eq!(vals, (0..1000).collect::<Vec<_>>());
		});
	}
}
