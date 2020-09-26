pub struct Category {
    pub name: String,
    pub path: String,
}

impl Category {
    pub fn read_categories(path: &str) -> Result<Vec<Category>, &'static str> {
        let mut categories: Vec<Category> = Vec::new();
        let root = match std::fs::read_dir(path) {
            Ok(dirs) => dirs,
            Err(_) => return Err("Error opening Obex categories directory"),
        };

        for item in root {
            let item = match item {
                Ok(i) => i,
                Err(_) => return Err("Error iterating over categories"),
            };

            if item.path().is_dir() && item.file_name() != "All" {
                let name = match item.file_name().into_string() {
                    Ok(n) => n,
                    Err(_) => return Err("Error reading category name"),
                };

                let item_path = item.path();

                let path_string = match item_path.to_str() {
                    Some(p) => p,
                    None => return Err("Error getting category path"),
                };

                categories.push(Category {
                    name: name,
                    path: path_string.to_string(),
                });
            }
        }

        Ok(categories)
    }
}
