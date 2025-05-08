#![warn(clippy::todo)]

use std::error::Error;
use std::fs;

use crate::cli::Cli;
use crate::cli::Command;
use clap::Parser;
use galvyn::rorm;
use galvyn::rorm::DatabaseDriver;
use galvyn::rorm::config::DatabaseConfig;

mod cli;
mod http;
mod models;
mod modules;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Start => http::server::start().await?,
        Command::MakeMigrations { migrations_dir } => {
            use std::io::Write;

            /// Temporary file to store models in
            const MODELS: &str = "/tmp/.models.json";

            let mut file = fs::File::create(MODELS)?;
            rorm::write_models(&mut file)?;
            file.flush()?;

            rorm::cli::make_migrations::run_make_migrations(
                rorm::cli::make_migrations::MakeMigrationsOptions {
                    models_file: MODELS.to_string(),
                    migration_dir: migrations_dir,
                    name: None,
                    non_interactive: false,
                    warnings_disabled: false,
                },
            )?;

            fs::remove_file(MODELS)?;
        }
        Command::Migrate { migrations_dir } => {
            rorm::cli::migrate::run_migrate_custom(
                DatabaseConfig {
                    driver: DatabaseDriver::SQLite {
                        filename: ":memory:".to_string(),
                    },
                    last_migration_table_name: None,
                },
                migrations_dir,
                false,
                None,
            )
            .await?
        }
    }

    Ok(())
}
