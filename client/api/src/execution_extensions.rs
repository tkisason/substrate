// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Execution extensions for runtime calls.
//!
//! This module is responsible for defining the execution
//! strategy for the runtime calls and provide the right `Externalities`
//! extensions to support APIs for particular execution context & capabilities.

use codec::Decode;
use parking_lot::RwLock;
use sc_transaction_pool_api::OffchainSubmitTransaction;
use sp_core::offchain::{self, Capabilities, OffchainDbExt, OffchainWorkerExt, TransactionPoolExt};
use sp_externalities::{Extension, Extensions};
use sp_keystore::{KeystoreExt, SyncCryptoStorePtr};
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, NumberFor},
};
use sp_state_machine::DefaultHandler;
use std::{
	marker::PhantomData,
	sync::{Arc, Weak},
};

/// Generate the starting set of [`Extensions`].
///
/// These [`Extensions`] are passed to the environment a runtime is executed in.
pub trait ExtensionsFactory<Block: BlockT>: Send + Sync {
	/// Create [`Extensions`] for the given input.
	///
	/// - `block_hash`: The hash of the block in the context that extensions will be used.
	/// - `block_number`: The number of the block in the context that extensions will be used.
	/// - `capabilities`: The capabilities
	fn extensions_for(&self, block_hash: Block::Hash, block_number: NumberFor<Block>)
		-> Extensions;
}

impl<Block: BlockT> ExtensionsFactory<Block> for () {
	fn extensions_for(&self, _: Block::Hash, _: NumberFor<Block>) -> Extensions {
		Extensions::new()
	}
}

impl<Block: BlockT, T: ExtensionsFactory<Block>> ExtensionsFactory<Block> for Vec<T> {
	fn extensions_for(
		&self,
		block_hash: Block::Hash,
		block_number: NumberFor<Block>,
	) -> Extensions {
		let mut exts = Extensions::new();
		exts.extend(self.iter().map(|e| e.extensions_for(block_hash, block_number)));
		exts
	}
}

/// An [`ExtensionsFactory`] that registers an [`Extension`] before a certain block.
pub struct ExtensionBeforeBlock<Block: BlockT, Ext> {
	before: NumberFor<Block>,
	_marker: PhantomData<fn(Ext) -> Ext>,
}

impl<Block: BlockT, Ext> ExtensionBeforeBlock<Block, Ext> {
	/// Create the extension factory.
	///
	/// - `before`: The block number until the extension should be registered.
	pub fn new(before: NumberFor<Block>) -> Self {
		Self { before, _marker: PhantomData }
	}
}

impl<Block: BlockT, Ext: Default + Extension> ExtensionsFactory<Block>
	for ExtensionBeforeBlock<Block, Ext>
{
	fn extensions_for(&self, _: Block::Hash, block_number: NumberFor<Block>) -> Extensions {
		let mut exts = Extensions::new();

		if block_number < self.before {
			exts.register(Ext::default());
		}

		exts
	}
}

/// A producer of execution extensions for offchain calls.
///
/// This crate aggregates extensions available for the offchain calls
/// and is responsible for producing a correct `Extensions` object.
/// for each call, based on required `Capabilities`.
pub struct ExecutionExtensions<Block: BlockT> {
	extensions_factory: RwLock<Box<dyn ExtensionsFactory<Block>>>,
}

impl<Block: BlockT> Default for ExecutionExtensions<Block> {
	fn default() -> Self {
		Self { extensions_factory: RwLock::new(Box::new(())) }
	}
}

impl<Block: BlockT> ExecutionExtensions<Block> {
	/// Create new `ExecutionExtensions` given a `keystore` and `ExecutionStrategies`.
	pub fn new() -> Self {
		Self::default()
	}

	/// Based on the execution context and capabilities it produces
	/// the extensions object to support desired set of APIs.
	pub fn extensions(
		&self,
		block_hash: Block::Hash,
		block_number: NumberFor<Block>,
	) -> Extensions {
		let extensions = self.extensions_factory.read().extensions_for(block_hash, block_number);

		extensions
	}
}
