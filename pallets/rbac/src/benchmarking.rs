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

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
