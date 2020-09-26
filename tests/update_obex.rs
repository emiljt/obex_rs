mod testing;

use obex::obex::Obex;

#[test]
fn succeeds() {
    let test_name = "update_obex_succeeds";
    let obex_url = "https://github.com/parallaxinc/propeller.git";
    let test_directory = testing::create_test_directory(test_name);

    let new_obex = Obex::new(obex_url, &test_directory).expect("Error initializing obex");

    let update_result = new_obex.update();

    testing::remove_test_directory(&test_directory);

    assert!(update_result.is_ok());
}
