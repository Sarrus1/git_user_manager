use lazy_static::lazy_static;
use regex::Regex;

use crate::{config::read_user_config, user::User, utils::git_config_exists};

/// Extract the user config from the local Git config
pub fn find_user_in_config() -> User {
    if !git_config_exists() {
        panic!("Git config not found at ./.git/config")
    }
    let lines = read_user_config("./.git/config");
    let mut in_user = false;
    let mut user = User {
        name: Some(String::from("")),
        email: Some(String::from("")),
        user: Some(String::from("")),
        signingkey: Some(String::from("")),
        use_config_only: Some(false),
    };
    for l in lines {
        let l_trimed = l.trim();
        if l_trimed.starts_with("#") {
            // Commented out line
            continue;
        }
        if l_trimed.starts_with("[user]") {
            in_user = true;
            continue;
        }
        if !in_user || l_trimed.starts_with("[") {
            // Not a relevant field
            in_user = false;
            continue;
        }

        match extract_config_value(&l) {
            None => continue,
            Some((key, val)) => {
                user.insert(key.as_str(), val.as_str());
            }
        }
    }
    return user;
}

/// Extract a Git config value from a line.
/// Return [None] if there is no match.
///
/// # Arguments
///
/// * `input` - Line to extract the value from
fn extract_config_value(input: &String) -> Option<(String, String)> {
    if input.trim() == "" {
        return None;
    }
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\w+)\s*=\s*(.+)").unwrap();
    }
    let caps = RE.captures(input).unwrap();
    return Some((String::from(&caps[1]), String::from(&caps[2])));
}
