// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0/

use std::process::Command;

pub fn update(){
    Command::new("yay").arg("-Syu"); 
    Command::new("flatpuck").arg("update"); 
}