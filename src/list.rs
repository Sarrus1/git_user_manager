use crate::{store::read_user_store, user::User};
use colored::Colorize;

/// Print all the users to the standard output
///
/// # Arguments
///
/// * `detailed` - Whether to print the attributes of each user
pub fn print_all_users(detailed: bool) -> Option<()> {
    let store = read_user_store(false)?;
    for (key, user) in store {
        print_user(&key, &user, detailed);
    }
    Some(())
}

/// Print a user to the standard output
///
/// # Arguments
///
/// * `key` - Store key of the user
/// * `user` - User object to print
/// * `detailed` - Whether to print the attributes of the user
pub fn print_user(key: &String, user: &User, detailed: bool) -> Option<()> {
    let to_print = user_to_strings(key, user, detailed);
    if to_print.is_none() {
        return None;
    }

    for line in to_print?.iter() {
        if line.starts_with("•") {
            println!("{}", line);
            continue;
        }
        println!("\t{}", line);
    }

    return Some(());
}

/// Convert a user to a vector of pretty printable strings
///
/// # Arguments
///
/// * `key` - Store key of the user
/// * `user` - User object to print
/// * `detailed` - Whether to print the attributes of the user
pub fn user_to_strings(key: &String, user: &User, detailed: bool) -> Option<Vec<String>> {
    let mut res: Vec<String> = vec![];

    res.push(format!("• {}", key.bold()));
    if !detailed {
        return Some(res);
    }
    if user.name.is_some() {
        res.push(format!("{}: {}", "Name".underline(), user.name.as_ref()?))
    }
    if user.email.is_some() {
        res.push(format!("{}: {}", "Email".underline(), user.email.as_ref()?))
    }
    if user.user.is_some() {
        res.push(format!("{}: {}", "User".underline(), user.user.as_ref()?))
    }
    if user.signingkey.is_some() {
        res.push(format!(
            "{}: {}",
            "Signing Key".underline(),
            user.signingkey.as_ref()?
        ))
    }
    if user.use_config_only.is_some() {
        res.push(format!(
            "{}: {}",
            "Use Config Only".underline(),
            user.use_config_only.as_ref()?
        ))
    }

    return Some(res);
}
