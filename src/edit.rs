use std::process::exit;

use colored::Colorize;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use read_input::shortcut::input;

use crate::{
    list::user_to_strings,
    store::{get_key_from_prompt, read_user_store, write_user_store},
    user::User,
};

/// Edit an entry in the store from user input
///
/// # Arguments
///
/// * `input_key` - Optional key of the user to delete
pub fn edit_from_store(input_key: &Option<String>) -> Option<()> {
    let mut store = read_user_store(true)?;
    let key = get_key_from_prompt(input_key);
    let user = store.get_mut(&key);
    if user.is_none() {
        println!("User does not exist.");
        exit(1);
    }
    let user = user?;

    loop {
        let index = select_user_property(user);
        if index.is_none() {
            break;
        }
        let val_key = user.index_to_key(index?);
        print!(
            "{}{}{}",
            "Enter the new value of the configuration for ".yellow(),
            val_key.bold(),
            ": ".yellow()
        );
        let value = input::<String>().get();

        user.insert(val_key.as_str(), value.as_str());
    }

    write_user_store(&store);
    println!("User {} was successfully edited.", key);

    Some(())
}

/// Prompt the user to select a user property and return its index
///
/// # Arguments
///
/// * `user` - User object to select the property of.
fn select_user_property(user: &User) -> Option<usize> {
    let lines = user_to_strings(&"".to_string(), user, true).unwrap();

    let key_id = Select::with_theme(&ColorfulTheme::default())
        .items(&lines[1..])
        .with_prompt("Select a value to edit:")
        .interact_on_opt(&Term::stderr())
        .unwrap();

    return match key_id {
        Some(index) => Some(index),
        None => None,
    };
}
