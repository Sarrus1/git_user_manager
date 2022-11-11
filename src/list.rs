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
fn print_user(key: &String, user: &User, detailed: bool) -> Option<()> {
    println!("• {}", key.bold());
    if !detailed {
        return Some(());
    }
    if user.name.is_some() {
        println!("\t{}: {}", "Name".underline(), user.name.as_ref()?)
    }
    if user.email.is_some() {
        println!("\t{}: {}", "Email".underline(), user.email.as_ref()?)
    }
    if user.user.is_some() {
        println!("\t{}: {}", "User".underline(), user.user.as_ref()?)
    }
    if user.signingkey.is_some() {
        println!(
            "\t{}: {}",
            "Signing Key".underline(),
            user.signingkey.as_ref()?
        )
    }
    if user.use_config_only.is_some() {
        println!(
            "\t{}: {}",
            "Use Config Only".underline(),
            user.use_config_only.as_ref()?
        )
    }

    return Some(());
}
