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
