use clap::{Args, Parser, Subcommand};
use store::add_to_store;
use user::use_user;

pub mod config;
pub mod store;
pub mod user;
pub mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Replace the local git user
    Use(Use),

    /// Edit the store
    Store(Store),
}

#[derive(Args)]
struct Use {
    /// The name of the user to use
    user: String,
}

#[derive(Args)]
struct Store {
    /// Add a user to the store
    #[arg(short, long)]
    add: bool,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Use(use_) => {
            use_user(&use_.user);
        }
        Commands::Store(store) => {
            if store.add {
                add_to_store();
            }
        }
    }
}
