use clap::{Args, Parser, Subcommand};
use colored::*;
use dirs::home_dir;
use read_input::shortcut::input;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: Option<String>,
    email: Option<String>,
    user: Option<String>,
    signingkey: Option<String>,
    use_config_only: Option<bool>,
}

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
    Config(Config),
}

#[derive(Args)]
struct Use {
    /// The name of the user to use
    user: String,
}

#[derive(Args)]
struct Config {
    /// The name of the user to use
    #[arg(short, long)]
    add: bool,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Use(use_) => {
            use_user(&use_.user);
        }
        Commands::Config(config) => {
            if config.add {
                add_config();
            }
        }
    }
}

fn add_config() -> Option<()> {
    let mut store = read_user_store(true)?;

    let mut user_config = User {
        user: None,
        email: None,
        name: None,
        signingkey: None,
        use_config_only: None,
    };

    print!("{}", "Enter the name of the configuration: ".yellow());
    let key = input::<String>().get();

    if store.contains_key(&key) {
        println!(
            "{} {} {}",
            "User".red(),
            key.underline(),
            "already exists.".red()
        );
        exit(1);
    }

    print!("{}", "user: ".yellow());
    let user = input::<String>().get();
    if user != "" {
        user_config.user = Some(user);
    }

    print!("{}", "name: ".yellow());
    let name = input::<String>().get();
    if name != "" {
        user_config.name = Some(name);
    }

    print!("{}", "email: ".yellow());
    let email = input::<String>().get();
    if email != "" {
        user_config.email = Some(email);
    }

    print!("{}", "signingkey: ".yellow());
    let signingkey = input::<String>().get();
    if signingkey != "" {
        user_config.signingkey = Some(signingkey);
    }

    print!("{}", "useConfigOnly: ".yellow());
    user_config.use_config_only = Some(input::<bool>().get());

    store.insert(key, user_config);
    write_user_store(store);

    Some(())
}

/// Edit the local git config to use a specified user
///
/// # Arguments
///
/// * `key` - The key of the config in the store
fn use_user(key: &String) -> Option<()> {
    let store = read_user_store(false)?;
    if !git_config_exists() {
        panic!("Git config not found at ./.git/config")
    }
    let user = store.get(key.as_str()).unwrap();
    set_user(user);

    Some(())
}

/// Returns whether a git config if found
fn git_config_exists() -> bool {
    return Path::new("./.git/config").exists();
}

/// Sets the values of the passed user struct to the git config file
///
/// # Arguments
///
/// * `user` - The struct holding the values to write
fn set_user(user: &User) -> Option<()> {
    let lines = read_user_config("./.git/config");
    let to_delete = find_user_in_config(&lines);
    let new_lines = update_lines(&user, &to_delete, &lines)?;
    write_user_config("./.git/config", &new_lines);

    Some(())
}

/// Find the user section in git config and returns corresponding line numbers
///
/// # Arguments
///
/// * `lines` - Vector of lines of the git config file
fn find_user_in_config(lines: &Vec<String>) -> Vec<usize> {
    let mut in_user = false;
    let mut to_delete: Vec<usize> = Vec::new();
    for (l_nb, l) in lines.iter().enumerate() {
        let l_trimed = l.trim();
        if l_trimed.starts_with("#") {
            // Commented out line
            continue;
        }
        if l_trimed.starts_with("[user]") {
            in_user = true;
            to_delete.push(l_nb);
            continue;
        }
        if !in_user || l_trimed.starts_with("[") {
            // Not a relevant field
            in_user = false;
            continue;
        }
        to_delete.push(l_nb);
    }
    return to_delete;
}

/// Update the original lines from the git config
///
/// # Arguments
///
/// * `user` - User config to use
/// * `to_delete` - Vector of line numbers to delete
/// * `lines` - Original lines of the config file
fn update_lines(user: &User, to_delete: &Vec<usize>, lines: &Vec<String>) -> Option<Vec<String>> {
    let mut new_lines: Vec<String> = Vec::new();
    for (l_nb, l) in lines.iter().enumerate() {
        if to_delete.contains(&l_nb) {
            continue;
        }
        new_lines.push(l.to_string());
    }
    let insert_idx = match to_delete.len() {
        0 => lines.len(),
        _ => to_delete[0],
    };
    if user.use_config_only.is_some() {
        new_lines.insert(
            insert_idx,
            format!("\tuseConfigOnly = {:?}", user.use_config_only?),
        )
    }
    if user.signingkey.is_some() {
        new_lines.insert(
            insert_idx,
            format!("\tsigningkey = {}", user.signingkey.as_ref()?),
        )
    }
    if user.user.is_some() {
        new_lines.insert(insert_idx, format!("\tuser = {}", user.user.as_ref()?))
    }
    if user.email.is_some() {
        new_lines.insert(insert_idx, format!("\temail = {}", user.email.as_ref()?))
    }
    if user.name.is_some() {
        new_lines.insert(insert_idx, format!("\tname = {}", user.name.as_ref()?))
    }
    new_lines.insert(insert_idx, String::from("[user]"));

    return Some(new_lines);
}

/// Write a git config file given a vector of strings
///
/// # Arguments
///
/// * `file_path` - Path to the git config file
/// * `lines` - Lines to write
fn write_user_config(file_path: &str, lines: &Vec<String>) {
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(_) => panic!("Cannot write to git config"),
    };
    for l in lines {
        write!(file, "{}\n", l).expect("Cannot write data");
    }
}

/// Read a git config file and return its content as a vector of strings
///
/// # Arguments
///
/// * `file_path` - Path to the git config files
fn read_user_config(file_paths: &str) -> Vec<String> {
    let mut file = match File::open(file_paths) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents
        .split("\n")
        .map(|s: &str| s.to_string())
        .collect();

    return lines;
}

/// Read and return the serialized user store
///
/// # Arguments
///
/// * `create` - Should the store be created if it does not exists
fn read_user_store(create: bool) -> Option<HashMap<String, User>> {
    let store_path = home_dir()?.join(".git_user_manager.config.toml");
    if !store_path.exists() {
        if create {
            match File::create(&store_path) {
                Ok(_file) => println!("Config file did not exist and was created."),
                Err(_) => panic!("Config file does not exist at {:?}", store_path),
            };
        } else {
            println!(
                "Error: Git User Manager's configuration file does not exist at {:?}\n\
                \n\
                Create it by running\n\
                \n\
                \tgum config -a\n",
                store_path
            );
            exit(1);
        }
    }
    let mut file = match File::open(&store_path) {
        Ok(file) => file,
        Err(_) => panic!("Config file does not exist at {:?}", store_path),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect(format!("Failed to read the config file at {:?}", store_path).as_str());
    let data: HashMap<String, User> = toml::from_str(file_contents.as_str()).unwrap();
    return Some(data);
}

/// Write the serialized user store
///
/// # Arguments
///
/// * `store` - The store to write
fn write_user_store(store: HashMap<String, User>) -> Option<()> {
    let store_path = home_dir()?.join(".git_user_manager.config.toml");
    let mut file = match File::create(&store_path) {
        Ok(file) => file,
        Err(_) => panic!("Could not create config file at {:?}", store_path),
    };
    let data = toml::to_string_pretty(&store).unwrap();
    write!(file, "{}", data).unwrap();

    return Some(());
}
