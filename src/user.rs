use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use std::process::exit;

use crate::{
    config::{read_user_config, update_config_lines, write_user_config},
    store::read_user_store,
    utils::git_config_exists,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: Option<String>,
    pub email: Option<String>,
    pub user: Option<String>,
    pub signingkey: Option<String>,
    pub use_config_only: Option<bool>,
}

/// Edit the local git config to use a specified user.
/// Exits if the user does not exist.
///
/// # Arguments
///
/// * `key` - The key of the config in the store
pub fn use_user(key: &String) -> Option<()> {
    let store = read_user_store(false)?;
    if !git_config_exists() {
        panic!("Git config not found at ./.git/config")
    }
    let user = store.get(key.as_str());
    if user.is_none() {
        println!("User {} does not exist.", key.bold());
        println!(
            "Use {} to list all the available users.",
            "gum list".yellow()
        );
        exit(1);
    }
    set_user(user?);

    Some(())
}

/// Sets the values of the passed user struct to the git config file.
///
/// # Arguments
///
/// * `user` - The struct holding the values to write
pub fn set_user(user: &User) -> Option<()> {
    let lines = read_user_config("./.git/config");
    let to_delete = find_user_in_config(&lines);
    let new_lines = update_config_lines(&user, &to_delete, &lines)?;
    write_user_config("./.git/config", &new_lines);

    Some(())
}

/// Find the user section in git config and returns corresponding line numbers.
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

    to_delete
}
