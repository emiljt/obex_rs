pub struct Object {
    pub name: String,
    pub path: String,
}

impl Object {
    pub fn read_objects(path: &str) -> Result<Vec<Object>, &'static str> {
        let mut objects: Vec<Object> = Vec::new();
        let root = match std::fs::read_dir(path) {
            Ok(dirs) => dirs,
            Err(_) => return Err("Error opening Obex objects directory"),
        };

        for item in root {
            let item = match item {
                Ok(i) => i,
                Err(_) => return Err("Error iterating over objects"),
            };

            if item.path().is_dir() {
                let name = match item.file_name().into_string() {
                    Ok(n) => n,
                    Err(_) => return Err("Error reading object name"),
                };

                let item_path = item.path();

                let path_string = match item_path.to_str() {
                    Some(p) => p,
                    None => return Err("Error getting object path"),
                };

                objects.push(Object {
                    name: name,
                    path: path_string.to_string(),
                });
            }
        }

        Ok(objects)
    }
}
