use clap::{Parser, Subcommand};
use sqlx::PgPool;
use std::{fs, path::Path, time::SystemTime};

use blockchain_indexer::config::load_config;

#[derive(Parser)]
#[command(name = "migrate", about = "Database migration CLI")]
struct Cli {
    #[command(subcommand)]
    command: MigrateCommand,
}

#[derive(Subcommand)]
enum MigrateCommand {
    /// Manage migrations
    Migrate {
        #[command(subcommand)]
        action: MigrateAction,
    },
}

#[derive(Subcommand)]
enum MigrateAction {
    /// Apply all pending migrations
    Up,

    /// Revert migrations
    Revert {
        /// Revert ALL migrations
        #[arg(long)]
        all: bool,
    },

    /// Create a new migration
    Add {
        /// Migration name
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        MigrateCommand::Migrate { action } => match action {
            MigrateAction::Add { name } => create_migration(&name)?,

            MigrateAction::Up => migrate_up().await?,

            MigrateAction::Revert { all } => migrate_revert(all).await?,
        },
    }

    Ok(())
}

async fn migrate_up() -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect_db().await?;

    println!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Migrations applied");

    Ok(())
}

async fn migrate_revert(all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect_db().await?;
    let migrator = sqlx::migrate!("./migrations");

    if all {
        println!("⚠️  Reverting ALL migrations...");
        migrator.undo(&pool, -1).await?;
    } else {
        println!("Reverting last migration...");
        migrator.undo(&pool, 1).await?;
    }

    println!("Revert completed");
    Ok(())
}

async fn connect_db() -> Result<PgPool, Box<dyn std::error::Error>> {
    let cfg = load_config()?;
    let db = &cfg.database;

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db.username, db.password, db.host, db.port, db.database
    );

    println!("Connecting to database...");
    Ok(PgPool::connect(&database_url).await?)
}

fn create_migration(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let dir = Path::new("migrations");
    fs::create_dir_all(dir)?;
    let up = dir.join(format!("{}_{}.up.sql", timestamp, name));
    let down = dir.join(format!("{}_{}.down.sql", timestamp, name));
    fs::write(&up, "-- UP migration\n")?;
    fs::write(&down, "-- DOWN migration\n")?;
    println!("Created migration:");
    println!(" {}", up.display());
    println!(" {}", down.display());
    Ok(())
}
