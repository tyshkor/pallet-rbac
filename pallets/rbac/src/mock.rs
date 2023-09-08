use crate as pallet_rbac;
use frame_support::{
	traits::{ConstU16, ConstU64},
	weights::Weight,
};
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;
type TestWeightInfo = ();

impl crate::pallet::WeightInfo for TestWeightInfo {
	fn create_role() -> Weight {
		todo!()
	}

	fn assign_role() -> Weight {
		todo!()
	}

	fn unassign_role() -> Weight {
		todo!()
	}

	fn add_global_admin() -> Weight {
		todo!()
	}
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		TemplateModule: pallet_rbac,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_rbac::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type RbacAdminOrigin = EnsureRoot<Self::AccountId>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext_with_general_admin() -> sp_io::TestExternalities {
	crate::pallet::GenesisConfig::<Test> { global_admins: vec![1] }
		.build_storage()
		.unwrap()
		.into()
}
