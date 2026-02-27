// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0/

use std::process::Command;

pub fn remove_pkg(pkg: &String) -> bool {
    // Попытка через pacman
    if let Ok(status) = Command::new("sudo").arg("pacman").arg("-Rs").arg(pkg).status() {
        if status.success() {
            return true;
        }
    }
    // Попытка через yay
    if let Ok(status) = Command::new("yay").arg("-Rs").arg(pkg).status() {
        if status.success() {
            return true;
        }
    }
    // Попытка через flatpak
    if let Ok(status) = Command::new("flatpak").arg("uninstall").arg(pkg).status() {
        if status.success() {
            return true;
        }
    }
    false // ничего не сработало
}