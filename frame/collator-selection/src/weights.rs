// This file is part of Astar.

// Copyright (C) 2019-2023 Stake Technologies Pte.Ltd.
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

// This file is part of Substrate.
//! Autogenerated weights for `pallet_collator_selection`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `shiden-collator-02-ovh`, CPU: `Intel(R) Xeon(R) E-2136 CPU @ 3.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("astar-dev"), DB CACHE: 1024

// Executed Command:
// ./astar-collator
// benchmark
// pallet
// --chain
// astar-dev
// --execution
// wasm
// --wasm-execution
// compiled
// --heap-pages
// 4096
// --pallet
// pallet_collator_selection
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallet_collator_selection_2.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

// The weight info trait for `pallet_collator_selection`.
pub trait WeightInfo {
    fn set_invulnerables(_b: u32) -> Weight;
    fn set_desired_candidates() -> Weight;
    fn set_candidacy_bond() -> Weight;
    fn register_as_candidate(_c: u32) -> Weight;
    fn leave_intent(_c: u32) -> Weight;
    fn note_author() -> Weight;
    fn new_session(_c: u32, _r: u32) -> Weight;
}

/// Weight functions for `pallet_collator_selection`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: Session NextKeys (r:1 w:0)
    // Storage: CollatorSelection Invulnerables (r:0 w:1)
    /// The range of component `b` is `[1, 48]`.
    fn set_invulnerables(b: u32) -> Weight {
        // Minimum execution time: 19_788 nanoseconds.
        Weight::from_ref_time(22_784_292 as u64)
            // Standard Error: 14_928
            .saturating_add(Weight::from_ref_time(2_318_672 as u64).saturating_mul(b as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(b as u64)))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection DesiredCandidates (r:0 w:1)
    fn set_desired_candidates() -> Weight {
        // Minimum execution time: 12_868 nanoseconds.
        Weight::from_ref_time(13_185_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection CandidacyBond (r:0 w:1)
    fn set_candidacy_bond() -> Weight {
        // Minimum execution time: 13_202 nanoseconds.
        Weight::from_ref_time(13_612_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:1)
    // Storage: CollatorSelection DesiredCandidates (r:1 w:0)
    // Storage: CollatorSelection Invulnerables (r:1 w:0)
    // Storage: Session NextKeys (r:1 w:0)
    // Storage: CollatorSelection CandidacyBond (r:1 w:0)
    // Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
    /// The range of component `c` is `[1, 148]`.
    fn register_as_candidate(c: u32) -> Weight {
        // Minimum execution time: 46_239 nanoseconds.
        Weight::from_ref_time(49_911_731 as u64)
            // Standard Error: 1_847
            .saturating_add(Weight::from_ref_time(61_723 as u64).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:1)
    // Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
    /// The range of component `c` is `[6, 148]`.
    fn leave_intent(c: u32) -> Weight {
        // Minimum execution time: 32_634 nanoseconds.
        Weight::from_ref_time(35_782_966 as u64)
            // Standard Error: 1_686
            .saturating_add(Weight::from_ref_time(59_601 as u64).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: System Account (r:2 w:2)
    // Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
    fn note_author() -> Weight {
        // Minimum execution time: 30_712 nanoseconds.
        Weight::from_ref_time(31_324_000 as u64)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:1)
    // Storage: CollatorSelection LastAuthoredBlock (r:148 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: CollatorSelection SlashDestination (r:1 w:0)
    // Storage: CollatorSelection Invulnerables (r:1 w:0)
    /// The range of component `r` is `[1, 148]`.
    /// The range of component `c` is `[1, 148]`.
    fn new_session(_r: u32, c: u32) -> Weight {
        // Minimum execution time: 20_614 nanoseconds.
        Weight::from_ref_time(21_154_000 as u64)
            // Standard Error: 388_667
            .saturating_add(Weight::from_ref_time(13_110_633 as u64).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: Session NextKeys (r:1 w:0)
    // Storage: CollatorSelection Invulnerables (r:0 w:1)
    /// The range of component `b` is `[1, 48]`.
    fn set_invulnerables(b: u32) -> Weight {
        // Minimum execution time: 19_788 nanoseconds.
        Weight::from_ref_time(22_784_292 as u64)
            // Standard Error: 14_928
            .saturating_add(Weight::from_ref_time(2_318_672 as u64).saturating_mul(b as u64))
            .saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(b as u64)))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection DesiredCandidates (r:0 w:1)
    fn set_desired_candidates() -> Weight {
        // Minimum execution time: 12_868 nanoseconds.
        Weight::from_ref_time(13_185_000 as u64)
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection CandidacyBond (r:0 w:1)
    fn set_candidacy_bond() -> Weight {
        // Minimum execution time: 13_202 nanoseconds.
        Weight::from_ref_time(13_612_000 as u64)
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:1)
    // Storage: CollatorSelection DesiredCandidates (r:1 w:0)
    // Storage: CollatorSelection Invulnerables (r:1 w:0)
    // Storage: Session NextKeys (r:1 w:0)
    // Storage: CollatorSelection CandidacyBond (r:1 w:0)
    // Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
    /// The range of component `c` is `[1, 148]`.
    fn register_as_candidate(c: u32) -> Weight {
        // Minimum execution time: 46_239 nanoseconds.
        Weight::from_ref_time(49_911_731 as u64)
            // Standard Error: 1_847
            .saturating_add(Weight::from_ref_time(61_723 as u64).saturating_mul(c as u64))
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:1)
    // Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
    /// The range of component `c` is `[6, 148]`.
    fn leave_intent(c: u32) -> Weight {
        // Minimum execution time: 32_634 nanoseconds.
        Weight::from_ref_time(35_782_966 as u64)
            // Standard Error: 1_686
            .saturating_add(Weight::from_ref_time(59_601 as u64).saturating_mul(c as u64))
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    // Storage: System Account (r:2 w:2)
    // Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
    fn note_author() -> Weight {
        // Minimum execution time: 30_712 nanoseconds.
        Weight::from_ref_time(31_324_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:1)
    // Storage: CollatorSelection LastAuthoredBlock (r:148 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: CollatorSelection SlashDestination (r:1 w:0)
    // Storage: CollatorSelection Invulnerables (r:1 w:0)
    /// The range of component `r` is `[1, 148]`.
    /// The range of component `c` is `[1, 148]`.
    fn new_session(_r: u32, c: u32) -> Weight {
        // Minimum execution time: 20_614 nanoseconds.
        Weight::from_ref_time(21_154_000 as u64)
            // Standard Error: 388_667
            .saturating_add(Weight::from_ref_time(13_110_633 as u64).saturating_mul(c as u64))
            .saturating_add(RocksDbWeight::get().reads(3 as u64))
            .saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
            .saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
    }
}
