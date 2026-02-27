use std::env;
pub mod args;
use crate::args::{
    update,
    flatpak,
    install,
    remove
};


fn main() {
    let args: Vec<String> = env::args().collect();

    match &args[1] as &str {
        "-install" => {
            let pkg = &args[2];
            install::install_pkg(pkg);
        },
        "-remove" =>{
            let pkg = &args[2];
            remove::remove_pkg(pkg);
        },
        "-update" =>{
            update::update();
        },
        "-set-flatpak" =>{
            let flatpack_url: &String = &args[3];
            let repo_name: &String = &args[2];
            flatpak::set_flatpack_repo(repo_name, flatpack_url);
        }
        _ => panic!("Not found operation: {}", args[1]),
    }
}