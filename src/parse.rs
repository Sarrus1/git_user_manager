use std::process::exit;

use colored::Colorize;
use dialoguer::Confirm;
use read_input::shortcut::input;

use crate::{
    list::print_user,
    reader::find_user_in_config,
    store::{read_user_store, write_user_store},
};

/// Parse the local Git config into the store
pub fn parse_config_into_store() -> Option<()> {
    let user = find_user_in_config();
    let mut key = String::from(user.user.as_ref().unwrap());
    if key == "" {
        print!(
            "{}",
            "Enter the id of the configuration (leave empty to use): ".yellow(),
        );
        key = input::<String>().get();
    } else {
        print!(
            "{}{}{}",
            "Enter the id of the configuration (leave empty to use ".yellow(),
            key.bold(),
            "): ".yellow()
        );
        let input_key = input::<String>().get();
        if input_key != "" {
            key = input_key;
        }
    }
    print_user(&key, &user, true);

    let accept = Confirm::new()
        .with_prompt(format!(
            "{}",
            "Are you sure you want to add the above user ?".yellow(),
        ))
        .interact()
        .unwrap();

    if !accept {
        println!("Aborted");
        exit(1)
    }
    let mut store = read_user_store(false)?;
    store.insert(key.to_string(), user);
    write_user_store(&store);
    Some(())
}
