//! # Role-based Access Control (RBAC) Pallet
//!
//! The RBAC FRAME Pallet can be used with other pallets:
//! 1. to define roles, 
//! 2. assign roles required for extrinsic calls, 
//! 3. assign accounts to roles, 
//! 4. restrict extrinsic calls to assigned accounts 
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use core::convert::TryInto;
use frame_support::{
	dispatch::{DispatchInfo, Vec},
	pallet_prelude::MaxEncodedLen,
	traits::*,
};
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{DispatchInfoOf, Dispatchable, SignedExtension},
	transaction_validity::{InvalidTransaction, TransactionValidity, TransactionValidityError},
	RuntimeDebug,
};
use sp_std::{convert::From, prelude::*};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

const PALLET_NAME_LENGTH: usize = 36;
type PalletName = [u8; PALLET_NAME_LENGTH];

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		type RbacAdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	// Set for storing all Global Admins i.e. accounts that have access to all pallets' roles
	// `StorageMap` is used as there are no native Set type 
	#[pallet::storage]
	#[pallet::getter(fn general_admins)]
	pub type GlobalAdminSet<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ()>;

	// Set for storing all `Permission`s
	// `StorageMap` is used as there are no native Set type 
	#[pallet::storage]
	#[pallet::getter(fn permissions)]
	pub type PermissionSet<T: Config> = StorageMap<_, Blake2_128Concat, (T::AccountId, Role), ()>;

	// Set for storing all `Role`s
	// `StorageMap` is used as there are no native Set type 
	#[pallet::storage]
	#[pallet::getter(fn roles)]
	pub type RoleSet<T: Config> = StorageMap<_, Blake2_128Concat, Role, ()>;
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum Permission {
	Execute = 1,
	Manage = 2,
}

impl Default for Permission {
	fn default() -> Self {
		Permission::Execute
	}
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct Role {
	pallet: [u8; 36],
	permission: Permission,
}
