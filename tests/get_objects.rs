mod testing;

use obex::obex::Obex;

#[test]
fn returns_a_list_of_objects() {
    let test_name = "get_objects_returns_a_list_of_objects";
    let obex_url = "https://github.com/parallaxinc/propeller.git";
    let test_directory = testing::create_test_directory(&test_name);

    let new_obex = Obex::new(obex_url, &test_directory).expect("Error initializing obex");

    testing::remove_test_directory(&test_directory);

    assert!(new_obex.community_objects.len() > 0);
}
