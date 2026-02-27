// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0/

use std::process::Command;

pub fn set_flatpack_repo(repo_name: &String, flatpack_url :&String){
    Command::new("flatpak")
        .arg("remote-add")
        .arg(repo_name)
        .arg(flatpack_url)
        .status()
        .expect("Error to set flatpack repo");
}