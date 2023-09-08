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

const CALL_NAME_LENGTH: usize = 36;
type CallName = [u8; CALL_NAME_LENGTH];

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
	#[pallet::getter(fn global_admins)]
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
		pub global_admins: Vec<T::AccountId>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { global_admins: Vec::new() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			for admin in &self.global_admins {
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
		GlobalAdminRemoved { account_id: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		AccessDenied,
		RoleAlreadyExists,
		RoleDoesNotExist,
		RoleWasNotAssigned,
		AccountWasNotGlobalAdmin,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates a `Role`.
		///
		/// Returns `RoleAlreadyExists` error in case `Role` that one is trying to create already
		/// exists.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_role())]
		pub fn create_role(
			origin: OriginFor<T>,
			pallet_name: PalletName,
			permission: Permission,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			if Self::verify_manage_access(who.clone(), pallet_name.clone()) ||
				<GlobalAdminSet<T>>::contains_key(&who)
			{
				let role = Role { pallet: pallet_name, permission };

				if <RoleSet<T>>::contains_key(&role) {
					return Err(Error::<T>::RoleAlreadyExists.into())
				}

				RoleSet::<T>::insert(role.clone(), ());
				Self::deposit_event(Event::RoleCreated { role });
			} else {
				return Err(Error::<T>::AccessDenied.into())
			}

			Ok(())
		}

		/// Assignes a `Role` to an account.
		///
		/// Returns `RoleDoesNotExist` error in case `Role` did not exist in the first place.
		/// Returns `AccessDenied` error in case the account trying to assign isn't an Admin or
		/// Global Admin.
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

			if Self::verify_manage_access(who.clone(), role.pallet.clone()) ||
				<GlobalAdminSet<T>>::contains_key(&who)
			{
				<PermissionSet<T>>::insert((account_id.clone(), role.clone()), ());

				Self::deposit_event(Event::RoleAssigned { pallet_name: role.pallet, account_id });
			} else {
				return Err(Error::<T>::AccessDenied.into())
			}
			Ok(())
		}

		/// Unassignes a `Role` from an account.
		///
		/// Returns `RoleWasNotAssigned` error in case `Role` was not assigned in the first place.
		/// Returns `AccessDenied` error in case the account trying to unassign isn't an Admin or
		/// Global Admin.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::unassign_role())]
		pub fn unassign_role(
			origin: OriginFor<T>,
			account_id: T::AccountId,
			role: Role,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			if Self::verify_manage_access(who.clone(), role.pallet.clone()) ||
				<GlobalAdminSet<T>>::contains_key(&who)
			{
				if !<PermissionSet<T>>::contains_key(&(account_id.clone(), role.clone())) {
					return Err(Error::<T>::RoleWasNotAssigned.into())
				}

				<PermissionSet<T>>::remove((account_id.clone(), role.clone()));

				Self::deposit_event(Event::RoleUnassigned { pallet_name: role.pallet, account_id });
			} else {
				return Err(Error::<T>::AccessDenied.into())
			}
			Ok(())
		}

		/// Add a new Global Admin.
		/// Global Admin has access to execute and manage all pallets.
		///
		/// Only _root_ can add a Global Admin.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::add_global_admin())]
		pub fn add_global_admin(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
			// Ensures that only root can call this ectrinsic
			T::RbacAdminOrigin::ensure_origin(origin)?;
			<GlobalAdminSet<T>>::insert(&account_id, ());
			Self::deposit_event(Event::GlobalAdminAdded { account_id });
			Ok(())
		}

		/// Remova a new Global Admin.
		/// Global Admin has access to execute and manage all pallets.
		///
		/// Only _root_ can remove a Global Admin.
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::add_global_admin())]
		pub fn remove_global_admin(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
			// Ensures that only root can call this ectrinsic
			T::RbacAdminOrigin::ensure_origin(origin)?;
			if !<GlobalAdminSet<T>>::contains_key(&account_id.clone()) {
				return Err(Error::<T>::AccountWasNotGlobalAdmin.into())
			}
			<GlobalAdminSet<T>>::remove(&account_id);
			Self::deposit_event(Event::GlobalAdminRemoved { account_id });
			Ok(())
		}
	}
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum Permission {
	Execute { call_name: CallName },
	Manage,
}

impl Default for Permission {
	fn default() -> Self {
		Permission::Execute { call_name: [0; CALL_NAME_LENGTH] }
	}
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct Role {
	pallet: [u8; 36],
	permission: Permission,
}

impl<T: Config> Pallet<T> {
	pub fn verify_execute_access(
		account_id: T::AccountId,
		pallet: PalletName,
		call_name: CallName,
	) -> bool {
		let role = Role { pallet, permission: Permission::Execute { call_name } };
		<RoleSet<T>>::contains_key(&role) && <PermissionSet<T>>::contains_key((account_id, role))
	}

	fn verify_manage_access(account_id: T::AccountId, pallet: PalletName) -> bool {
		let role = Role { pallet, permission: Permission::Manage };
		<RoleSet<T>>::contains_key(&role) && <PermissionSet<T>>::contains_key((account_id, role))
	}
}

/// The `Authorization` struct.
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Authorization<T: Config + Send + Sync>(sp_std::marker::PhantomData<T>);

impl<T: Config + Send + Sync> sp_std::fmt::Debug for Authorization<T> {
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "Authorization")
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}

impl<T: Config + Send + Sync> Authorization<T> {
	pub fn new() -> Self {
		Self(sp_std::marker::PhantomData)
	}
}

/// `SignedExtension` has to be implemented for `Authorization` to be able to
/// filter out extrinsics sent by the not authorized accounts
///  Validation is happenning at transaction queue level,
///  and the extrinsics are filtered out before they hit the pallet logic.
impl<T: Config + Send + Sync> SignedExtension for Authorization<T>
where
	T::RuntimeCall: Dispatchable<Info = DispatchInfo> + GetCallMetadata,
{
	type AccountId = T::AccountId;
	type Call = T::RuntimeCall;
	type AdditionalSigned = ();
	type Pre = ();
	const IDENTIFIER: &'static str = "Authorization";

	fn pre_dispatch(
		self,
		_who: &Self::AccountId,
		_call: &Self::Call,
		_info: &<Self::Call as Dispatchable>::Info,
		_len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		Ok(())
	}

	fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
		Ok(())
	}

	fn validate(
		&self,
		who: &Self::AccountId,
		call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		_len: usize,
	) -> TransactionValidity {
		if <GlobalAdminSet<T>>::contains_key(who.clone()) {
			return Ok(Default::default())
		}

		let md = call.get_call_metadata();

		let call_name: CallName = validate_name::<CALL_NAME_LENGTH>(md.function_name)?;

		let pallet_name: PalletName = validate_name::<PALLET_NAME_LENGTH>(md.pallet_name)?;

		if <Pallet<T>>::verify_execute_access(who.clone(), pallet_name, call_name) {
			Ok(Default::default())
		} else {
			Err(InvalidTransaction::Call.into())
		}
	}
}

fn validate_name<const NAME_LENGTH: usize>(
	name: &str,
) -> Result<[u8; NAME_LENGTH], TransactionValidityError> {
	let name_bytes = name.as_bytes();

	if name_bytes.len() > NAME_LENGTH {
		return Err(InvalidTransaction::Call.into())
	}

	let mut name = [0; NAME_LENGTH];

	for (i, &byte) in name_bytes.iter().enumerate() {
		name[i] = byte;
	}

	Ok(name)
}
