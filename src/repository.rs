use git2;

pub struct Repository {
    pub path: String,
    repo: git2::Repository,
}

impl Repository {
    pub fn clone(url: &str, path: &str) -> Result<Repository, &'static str> {
        // Clone git repository
        let new_repo = match git2::Repository::clone(url, path) {
            Ok(repo) => repo,
            Err(_) => return Err("Error cloning repository"),
        };

        Ok(Repository {
            path: String::from(path),
            repo: new_repo,
        })
    }

    pub fn open(path: &str) -> Result<Repository, &'static str> {
        // Open git repository
        let new_repo = match git2::Repository::open(path) {
            Ok(repo) => repo,
            Err(_) => return Err("Error opening repository"),
        };

        Ok(Repository {
            path: String::from(path),
            repo: new_repo,
        })
    }

    pub fn pull(&self, branch: &str) -> Result<(), &'static str> {
        // Get remote
        let mut remote = match self.repo.find_remote("origin") {
            Ok(r) => r,
            Err(_) => return Err("Unable to find remote 'origin'"),
        };

        // Fetch updates for branch
        if remote.fetch(&[branch], None, None).is_err() {
            return Err("Error fetching branch");
        }

        // Get FETCH_HEAD
        let fetch_head = match self.repo.find_reference("FETCH_HEAD") {
            Ok(head) => head,
            Err(_) => return Err("Error finding FETCH_HEAD"),
        };

        // Create commit for merge
        let commit = match self.repo.reference_to_annotated_commit(&fetch_head) {
            Ok(c) => c,
            Err(_) => return Err("Error creating commit for fetch"),
        };

        // Analyze merge
        let merge = match self.repo.merge_analysis(&[&commit]) {
            Ok(m) => m,
            Err(_) => return Err("Error analyizing merge")
        };

        // Validate analysis
        if merge.0.is_up_to_date() {
            Ok(())
        } else if merge.0.is_fast_forward() {
            let mut branch_head = match self.repo.find_reference(&format!("refs/heads/{}", branch)) {
                Ok(reference) => reference,
                Err(_) => return Err("Error finding branch HEAD")
            };

            // Fast-forward
            match branch_head.set_target(commit.id(), "Fast-forward") {
                Ok(_) => (),
                Err(_) => return Err("Error fast-forwarding branch")
            }

            // Set HEAD for current branch
            match self.repo.set_head(&format!("refs/heads/{}", branch)) {
                Ok(_) => (),
                Err(_) => return Err("Error setting branch HEAD")
            };

            // Checkout new HEAD
            match self.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().safe())) {
                Ok(_) => Ok(()),
                Err(_) => Err("Error checking out new HEAD")
            }
        } else {
            Err("Error merging fetched changes")
        }
    }
}
