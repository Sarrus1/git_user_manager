use std::{
    fs::File,
    io::{Read, Write},
};

use crate::user::User;

/// Write a git config file given a vector of strings
///
/// # Arguments
///
/// * `file_path` - Path to the git config file
/// * `lines` - Lines to write
pub fn write_user_config(file_path: &str, lines: &Vec<String>) {
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
pub fn read_user_config(file_path: &str) -> Vec<String> {
    let mut file = match File::open(file_path) {
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

/// Update the original lines from the git config
///
/// # Arguments
///
/// * `user` - User config to use
/// * `to_delete` - Vector of line numbers to delete
/// * `lines` - Original lines of the config file
pub fn update_config_lines(
    user: &User,
    to_delete: &Vec<usize>,
    lines: &Vec<String>,
) -> Option<Vec<String>> {
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

    Some(new_lines)
}
