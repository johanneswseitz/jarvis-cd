extern crate url;
extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
extern crate git2;
extern crate shiplift;

use std::fs;
use projects::config::JarvisConfig;
use std::time::Duration;
use std::thread;
use std::path::Path;


mod projects;
mod build_jobs;


//fn main() {
//    println!("{}", banner());
//    loop{
//        let config_file = JarvisConfig::read_from_file("jarvis-config.yml");
//        fs::create_dir_all(&config_file.workdir);
//        println!("Workdir is set to '{}'", &config_file.workdir);
//        let workdir = Path::new(&config_file.workdir);
//        for project in config_file.projects {
//            project.build_if_necessary_inside(workdir);
//        }
//        thread::sleep(Duration::from_secs(30));
//    }
//}

fn main() {

    build_jobs::container::Container::new(String::from("ubuntu:16.04"), vec![]).build();
}

fn banner() -> String { String::from(r"
       __       ___      .______     ____    ____  __       _______.     ______  _______
      |  |     /   \     |   _  \    \   \  /   / |  |     /       |    /      ||       \
      |  |    /  ^  \    |  |_)  |    \   \/   /  |  |    |   (----`   |  ,----'|  .--.  |
.--.  |  |   /  /_\  \   |      /      \      /   |  |     \   \       |  |     |  |  |  |
|  `--'  |  /  _____  \  |  |\  \----.  \    /    |  | .----)   |      |  `----.|  '--'  |
 \______/  /__/     \__\ | _| `._____|   \__/     |__| |_______/        \______||_______/
 ")
}
