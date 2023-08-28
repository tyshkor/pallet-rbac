use crate::{mock::*, Event, Role};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_role() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Assert creating the role succeeds
		assert_ok!(TemplateModule::create_role(RuntimeOrigin::signed(1), [0; 36], crate::Permission::Execute));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::RoleCreated { role: Role { pallet: [0; 36],permission: crate::Permission::Execute } }.into());
	});
}

#[test]
fn create_the_same_role_twice_should_fail() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let pallet_name = [0; 36];
		let role = Role { pallet: pallet_name,permission: crate::Permission::Execute };
		// Assert creating the role first time succeeds
		assert_ok!(TemplateModule::create_role(RuntimeOrigin::signed(1), pallet_name, crate::Permission::Execute));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::RoleCreated { role  }.into());
		// Assert creating the same role second time fails
		assert_noop!(
			TemplateModule::create_role(RuntimeOrigin::signed(1), pallet_name, crate::Permission::Execute),
			crate::pallet::Error::RoleAlreadyExists::<Test>,
		);
	});
}
