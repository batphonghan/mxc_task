use crate::{mock::*, Error};
use frame_support::{assert_err, assert_noop, assert_ok};

#[test]
fn test_create_club() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(Origin::signed(1), 1));
		// Read pallet storage and assert an expected result.
		assert_noop!(
			TemplateModule::create_club(Origin::signed(1), 1),
			Error::<Test>::ClubAlreadyCreated
		);
	});
}
