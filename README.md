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
      <img 
        alt="GitHub all releases"
        src="https://img.shields.io/github/downloads/sarrus1/git_user_manager/total">
    </a>
  </p>
</div>

## How to install

### Windows

1. Download the [latest release](https://github.com/Sarrus1/git_user_manager/releases/latest) for windows.
2. Extract the zip file and move `gum.exe` to `C:\Windows\System32`.
3. In your command prompt, run `gum --help`.

### Linux

1. Download the [latest release](https://github.com/Sarrus1/git_user_manager/releases/latest) for linux.
2. Extract the tar file and move `gum` to `/usr/local/bin`.
3. In your command prompt, run `gum --help`.

### MacOS

1. Download the [latest release](https://github.com/Sarrus1/git_user_manager/releases/latest) for mac.
2. Extract the tar file and move `gum` to `/usr/local/bin`.
3. In your command prompt, run `gum --help`.

## How to use

### Add a user to the store

Users are stored in the store. To add one, use `gum store -a` and follow the prompt.

### Switch to a user

When in a git repository, switch to a different user by using `gum use <name>`.
