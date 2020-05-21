mod testing;

use obex::obex::Obex;

#[test]
fn succeeds_when_given_good_params() {
    let test_name = "open_obex_succeeds_when_given_good_params";
    let obex_url = "https://github.com/parallaxinc/propeller.git";
    let test_directory = testing::create_test_directory(&test_name);
    let new_obex = Obex::initialize(obex_url, &test_directory);
    let existing_obex = Obex::open(&test_directory);

    testing::remove_test_directory(&test_directory);

    assert!(existing_obex.is_ok());
}

#[test]
fn fails_when_given_nonexistant_path() {
    let test_name = "open_obex_fails_when_given_nonexistant_path";
    let obex_url = "https://github.com/parallaxinc/propeller.git";
    let nonexistant_directory = "/tmp/open_obex_nonexistant_directory";
    let test_directory = testing::create_test_directory(&test_name);
    let new_obex = Obex::initialize(obex_url, &test_directory);
    let existing_obex = Obex::open(&nonexistant_directory);

    testing::remove_test_directory(&test_directory);

    assert!(existing_obex.is_err());
}

#[test]
fn fails_when_given_wrong_repository() {
    let test_name = "open_obex_fails_when_given_wrong_repository";
    let obex_url = "https://github.com/emiljt/obex_rs.git";
    let test_directory = testing::create_test_directory(&test_name);
    let new_obex = Obex::initialize(obex_url, &test_directory);
    let existing_obex = Obex::open(&test_directory);

    testing::remove_test_directory(&test_directory);

    assert!(existing_obex.is_err());
}
