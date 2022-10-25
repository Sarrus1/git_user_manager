use lazy_static::lazy_static;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    email: String,
    user: String,
    signingkey: String,
    use_config_only: bool,
}

impl User {
    fn insert(&mut self, key: &str, val: &str) {
        let val_string = String::from(val.trim());
        match key {
            "name" => self.name = val_string,
            "email" => self.email = val_string,
            "user" => self.user = val_string,
            "signingkey" => self.signingkey = val_string,
            "useConfigOnly" => self.use_config_only = val_string == "true",
            _ => println!("Unsuported user key {}", key),
        }
    }
}

fn read_config() -> HashMap<String, User> {
    let mut file = match File::open("./test.toml") {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let data: HashMap<String, User> = toml::from_str(file_contents.as_str()).unwrap();
    data
}

fn main() {
    let data = read_config();
    let user_data = data.get("sarrus").unwrap();
    let lines = lines_from_file("./.git/config");
    let mut user = find_user_in_config(lines);
    println!("{:#?}", user);
}

fn find_user_in_config(lines: Vec<String>) -> User {
    let mut in_user = false;
    let mut user = User {
        name: String::from(""),
        email: String::from(""),
        user: String::from(""),
        signingkey: String::from(""),
        use_config_only: false,
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

fn extract_config_value(input: &String) -> Option<(String, String)> {
    lazy_static! {
        static ref re: Regex = Regex::new(r"(\w+)\s*=\s*(.+)").unwrap();
    }
    let caps = re.captures(input).unwrap();
    return Some((String::from(&caps[1]), String::from(&caps[2])));
}

fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
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
    lines
}
