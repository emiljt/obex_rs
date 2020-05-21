use super::repository::Repository;
use super::category::Category;
use super::object::Object;

const OBEX_TRACKING_BRANCH: &str = "master";
const OBEX_ROOT_PATH: &str = "libraries";

pub struct Obex {
    repository: Repository,
    pub official_categories: Vec<Category>,
    pub community_categories: Vec<Category>,
    pub official_objects: Vec<Object>,
    pub community_objects: Vec<Object>,
}

impl Obex {
    pub fn initialize(url: &str, path: &str) -> Result<Obex, &'static str> {
        // Clone Obex git repository
        let new_repo = match Repository::clone(url, path) {
            Ok(repo) => repo,
            Err(_) => return Err("Error cloning Obex repository"),
        };

        let mut new_obex = Obex {
            repository: new_repo,
            official_categories: Vec::new(),
            community_categories: Vec::new(),
            official_objects: Vec::new(),
            community_objects: Vec::new(),
        };

        match new_obex.read_repository() {
            Ok(_) => (),
            Err(_) => return Err("Error reading repository"),
        };

        Ok(new_obex)
    }

    pub fn open(path: &str) -> Result<Obex, &'static str> {
        // Open Obex git repository
        let new_repo = match Repository::open(path) {
            Ok(repo) => repo,
            Err(_) => return Err("Error opening Obex repository"),
        };

        let mut new_obex = Obex {
            repository: new_repo,
            official_categories: Vec::new(),
            community_categories: Vec::new(),
            official_objects: Vec::new(),
            community_objects: Vec::new(),
        };

        match new_obex.read_repository() {
            Ok(_) => (),
            Err(_) => return Err("Error reading repository"),
        };

        Ok(new_obex)
    }

    pub fn update(&self) -> Result<(), &'static str> {
        self.repository.pull(OBEX_TRACKING_BRANCH)?;
        Ok(())
    }

    fn read_repository(&mut self) -> Result<(), &'static str> {
        // Read official categories
        // self.official_categories = match Category::read_categories(
        //     &format!(
        //         "{path}/{root}/{source}/{platform}",
        //         path = self.repository.path,
        //         root = OBEX_ROOT_PATH,
        //         source = "official",
        //         platform = "p1"
        //     ))
        // {
        //     Ok(c) => c,
        //     Err(_) => return Err("Error reading obex categories"),
        // };

        // Read community categories
        self.community_categories = match Category::read_categories(
            &format!(
                "{path}/{root}/{source}/{platform}",
                path = self.repository.path,
                root = OBEX_ROOT_PATH,
                source = "community",
                platform = "p1"
            ))
        {
            Ok(c) => c,
            Err(_) => return Err("Error reading obex categories"),
        };

        // Read official objects
        // self.official_objects = match Object::read_objects(
        //     &format!(
        //         "{path}/{root}/{source}/{platform}/All",
        //         path = self.repository.path,
        //         root = OBEX_ROOT_PATH,
        //         source = "official",
        //         platform = "p1"
        //     ))
        // {
        //     Ok(o) => o,
        //     Err(_) => return Err("Error reading obex objects"),
        // };

        // Read community objects
        self.community_objects = match Object::read_objects(
            &format!(
                "{path}/{root}/{source}/{platform}/All",
                path = self.repository.path,
                root = OBEX_ROOT_PATH,
                source = "community",
                platform = "p1"
            ))
        {
            Ok(o) => o,
            Err(_) => return Err("Error reading obex objects"),
        };

        Ok(())
    }
}