use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_create_club() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::create_club(Origin::signed(1), 1));

		assert_noop!(
			TemplateModule::create_club(Origin::signed(1), 1),
			Error::<Test>::ClubAlreadyCreated
		);
	});
}

#[test]
fn test_add_member() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::create_club(Origin::signed(1), 1));

		assert_ok!(TemplateModule::add_member(Origin::signed(1), 1, 1));

		assert_noop!(
			TemplateModule::add_member(Origin::signed(1), 1, 1),
			Error::<Test>::MemberAlreadyAdded
		);

		assert_noop!(
			TemplateModule::add_member(Origin::signed(0), 2, 1),
			Error::<Test>::NotClubOwner
		);
	});
}

#[test]
fn test_remove_member() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::create_club(Origin::signed(1), 1));

		assert_ok!(TemplateModule::add_member(Origin::signed(1), 1, 1));

		assert_noop!(
			TemplateModule::remove_member(Origin::signed(0), 1, 1),
			Error::<Test>::NotClubOwner
		);

		assert_ok!(TemplateModule::remove_member(Origin::signed(1), 1, 1));

		assert_noop!(
			TemplateModule::remove_member(Origin::signed(1), 1, 1),
			Error::<Test>::NoSuchMember
		);
	});
}
