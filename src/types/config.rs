use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub github_repository: GithubRepository,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubRepository {
    pub owner: String,
    pub repository: String,
}

impl Default for GithubRepository {
    fn default() -> Self {
        Self {
            owner: "OrdinaryHoppou".to_string(),
            repository: "cs50_template".to_string(),
        }
    }
}
