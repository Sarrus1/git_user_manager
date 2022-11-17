use colored::Colorize;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use dirs::home_dir;
use read_input::shortcut::input;
use std::io::Write;
use std::{collections::HashMap, fs::File, io::Read, process::exit};

use crate::user::User;

/// Read and return the serialized user store
///
/// # Arguments
///
/// * `create` - Should the store be created if it does not exists
pub fn read_user_store(create: bool) -> Option<HashMap<String, User>> {
    let store_path = home_dir()?.join(".git_user_manager.config.toml");
    if !store_path.exists() {
        if create {
            match File::create(&store_path) {
                Ok(_file) => println!("Store file did not exist and was created."),
                Err(_) => panic!("Store file does not exist at {:?}", store_path),
            };
        } else {
            println!(
                "Error: Git User Manager's store file does not exist at {:?}\n\
                \n\
                Create it by running\n\
                \n\
                \tgum store -a\n",
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

    Some(data)
}

/// Write the serialized user store
///
/// # Arguments
///
/// * `store` - The store to write
pub fn write_user_store(store: &HashMap<String, User>) -> Option<()> {
    let store_path = home_dir()?.join(".git_user_manager.config.toml");
    let mut file = match File::create(&store_path) {
        Ok(file) => file,
        Err(_) => panic!("Could not create config file at {:?}", store_path),
    };
    let data = toml::to_string_pretty(&store).unwrap();
    write!(file, "{}", data).unwrap();

    Some(())
}

/// Add a new entry to the store from user input
pub fn add_to_store() -> Option<()> {
    let mut store = read_user_store(true)?;

    let mut user_config = User {
        user: None,
        email: None,
        name: None,
        signingkey: None,
        use_config_only: None,
    };

    print!("{}", "Enter the id of the configuration: ".yellow());
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
    write_user_store(&store);

    Some(())
}

/// Returns the passed key or prompts the user to select one
///
/// # Arguments
///
/// * `input_key` - Optional key of the user
pub fn get_key_from_prompt(input_key: &Option<String>) -> String {
    let store = read_user_store(true).unwrap();
    if input_key.is_some() {
        return String::from(input_key.as_ref().unwrap());
    }

    let mut keys: Vec<String> = vec![];
    for key in store.keys() {
        keys.push(key.clone());
    }

    let key_id = Select::with_theme(&ColorfulTheme::default())
        .items(&keys)
        .with_prompt("Select a user:")
        .interact_on_opt(&Term::stderr())
        .unwrap();

    return match key_id {
        Some(index) => keys[index].clone(),
        None => {
            println!("Nothing selected");
            exit(1)
        }
    };
}

/// Delete an entry from the store from user input
///
/// # Arguments
///
/// * `input_key` - Optional key of the user to delete
pub fn delete_from_store(input_key: &Option<String>) -> Option<()> {
    let mut store = read_user_store(true)?;
    let key = get_key_from_prompt(&input_key);

    let accept = Confirm::new()
        .with_prompt(format!(
            "{} {} {}",
            "Are you sure you want to delete user".yellow(),
            key.bold(),
            "from the config?".yellow(),
        ))
        .interact()
        .unwrap();

    if !accept {
        println!("Aborted");
        exit(1)
    }

    store.remove(&key);
    write_user_store(&store);
    println!("User {} was successfully deleted.", key);

    Some(())
}
