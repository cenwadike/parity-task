use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_record() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::create_new_record(Origin::signed(A), "foo", "bar", 18));
		// read first patient record
		assert_eq!(TemplateModule::Biodata::get(1).name = "foo");
		assert_eq!(TemplateModule::Biodata::get(1).sex = "bar");
		assert_eq!(TemplateModule::Biodata::get(1).age = 18);
	});
}

#[test]
fn grant_access_pass() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::create_new_record(Origin::signed(A), "foo", "bar", 18));
		assert_ok!(TemplateModule::grant_access(Origin::signed(A), B, 1));
		assert_eq!(TemplateModule::Biodata::get(1).access.contains(B));
	});
}

#[test]
fn grant_access_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::create_new_record(Origin::signed(A), "foo", "bar", 18));
		assert_ok!(TemplateModule::grant_access(Origin::signed(B), C, 1));
		assert_eq!(!TemplateModule::Biodata::get(1).access.contains(B));
	});
}

#[test]
fn revoke_access_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::create_new_record(Origin::signed(A), "foo", "bar", 18));
		assert_ok!(TemplateModule::grant_access(Origin::signed(A), B, 1)); 
		assert_ok!(!TemplateModule::revoke_access(Origin::signed(B), B, 1)); 
		assert_eq!(TemplateModule::Biodata::get(1).access.contains(B)); // access persist
	});
}

#[test]
fn revoke_access_pass() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::create_new_record(Origin::signed(A), "foo", "bar", 18));
		assert_ok!(TemplateModule::grant_access(Origin::signed(A), B, 1));
		assert_ok!(TemplateModule::revoke_access(Origin::signed(A), B, 1));
		assert_eq!(!TemplateModule::Biodata::get(1).access.contains(B)); // access revoked
	});
}
