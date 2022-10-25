use std::path::Path;

/// Returns whether a git config if found
pub fn git_config_exists() -> bool {
    return Path::new("./.git/config").exists();
}
