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

	/// Weight functions needed for pallet_rbac.
	pub trait WeightInfo {
		fn create_role() -> Weight;
		fn assign_role() -> Weight;
		fn unassign_role() -> Weight;
		fn add_global_admin() -> Weight;
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

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub general_admins: Vec<T::AccountId>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { general_admins: Vec::new() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			for admin in &self.general_admins {
				<GlobalAdminSet<T>>::insert(admin, ());
			}
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RoleCreated { role: Role },
		RoleUnassigned { pallet_name: PalletName, account_id: T::AccountId },
		RoleAssigned { pallet_name: PalletName, account_id: T::AccountId },
		GlobalAdminAdded { account_id: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		AccessDenied,
		RoleAlreadyExists,
		RoleDoesNotExist,
		RoleWasNotAssigned,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates a `Role`.
		/// 
		/// Returns `RoleAlreadyExists` error in case `Role` that one is trying to create already exists.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_role())]
		pub fn create_role(
			origin: OriginFor<T>,
			pallet_name: PalletName,
			permission: Permission,
		) -> DispatchResult {
			ensure_signed(origin)?;

			let role = Role { pallet: pallet_name, permission };

			if <RoleSet<T>>::contains_key(&role)  {
				return Err(Error::<T>::RoleAlreadyExists.into())
			}
			
			RoleSet::<T>::insert(role.clone(), ());
			Self::deposit_event(Event::RoleCreated { role });

			Ok(())
		}

		/// Assignes a `Role` to an account.
		/// 
		/// Returns `RoleDoesNotExist` error in case `Role` did not exist in the first place.
		/// Returns `AccessDenied` error in case the account trying to assign isn't an Admin or Global Admin.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::assign_role())]
		pub fn assign_role(
			origin: OriginFor<T>,
			account_id: T::AccountId,
			role: Role,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			if !<RoleSet<T>>::contains_key(&role) {
				return Err(Error::<T>::RoleDoesNotExist.into())
			}

			if Self::verify_manage_access(who.clone(), role.pallet.clone()) || <GlobalAdminSet<T>>::contains_key(&who) {
				<PermissionSet<T>>::insert((account_id.clone(), role.clone()), ());

				Self::deposit_event(Event::RoleAssigned { pallet_name: role.pallet, account_id });
			} else {
				return Err(Error::<T>::AccessDenied.into())
			}
			Ok(())
		}
	}
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
