use std::fs;

const ROOT_DIRECTORY: &str = "/tmp";

pub fn create_test_directory(name: &str) -> String {
    let test_directory = format!("{}/{}", ROOT_DIRECTORY, name);

    fs::read_dir(ROOT_DIRECTORY).expect("Error opening temp for tests");

    match fs::read_dir(&test_directory) {
        Ok(_) => {
            remove_test_directory(&test_directory);
        }
        Err(_) => (),
    }

    fs::create_dir(&test_directory).expect("Error creating test directory");

    test_directory
}

pub fn remove_test_directory(directory: &str) {
    fs::remove_dir_all(&directory).expect("Error removing test directory");
}
