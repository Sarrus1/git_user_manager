<div align="center">
  <h1><code>git_user_manager</code></h1>
  <p>
    <strong>CLI tool written in Rust to quickly switch between git users
    </strong>
  </p>
  <p style="margin-bottom: 0.5ex;">
    <a href="https://github.com/Sarrus1/git_user_manager/actions/workflows/release.yml">
      <img
        alt="Github release status"
        src="https://github.com/Sarrus1/git_user_manager/actions/workflows/release.yml/badge.svg"
      />
    </a>
    <a href="https://github.com/Sarrus1/git_user_manager/releases/latest">
      <img alt="Crates.io" src="https://img.shields.io/crates/d/git_user_manager">
    </a>
    <a href="https://github.com/Sarrus1/git_user_manager/releases/latest">
      <img alt="Crates.io" src="https://img.shields.io/crates/v/git_user_manager">
    </a>
    <img alt="GitHub" src="https://img.shields.io/github/license/Sarrus1/git_user_manager">
  </p>
  <img src="https://raw.githubusercontent.com/Sarrus1/git_user_manager/main/img/logo.png" alt="Logo">
</div>

## How to install

### Windows

1. Download the [latest release](https://github.com/Sarrus1/git_user_manager/releases/latest) for windows.
2. Extract the zip file and move `gum.exe` to `C:\Windows\System32`.
3. In your command prompt, run `gum --help`.

### Linux/MacOS

#### Homebrew

1. Setup tap: `brew tap Sarrus1/formulas`
2. Install: `brew install git_user_manager`

#### Manual

1. Download the [latest release](https://github.com/Sarrus1/git_user_manager/releases/latest) for Linux or MacOS.
2. Extract the tar file and move `gum` to `/usr/local/bin`.
3. In your command prompt, run `gum --help`.

## How to use

### Add a user to the store

Users are stored in the store. To add one, use `gum store -a` and follow the prompt.

### Switch to a user

When in a git repository, switch to a different user by using `gum use <name>`.
