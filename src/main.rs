use clap::{Args, Parser, Subcommand};
use edit::edit_from_store;
use list::print_all_users;
use parse::parse_config_into_store;
use store::{add_to_store, delete_from_store};
use user::use_user;

pub mod config;
pub mod edit;
pub mod list;
pub mod parse;
pub mod reader;
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

    /// Parse a user from the local Git config
    Parse(Parse),

    /// Edit a user from the store
    Edit(Edit),
}

#[derive(Args)]
struct Use {
    /// The name of the user to use
    user: Option<String>,
}

#[derive(Args)]
struct Add {}

#[derive(Args)]
struct Delete {
    /// The name of the user to delete
    #[arg(index = 1)]
    user: Option<String>,
}

#[derive(Args)]
struct List {
    /// Enable detailed view of each user
    #[arg(short, long)]
    detailed: bool,
}

#[derive(Args)]
struct Parse {}

#[derive(Args)]
struct Edit {
    /// The name of the user to edit
    #[arg(index = 1)]
    user: Option<String>,
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
        Commands::Parse(_) => {
            parse_config_into_store();
        }
        Commands::Edit(args) => {
            edit_from_store(&args.user);
        }
    }
}
