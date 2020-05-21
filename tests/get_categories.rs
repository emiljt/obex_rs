mod testing;

use obex::obex::Obex;

#[test]
fn returns_a_list_of_categories() {
    let test_name = "get_categories_returns_a_list_of_categories";
    let obex_url = "https://github.com/parallaxinc/propeller.git";
    let test_directory = testing::create_test_directory(&test_name);

    let new_obex = Obex::initialize(obex_url, &test_directory).expect("Error initializing obex");

    testing::remove_test_directory(&test_directory);

    assert!(new_obex.community_categories.len() > 0);
}
