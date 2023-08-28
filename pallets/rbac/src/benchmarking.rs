//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_role() {
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		create_role(RawOrigin::Signed(caller), [0; 36], crate::Permission::Execute);

	}

	#[benchmark]
	fn assign_role() {
		let caller: T::AccountId = whitelisted_caller();

		let role = Role { pallet: [0; 36], permission: crate::Permission::Manage };

		RoleSet::<T>::insert(
			role.clone(),
			()
		);

		PermissionSet::<T>::insert(
			(caller.clone(), role.clone()),
			()
		);

		let account_id: T::AccountId = account("Bob", 3, 3);

		#[extrinsic_call]
		assign_role(RawOrigin::Signed(caller), account_id, role);
	}

	#[benchmark]
	fn unassign_role() {
		let caller: T::AccountId = whitelisted_caller();

		let role = Role { pallet: [0; 36], permission: crate::Permission::Manage };

		RoleSet::<T>::insert(
			role.clone(),
			()
		);

		PermissionSet::<T>::insert(
			(caller.clone(), role.clone()),
			()
		);

		let account_id: T::AccountId = account("Bob", 3, 3);

		PermissionSet::<T>::insert(
			(account_id.clone(), role.clone()),
			()
		);

		#[extrinsic_call]
		unassign_role(RawOrigin::Signed(caller), account_id, role);

	}

	#[benchmark]
	fn add_global_admin() {
		let global_admin_candidate: T::AccountId = account("Bob", 3, 3);

		#[extrinsic_call]
		add_global_admin(RawOrigin::Root, global_admin_candidate);
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
