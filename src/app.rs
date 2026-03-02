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
        let repo_owner = &self.config.github_repository.owner.clone();
        let repo_name = &self.config.github_repository.repository.clone();

        let mut latest = &self.get_latest_release(repo_owner, repo_name).await;

        &mut latest.unwrap();

        Ok(())
    }

    pub async fn init_config(&mut self) -> anyhow::Result<()> {
        let config_dir = dirs::config_dir().unwrap().join("create50");
        let config_file = &config_dir.join("config.toml");

        info!(
            "Attempting to load configuration file from {:?}",
            config_file
        );

        if !config_file.exists() {
            warn!("Configuration file does not exist. Attempting to create.");

            if let Err(e) = tokio::fs::create_dir_all(&config_dir).await {
                error!(
                    "Unable to create configuration directory at {:?}: {}",
                    config_dir, e
                );
                std::process::exit(0);
            }

            if let Err(e) = &self.recreate_config(config_dir).await {
                error!("Unable to recreate configuration file: {}", e);
                std::process::exit(1);
            }
        } else {
            info!("Reading config file {:?}", config_file);
            let config_file_contents = tokio::fs::read_to_string(config_file).await;

            if let Err(e) = config_file_contents {
                error!("Unable to read configuration file: {}", e);
                std::process::exit(1);
            }

            info!("Deserializing config file.");
            let deserialized_config =
                ron::de::from_str::<config::Config>(&config_file_contents.unwrap());

            if let Err(e) = deserialized_config {
                error!("Unable to deserialize configuration file: {}", e);
                std::process::exit(1);
            }

            self.config = deserialized_config.unwrap();

            info!("Loaded configuration file successfully.");
        }

        Ok(())
    }

    pub async fn recreate_config(&mut self, config_dir: std::path::PathBuf) -> anyhow::Result<()> {
        let config_file = &config_dir.join("config.toml");
        let content = ron::ser::to_string_pretty(&self.config, PrettyConfig::new())?;

        tokio::fs::write(config_file, content).await?;

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
