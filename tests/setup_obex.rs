mod testing;

use obex::obex::Obex;

#[test]
fn initializes_when_given_good_params() {
    let test_name = "setup_obex_initializes_when_given_good_params";
    let obex_url = "https://github.com/parallaxinc/propeller.git";
    let test_directory = testing::create_test_directory(&test_name);

    let new_obex = Obex::initialize(obex_url, &test_directory);

    testing::remove_test_directory(&test_directory);

    assert!(new_obex.is_ok());
}

#[test]
fn initializes_when_given_nonexistant_directory() {
    let test_name = "setup_obex_initializes_when_given_nonexistant_directory";
    let obex_url = "https://github.com/parallaxinc/propeller.git";
    let nonexistant_directory = testing::create_test_directory(test_name);

    testing::remove_test_directory(&nonexistant_directory);

    let new_obex = Obex::initialize(obex_url, &nonexistant_directory);

    assert!(new_obex.is_ok());
}

#[test]
fn fails_when_given_wrong_url() {
    let test_name = "setup_obex_fails_when_given_wrong_url";
    let obex_rs_url = "https://github.com/emiljt/obex_rs.git";
    let test_directory = testing::create_test_directory(&test_name);

    let new_obex = Obex::initialize(obex_rs_url, &test_directory);

    testing::remove_test_directory(&test_directory);

    assert!(new_obex.is_err());
}

#[test]
fn fails_when_given_bad_url() {
    let test_name = "setup_obex_fails_when_given_bad_url";
    let invalid_url = "https://github/emiljt/obex_rs.git";
    let test_directory = testing::create_test_directory(&test_name);

    let new_obex = Obex::initialize(invalid_url, &test_directory);

    testing::remove_test_directory(&test_directory);

    assert!(new_obex.is_err());
}

#[test]
fn fails_to_initialize_in_non_empty_directory() {
    let test_name = "setup_obex_fails_to_initialize_in_non_empty_directory";
    let obex_url = "https://github.com/parallaxinc/propeller.git";
    let test_directory = testing::create_test_directory(&test_name);

    std::fs::write(&format!("{}/foo.txt", test_directory), "testing").expect("Error creating file");
    let new_obex = Obex::initialize(obex_url, &test_directory);

    testing::remove_test_directory(&test_directory);

    assert!(new_obex.is_err());
}
