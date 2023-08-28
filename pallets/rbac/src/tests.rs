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
