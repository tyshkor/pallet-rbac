use crate::{mock::*, Event, Role};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_role() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Assert adding a new Global Admin succeeds
		assert_ok!(TemplateModule::add_global_admin(RuntimeOrigin::root(), 1));
		let permission = crate::Permission::Execute { call_name: [1; 36] };
		// Assert creating the role succeeds
		assert_ok!(TemplateModule::create_role(
			RuntimeOrigin::signed(1),
			[0; 36],
			permission.clone(),
		));
		// Assert that the correct event was deposited
		System::assert_last_event(
			Event::RoleCreated { role: Role { pallet: [0; 36], permission } }.into(),
		);
	});
}

#[test]
fn create_the_same_role_twice_should_fail() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Assert adding a new Global Admin succeeds
		assert_ok!(TemplateModule::add_global_admin(RuntimeOrigin::root(), 1));
		let pallet_name = [0; 36];
		let permission = crate::Permission::Execute { call_name: [1; 36] };
		let role = Role { pallet: pallet_name, permission: permission.clone() };
		// Assert creating the role first time succeeds
		assert_ok!(TemplateModule::create_role(
			RuntimeOrigin::signed(1),
			pallet_name,
			permission.clone(),
		));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::RoleCreated { role }.into());
		// Assert creating the same role second time fails
		assert_noop!(
			TemplateModule::create_role(RuntimeOrigin::signed(1), pallet_name, permission,),
			crate::pallet::Error::RoleAlreadyExists::<Test>,
		);
	});
}

#[test]
fn assign_role() {
	new_test_ext_with_general_admin().execute_with(|| {
		System::set_block_number(1);
		// Assert adding a new Global Admin succeeds
		assert_ok!(TemplateModule::add_global_admin(RuntimeOrigin::root(), 1));
		let permission = crate::Permission::Execute { call_name: [1; 36] };
		// Assert creating the role succeeds
		assert_ok!(TemplateModule::create_role(
			RuntimeOrigin::signed(1),
			[0; 36],
			permission.clone()
		));
		let pallet_name = [0; 36];
		// Assert assigning the role succeeds
		assert_ok!(TemplateModule::assign_role(
			RuntimeOrigin::signed(1),
			42,
			crate::Role { pallet: pallet_name, permission }
		));
		System::assert_last_event(Event::RoleAssigned { pallet_name, account_id: 42 }.into());
	});
}

#[test]
fn assign_non_existent_role_should_fail() {
	new_test_ext_with_general_admin().execute_with(|| {
		System::set_block_number(1);
		// Assert adding a new Global Admin succeeds
		assert_ok!(TemplateModule::add_global_admin(RuntimeOrigin::root(), 1));
		let pallet = [0; 36];
		let permission = crate::Permission::Execute { call_name: [1; 36] };
		// Assert assigning the non-existent role fails
		assert_noop!(
			TemplateModule::assign_role(
				RuntimeOrigin::signed(1),
				42,
				crate::Role { pallet, permission }
			),
			crate::pallet::Error::RoleDoesNotExist::<Test>,
		);
	});
}

#[test]
fn unassign_role() {
	new_test_ext_with_general_admin().execute_with(|| {
		System::set_block_number(1);
		// Assert adding a new Global Admin succeeds
		assert_ok!(TemplateModule::add_global_admin(RuntimeOrigin::root(), 1));
		let pallet_name = [0; 36];
		let account_id = 42;
		// Assert creating the role succeeds
		assert_ok!(TemplateModule::create_role(
			RuntimeOrigin::signed(1),
			pallet_name.clone(),
			crate::Permission::Manage
		));
		// Assert assigning the role succeeds
		assert_ok!(TemplateModule::assign_role(
			RuntimeOrigin::signed(1),
			account_id,
			crate::Role { pallet: pallet_name, permission: crate::Permission::Manage }
		));
		// Assert unassigning the role succeeds
		assert_ok!(TemplateModule::unassign_role(
			RuntimeOrigin::signed(1),
			account_id,
			crate::Role { pallet: pallet_name, permission: crate::Permission::Manage }
		));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::RoleUnassigned { pallet_name, account_id }.into());
	});
}

#[test]
fn unassign_role_that_was_not_assigned_should_fail() {
	new_test_ext_with_general_admin().execute_with(|| {
		System::set_block_number(1);
		// Assert adding a new Global Admin succeeds
		assert_ok!(TemplateModule::add_global_admin(RuntimeOrigin::root(), 1));
		let pallet_name = [0; 36];
		// Assert creating the role succeeds
		assert_ok!(TemplateModule::create_role(
			RuntimeOrigin::signed(1),
			pallet_name,
			crate::Permission::Manage
		));
		// Assert unassigning the non-assigned role fails
		assert_noop!(
			TemplateModule::unassign_role(
				RuntimeOrigin::signed(1),
				42,
				crate::Role { pallet: pallet_name, permission: crate::Permission::Manage }
			),
			crate::pallet::Error::RoleWasNotAssigned::<Test>,
		);
	});
}

#[test]
fn add_global_admin() {
	new_test_ext_with_general_admin().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let account_id = 2;
		// Assert adding a new Global Admin succeeds
		assert_ok!(TemplateModule::add_global_admin(RuntimeOrigin::root(), account_id.clone()));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::GlobalAdminAdded { account_id }.into());
	});
}

#[test]
fn remove_global_admin() {
	new_test_ext_with_general_admin().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let account_id = 2;
		// Assert adding a new Global Admin succeeds
		assert_ok!(TemplateModule::add_global_admin(RuntimeOrigin::root(), account_id.clone()));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::GlobalAdminAdded { account_id }.into());
		// Assert removing a new Global Admin succeeds
		assert_ok!(TemplateModule::remove_global_admin(RuntimeOrigin::root(), account_id.clone()));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::GlobalAdminRemoved { account_id }.into());
	});
}

#[test]
fn removing_non_global_admin_should_fail() {
	new_test_ext_with_general_admin().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let account_id = 2;
		// Assert removing a non-Global Admin fails
		assert_noop!(TemplateModule::remove_global_admin(RuntimeOrigin::root(), account_id.clone()),
		crate::pallet::Error::AccountWasNotGlobalAdmin::<Test>,);
	});
}

#[test]
fn add_general_admin_by_nonroot_should_fail() {
	new_test_ext().execute_with(|| {
		// Assert adding a new Global Admin by root account succeeds
		assert_ok!(TemplateModule::add_global_admin(RuntimeOrigin::root(), 2));
		// Assert adding a new Global Admin by non-root account fails
		assert_noop!(
			TemplateModule::add_global_admin(RuntimeOrigin::signed(2), 3),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}
