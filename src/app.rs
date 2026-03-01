use crate::types::*;
use clap::*;
use log::*;
use octocrab;
use ron::ser::PrettyConfig;
use std::sync::Arc;

pub struct App {
    pub cli: cli::Cli,
    pub github: Arc<octocrab::Octocrab>,
    pub config: config::Config,
}

impl App {
    pub fn new() -> Self {
        Self {
            cli: cli::Cli::parse(),
            github: octocrab::instance(),
            config: config::Config::default(),
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.create_config().await?;

        let repo_owner = &self.config.github_repository.owner.clone();
        let repo_name = &self.config.github_repository.repository.clone();

        let latest = &self.get_latest_release(repo_owner, repo_name).await?;
        println!("{}", latest);

        Ok(())
    }

    pub async fn create_config(&self) -> anyhow::Result<()> {
        let config_dir = dirs::config_dir().unwrap().join("create50");

        tokio::fs::create_dir_all(&config_dir).await?;

        let content = ron::ser::to_string_pretty(&self.config, PrettyConfig::new())?;
        tokio::fs::write(&config_dir.join("config.toml"), content).await?;

        Ok(())
    }

    pub async fn get_latest_release(
        &mut self,
        owner: &String,
        repository: &String,
    ) -> anyhow::Result<String> {
        info!("Getting latest release url from {}/{}", owner, repository);

        let query = &self
            .github
            .repos(owner, repository)
            .list_tags()
            .per_page(1)
            .send()
            .await?;

        if let Some(tag) = query.into_iter().next() {
            Ok(tag.zipball_url.to_string().clone())
        } else {
            Err(anyhow::anyhow!(
                "Unable to find most recent tag from {}/{}",
                &owner,
                &repository
            ))
        }
    }
}
