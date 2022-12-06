// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm2`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/staking/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_staking.
pub trait WeightInfo {
	fn bond() -> Weight;
	fn bond_extra() -> Weight;
	fn unbond() -> Weight;
	fn withdraw_unbonded_update(s: u32, ) -> Weight;
	fn withdraw_unbonded_kill(s: u32, ) -> Weight;
	fn validate() -> Weight;
	fn kick(k: u32, ) -> Weight;
	fn nominate(n: u32, ) -> Weight;
	fn chill() -> Weight;
	fn set_payee() -> Weight;
	fn set_controller() -> Weight;
	fn set_validator_count() -> Weight;
	fn force_no_eras() -> Weight;
	fn force_new_era() -> Weight;
	fn force_new_era_always() -> Weight;
	fn set_invulnerables(v: u32, ) -> Weight;
	fn force_unstake(s: u32, ) -> Weight;
	fn cancel_deferred_slash(s: u32, ) -> Weight;
	fn payout_stakers_dead_controller(n: u32, ) -> Weight;
	fn payout_stakers_alive_staked(n: u32, ) -> Weight;
	fn rebond(l: u32, ) -> Weight;
	fn reap_stash(s: u32, ) -> Weight;
	fn new_era(v: u32, n: u32, ) -> Weight;
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight;
	fn get_npos_targets(v: u32, ) -> Weight;
	fn set_staking_configs_all_set() -> Weight;
	fn set_staking_configs_all_remove() -> Weight;
	fn chill_other() -> Weight;
	fn force_apply_min_commission() -> Weight;
}

/// Weights for pallet_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	fn bond() -> Weight {
		// Minimum execution time: 53_097 nanoseconds.
		Weight::from_ref_time(53_708_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: VoterList ListBags (r:2 w:2)
	fn bond_extra() -> Weight {
		// Minimum execution time: 92_199 nanoseconds.
		Weight::from_ref_time(93_541_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListBags (r:2 w:2)
	fn unbond() -> Weight {
		// Minimum execution time: 98_227 nanoseconds.
		Weight::from_ref_time(99_070_000 as u64)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		// Minimum execution time: 45_058 nanoseconds.
		Weight::from_ref_time(46_592_713 as u64)
			// Standard Error: 413
			.saturating_add(Weight::from_ref_time(63_036 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_kill(_s: u32, ) -> Weight {
		// Minimum execution time: 86_087 nanoseconds.
		Weight::from_ref_time(87_627_894 as u64)
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(11 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MinValidatorBond (r:1 w:0)
	// Storage: Staking MinCommission (r:1 w:0)
	// Storage: Staking Validators (r:1 w:1)
	// Storage: Staking MaxValidatorsCount (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListNodes (r:1 w:1)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking CounterForValidators (r:1 w:1)
	fn validate() -> Weight {
		// Minimum execution time: 67_690 nanoseconds.
		Weight::from_ref_time(68_348_000 as u64)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	/// The range of component `k` is `[1, 128]`.
	fn kick(k: u32, ) -> Weight {
		// Minimum execution time: 43_512 nanoseconds.
		Weight::from_ref_time(47_300_477 as u64)
			// Standard Error: 11_609
			.saturating_add(Weight::from_ref_time(6_770_405 as u64).saturating_mul(k as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	/// The range of component `n` is `[1, 16]`.
	fn nominate(n: u32, ) -> Weight {
		// Minimum execution time: 74_296 nanoseconds.
		Weight::from_ref_time(73_201_782 as u64)
			// Standard Error: 5_007
			.saturating_add(Weight::from_ref_time(2_810_370 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	fn chill() -> Weight {
		// Minimum execution time: 66_605 nanoseconds.
		Weight::from_ref_time(67_279_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Payee (r:0 w:1)
	fn set_payee() -> Weight {
		// Minimum execution time: 18_897 nanoseconds.
		Weight::from_ref_time(19_357_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:2 w:2)
	fn set_controller() -> Weight {
		// Minimum execution time: 26_509 nanoseconds.
		Weight::from_ref_time(26_961_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Staking ValidatorCount (r:0 w:1)
	fn set_validator_count() -> Weight {
		// Minimum execution time: 5_025 nanoseconds.
		Weight::from_ref_time(5_240_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_no_eras() -> Weight {
		// Minimum execution time: 5_107 nanoseconds.
		Weight::from_ref_time(5_320_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era() -> Weight {
		// Minimum execution time: 5_094 nanoseconds.
		Weight::from_ref_time(5_377_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era_always() -> Weight {
		// Minimum execution time: 5_219 nanoseconds.
		Weight::from_ref_time(5_434_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking Invulnerables (r:0 w:1)
	/// The range of component `v` is `[0, 1000]`.
	fn set_invulnerables(v: u32, ) -> Weight {
		// Minimum execution time: 5_122 nanoseconds.
		Weight::from_ref_time(5_977_533 as u64)
			// Standard Error: 34
			.saturating_add(Weight::from_ref_time(10_205 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Ledger (r:0 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking SpanSlash (r:0 w:2)
	/// The range of component `s` is `[0, 100]`.
	fn force_unstake(s: u32, ) -> Weight {
		// Minimum execution time: 80_216 nanoseconds.
		Weight::from_ref_time(86_090_609 as u64)
			// Standard Error: 2_006
			.saturating_add(Weight::from_ref_time(1_039_308 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Staking UnappliedSlashes (r:1 w:1)
	/// The range of component `s` is `[1, 1000]`.
	fn cancel_deferred_slash(s: u32, ) -> Weight {
		// Minimum execution time: 92_034 nanoseconds.
		Weight::from_ref_time(896_585_370 as u64)
			// Standard Error: 58_231
			.saturating_add(Weight::from_ref_time(4_908_277 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: Staking Payee (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	/// The range of component `n` is `[0, 256]`.
	fn payout_stakers_dead_controller(n: u32, ) -> Weight {
		// Minimum execution time: 127_936 nanoseconds.
		Weight::from_ref_time(184_556_084 as u64)
			// Standard Error: 26_981
			.saturating_add(Weight::from_ref_time(21_786_304 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: Staking Payee (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `n` is `[0, 256]`.
	fn payout_stakers_alive_staked(n: u32, ) -> Weight {
		// Minimum execution time: 157_778 nanoseconds.
		Weight::from_ref_time(223_306_359 as u64)
			// Standard Error: 27_216
			.saturating_add(Weight::from_ref_time(30_612_663 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().reads((5 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(n as u64)))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListBags (r:2 w:2)
	/// The range of component `l` is `[1, 32]`.
	fn rebond(l: u32, ) -> Weight {
		// Minimum execution time: 92_880 nanoseconds.
		Weight::from_ref_time(94_434_663 as u64)
			// Standard Error: 1_734
			.saturating_add(Weight::from_ref_time(34_453 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:1)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking SpanSlash (r:0 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn reap_stash(s: u32, ) -> Weight {
		// Minimum execution time: 92_334 nanoseconds.
		Weight::from_ref_time(95_207_614 as u64)
			// Standard Error: 1_822
			.saturating_add(Weight::from_ref_time(1_036_787 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: VoterList CounterForListNodes (r:1 w:0)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: VoterList ListBags (r:200 w:0)
	// Storage: VoterList ListNodes (r:101 w:0)
	// Storage: Staking Nominators (r:101 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking Bonded (r:101 w:0)
	// Storage: Staking Ledger (r:101 w:0)
	// Storage: Staking CounterForValidators (r:1 w:0)
	// Storage: Staking ValidatorCount (r:1 w:0)
	// Storage: Staking MinimumValidatorCount (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:0 w:1)
	// Storage: Staking ErasValidatorPrefs (r:0 w:1)
	// Storage: Staking ErasStakers (r:0 w:1)
	// Storage: Staking ErasTotalStake (r:0 w:1)
	// Storage: Staking ErasStartSessionIndex (r:0 w:1)
	/// The range of component `v` is `[1, 10]`.
	/// The range of component `n` is `[0, 100]`.
	fn new_era(v: u32, n: u32, ) -> Weight {
		// Minimum execution time: 535_169 nanoseconds.
		Weight::from_ref_time(548_667_000 as u64)
			// Standard Error: 1_759_252
			.saturating_add(Weight::from_ref_time(58_283_319 as u64).saturating_mul(v as u64))
			// Standard Error: 175_299
			.saturating_add(Weight::from_ref_time(13_578_512 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(207 as u64))
			.saturating_add(T::DbWeight::get().reads((5 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().reads((4 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(v as u64)))
	}
	// Storage: VoterList CounterForListNodes (r:1 w:0)
	// Storage: Staking SlashingSpans (r:21 w:0)
	// Storage: VoterList ListBags (r:200 w:0)
	// Storage: VoterList ListNodes (r:1500 w:0)
	// Storage: Staking Nominators (r:1500 w:0)
	// Storage: Staking Validators (r:500 w:0)
	// Storage: Staking Bonded (r:1500 w:0)
	// Storage: Staking Ledger (r:1500 w:0)
	/// The range of component `v` is `[500, 1000]`.
	/// The range of component `n` is `[500, 1000]`.
	/// The range of component `s` is `[1, 20]`.
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight {
		// Minimum execution time: 25_323_129 nanoseconds.
		Weight::from_ref_time(25_471_672_000 as u64)
			// Standard Error: 266_391
			.saturating_add(Weight::from_ref_time(6_665_504 as u64).saturating_mul(v as u64))
			// Standard Error: 266_391
			.saturating_add(Weight::from_ref_time(6_956_606 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(202 as u64))
			.saturating_add(T::DbWeight::get().reads((5 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().reads((4 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Staking CounterForValidators (r:1 w:0)
	// Storage: Staking Validators (r:501 w:0)
	/// The range of component `v` is `[500, 1000]`.
	fn get_npos_targets(v: u32, ) -> Weight {
		// Minimum execution time: 4_905_036 nanoseconds.
		Weight::from_ref_time(78_163_554 as u64)
			// Standard Error: 23_723
			.saturating_add(Weight::from_ref_time(9_784_870 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: Staking MinCommission (r:0 w:1)
	// Storage: Staking MinValidatorBond (r:0 w:1)
	// Storage: Staking MaxValidatorsCount (r:0 w:1)
	// Storage: Staking ChillThreshold (r:0 w:1)
	// Storage: Staking MaxNominatorsCount (r:0 w:1)
	// Storage: Staking MinNominatorBond (r:0 w:1)
	fn set_staking_configs_all_set() -> Weight {
		// Minimum execution time: 10_096 nanoseconds.
		Weight::from_ref_time(10_538_000 as u64)
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking MinCommission (r:0 w:1)
	// Storage: Staking MinValidatorBond (r:0 w:1)
	// Storage: Staking MaxValidatorsCount (r:0 w:1)
	// Storage: Staking ChillThreshold (r:0 w:1)
	// Storage: Staking MaxNominatorsCount (r:0 w:1)
	// Storage: Staking MinNominatorBond (r:0 w:1)
	fn set_staking_configs_all_remove() -> Weight {
		// Minimum execution time: 9_045 nanoseconds.
		Weight::from_ref_time(9_379_000 as u64)
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking ChillThreshold (r:1 w:0)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	fn chill_other() -> Weight {
		// Minimum execution time: 81_457 nanoseconds.
		Weight::from_ref_time(82_410_000 as u64)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking MinCommission (r:1 w:0)
	// Storage: Staking Validators (r:1 w:1)
	fn force_apply_min_commission() -> Weight {
		// Minimum execution time: 19_684 nanoseconds.
		Weight::from_ref_time(20_059_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	fn bond() -> Weight {
		// Minimum execution time: 53_097 nanoseconds.
		Weight::from_ref_time(53_708_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: VoterList ListBags (r:2 w:2)
	fn bond_extra() -> Weight {
		// Minimum execution time: 92_199 nanoseconds.
		Weight::from_ref_time(93_541_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListBags (r:2 w:2)
	fn unbond() -> Weight {
		// Minimum execution time: 98_227 nanoseconds.
		Weight::from_ref_time(99_070_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(12 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		// Minimum execution time: 45_058 nanoseconds.
		Weight::from_ref_time(46_592_713 as u64)
			// Standard Error: 413
			.saturating_add(Weight::from_ref_time(63_036 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_kill(_s: u32, ) -> Weight {
		// Minimum execution time: 86_087 nanoseconds.
		Weight::from_ref_time(87_627_894 as u64)
			.saturating_add(RocksDbWeight::get().reads(13 as u64))
			.saturating_add(RocksDbWeight::get().writes(11 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MinValidatorBond (r:1 w:0)
	// Storage: Staking MinCommission (r:1 w:0)
	// Storage: Staking Validators (r:1 w:1)
	// Storage: Staking MaxValidatorsCount (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListNodes (r:1 w:1)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking CounterForValidators (r:1 w:1)
	fn validate() -> Weight {
		// Minimum execution time: 67_690 nanoseconds.
		Weight::from_ref_time(68_348_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(11 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	/// The range of component `k` is `[1, 128]`.
	fn kick(k: u32, ) -> Weight {
		// Minimum execution time: 43_512 nanoseconds.
		Weight::from_ref_time(47_300_477 as u64)
			// Standard Error: 11_609
			.saturating_add(Weight::from_ref_time(6_770_405 as u64).saturating_mul(k as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	/// The range of component `n` is `[1, 16]`.
	fn nominate(n: u32, ) -> Weight {
		// Minimum execution time: 74_296 nanoseconds.
		Weight::from_ref_time(73_201_782 as u64)
			// Standard Error: 5_007
			.saturating_add(Weight::from_ref_time(2_810_370 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(12 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	fn chill() -> Weight {
		// Minimum execution time: 66_605 nanoseconds.
		Weight::from_ref_time(67_279_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Payee (r:0 w:1)
	fn set_payee() -> Weight {
		// Minimum execution time: 18_897 nanoseconds.
		Weight::from_ref_time(19_357_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:2 w:2)
	fn set_controller() -> Weight {
		// Minimum execution time: 26_509 nanoseconds.
		Weight::from_ref_time(26_961_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Staking ValidatorCount (r:0 w:1)
	fn set_validator_count() -> Weight {
		// Minimum execution time: 5_025 nanoseconds.
		Weight::from_ref_time(5_240_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_no_eras() -> Weight {
		// Minimum execution time: 5_107 nanoseconds.
		Weight::from_ref_time(5_320_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era() -> Weight {
		// Minimum execution time: 5_094 nanoseconds.
		Weight::from_ref_time(5_377_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era_always() -> Weight {
		// Minimum execution time: 5_219 nanoseconds.
		Weight::from_ref_time(5_434_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Staking Invulnerables (r:0 w:1)
	/// The range of component `v` is `[0, 1000]`.
	fn set_invulnerables(v: u32, ) -> Weight {
		// Minimum execution time: 5_122 nanoseconds.
		Weight::from_ref_time(5_977_533 as u64)
			// Standard Error: 34
			.saturating_add(Weight::from_ref_time(10_205 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Ledger (r:0 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking SpanSlash (r:0 w:2)
	/// The range of component `s` is `[0, 100]`.
	fn force_unstake(s: u32, ) -> Weight {
		// Minimum execution time: 80_216 nanoseconds.
		Weight::from_ref_time(86_090_609 as u64)
			// Standard Error: 2_006
			.saturating_add(Weight::from_ref_time(1_039_308 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(11 as u64))
			.saturating_add(RocksDbWeight::get().writes(12 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Staking UnappliedSlashes (r:1 w:1)
	/// The range of component `s` is `[1, 1000]`.
	fn cancel_deferred_slash(s: u32, ) -> Weight {
		// Minimum execution time: 92_034 nanoseconds.
		Weight::from_ref_time(896_585_370 as u64)
			// Standard Error: 58_231
			.saturating_add(Weight::from_ref_time(4_908_277 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: Staking Payee (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	/// The range of component `n` is `[0, 256]`.
	fn payout_stakers_dead_controller(n: u32, ) -> Weight {
		// Minimum execution time: 127_936 nanoseconds.
		Weight::from_ref_time(184_556_084 as u64)
			// Standard Error: 26_981
			.saturating_add(Weight::from_ref_time(21_786_304 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(9 as u64))
			.saturating_add(RocksDbWeight::get().reads((3 as u64).saturating_mul(n as u64)))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: Staking Payee (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `n` is `[0, 256]`.
	fn payout_stakers_alive_staked(n: u32, ) -> Weight {
		// Minimum execution time: 157_778 nanoseconds.
		Weight::from_ref_time(223_306_359 as u64)
			// Standard Error: 27_216
			.saturating_add(Weight::from_ref_time(30_612_663 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(10 as u64))
			.saturating_add(RocksDbWeight::get().reads((5 as u64).saturating_mul(n as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(n as u64)))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListBags (r:2 w:2)
	/// The range of component `l` is `[1, 32]`.
	fn rebond(l: u32, ) -> Weight {
		// Minimum execution time: 92_880 nanoseconds.
		Weight::from_ref_time(94_434_663 as u64)
			// Standard Error: 1_734
			.saturating_add(Weight::from_ref_time(34_453 as u64).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(9 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:1)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking SpanSlash (r:0 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn reap_stash(s: u32, ) -> Weight {
		// Minimum execution time: 92_334 nanoseconds.
		Weight::from_ref_time(95_207_614 as u64)
			// Standard Error: 1_822
			.saturating_add(Weight::from_ref_time(1_036_787 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(12 as u64))
			.saturating_add(RocksDbWeight::get().writes(12 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: VoterList CounterForListNodes (r:1 w:0)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: VoterList ListBags (r:200 w:0)
	// Storage: VoterList ListNodes (r:101 w:0)
	// Storage: Staking Nominators (r:101 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking Bonded (r:101 w:0)
	// Storage: Staking Ledger (r:101 w:0)
	// Storage: Staking CounterForValidators (r:1 w:0)
	// Storage: Staking ValidatorCount (r:1 w:0)
	// Storage: Staking MinimumValidatorCount (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:0 w:1)
	// Storage: Staking ErasValidatorPrefs (r:0 w:1)
	// Storage: Staking ErasStakers (r:0 w:1)
	// Storage: Staking ErasTotalStake (r:0 w:1)
	// Storage: Staking ErasStartSessionIndex (r:0 w:1)
	/// The range of component `v` is `[1, 10]`.
	/// The range of component `n` is `[0, 100]`.
	fn new_era(v: u32, n: u32, ) -> Weight {
		// Minimum execution time: 535_169 nanoseconds.
		Weight::from_ref_time(548_667_000 as u64)
			// Standard Error: 1_759_252
			.saturating_add(Weight::from_ref_time(58_283_319 as u64).saturating_mul(v as u64))
			// Standard Error: 175_299
			.saturating_add(Weight::from_ref_time(13_578_512 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(207 as u64))
			.saturating_add(RocksDbWeight::get().reads((5 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().reads((4 as u64).saturating_mul(n as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(v as u64)))
	}
	// Storage: VoterList CounterForListNodes (r:1 w:0)
	// Storage: Staking SlashingSpans (r:21 w:0)
	// Storage: VoterList ListBags (r:200 w:0)
	// Storage: VoterList ListNodes (r:1500 w:0)
	// Storage: Staking Nominators (r:1500 w:0)
	// Storage: Staking Validators (r:500 w:0)
	// Storage: Staking Bonded (r:1500 w:0)
	// Storage: Staking Ledger (r:1500 w:0)
	/// The range of component `v` is `[500, 1000]`.
	/// The range of component `n` is `[500, 1000]`.
	/// The range of component `s` is `[1, 20]`.
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight {
		// Minimum execution time: 25_323_129 nanoseconds.
		Weight::from_ref_time(25_471_672_000 as u64)
			// Standard Error: 266_391
			.saturating_add(Weight::from_ref_time(6_665_504 as u64).saturating_mul(v as u64))
			// Standard Error: 266_391
			.saturating_add(Weight::from_ref_time(6_956_606 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(202 as u64))
			.saturating_add(RocksDbWeight::get().reads((5 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().reads((4 as u64).saturating_mul(n as u64)))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Staking CounterForValidators (r:1 w:0)
	// Storage: Staking Validators (r:501 w:0)
	/// The range of component `v` is `[500, 1000]`.
	fn get_npos_targets(v: u32, ) -> Weight {
		// Minimum execution time: 4_905_036 nanoseconds.
		Weight::from_ref_time(78_163_554 as u64)
			// Standard Error: 23_723
			.saturating_add(Weight::from_ref_time(9_784_870 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: Staking MinCommission (r:0 w:1)
	// Storage: Staking MinValidatorBond (r:0 w:1)
	// Storage: Staking MaxValidatorsCount (r:0 w:1)
	// Storage: Staking ChillThreshold (r:0 w:1)
	// Storage: Staking MaxNominatorsCount (r:0 w:1)
	// Storage: Staking MinNominatorBond (r:0 w:1)
	fn set_staking_configs_all_set() -> Weight {
		// Minimum execution time: 10_096 nanoseconds.
		Weight::from_ref_time(10_538_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: Staking MinCommission (r:0 w:1)
	// Storage: Staking MinValidatorBond (r:0 w:1)
	// Storage: Staking MaxValidatorsCount (r:0 w:1)
	// Storage: Staking ChillThreshold (r:0 w:1)
	// Storage: Staking MaxNominatorsCount (r:0 w:1)
	// Storage: Staking MinNominatorBond (r:0 w:1)
	fn set_staking_configs_all_remove() -> Weight {
		// Minimum execution time: 9_045 nanoseconds.
		Weight::from_ref_time(9_379_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking ChillThreshold (r:1 w:0)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	fn chill_other() -> Weight {
		// Minimum execution time: 81_457 nanoseconds.
		Weight::from_ref_time(82_410_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(11 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: Staking MinCommission (r:1 w:0)
	// Storage: Staking Validators (r:1 w:1)
	fn force_apply_min_commission() -> Weight {
		// Minimum execution time: 19_684 nanoseconds.
		Weight::from_ref_time(20_059_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}