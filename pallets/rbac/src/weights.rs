
//! Autogenerated weights for `pallet_rbac`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `a-virtualbox`, CPU: `AMD Ryzen 9 5900HX with Radeon Graphics`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --wasm-execution=compiled
// --pallet=pallet_rbac
// --extrinsic=*
// --output=./pallets/rbac/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_rbac`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::pallet::WeightInfo for WeightInfo<T> {
	/// Storage: `TemplateModule::RoleSet` (r:1 w:1)
	/// Proof: `TemplateModule::RoleSet` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn create_role() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `43`
		//  Estimated: `3518`
		// Minimum execution time: 17_181_000 picoseconds.
		Weight::from_parts(18_404_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `TemplateModule::RoleSet` (r:1 w:0)
	/// Proof: `TemplateModule::RoleSet` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `TemplateModule::PermissionSet` (r:1 w:1)
	/// Proof: `TemplateModule::PermissionSet` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn assign_role() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `225`
		//  Estimated: `3550`
		// Minimum execution time: 26_262_000 picoseconds.
		Weight::from_parts(27_074_000, 0)
			.saturating_add(Weight::from_parts(0, 3550))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `TemplateModule::RoleSet` (r:1 w:0)
	/// Proof: `TemplateModule::RoleSet` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `TemplateModule::PermissionSet` (r:2 w:1)
	/// Proof: `TemplateModule::PermissionSet` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn unassign_role() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `321`
		//  Estimated: `6110`
		// Minimum execution time: 31_014_000 picoseconds.
		Weight::from_parts(37_920_000, 0)
			.saturating_add(Weight::from_parts(0, 6110))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `TemplateModule::GlobalAdminSet` (r:0 w:1)
	/// Proof: `TemplateModule::GlobalAdminSet` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn add_global_admin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_800_000 picoseconds.
		Weight::from_parts(14_674_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
		/// Storage: `TemplateModule::GlobalAdminSet` (r:0 w:1)
	/// Proof: `TemplateModule::GlobalAdminSet` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn remove_global_admin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_800_000 picoseconds.
		Weight::from_parts(14_674_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
