use clap::{Args, Parser, Subcommand};
use list::print_all_users;
use store::{add_to_store, delete_from_store};
use user::use_user;

pub mod config;
pub mod list;
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

    /// Add a user to the store
    Add(Add),

    /// Delete a user from the store
    Delete(Delete),

    /// List all available users
    List(List),
}

#[derive(Args)]
struct List {
    /// Enable detailed view of each user
    #[arg(short, long)]
    detailed: bool,
}

#[derive(Args)]
struct Use {
    /// The name of the user to use
    user: String,
}

#[derive(Args)]
struct Delete {
    /// The name of the user to delete
    #[arg(index = 1)]
    user: Option<String>,
}

#[derive(Args)]
struct Add {}

#[derive(Args)]
struct Store {
    /// Delete a user from the store
    #[arg(short, long, exclusive = true)]
    delete: bool,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Use(args) => {
            use_user(&args.user);
        }
        Commands::Delete(args) => {
            delete_from_store(&args.user);
        }
        Commands::Add(_) => {
            add_to_store();
        }
        Commands::List(args) => {
            print_all_users(args.detailed);
        }
    }
}
