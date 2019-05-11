extern crate url;
extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
extern crate git2;
extern crate shiplift;

use std::{fs, env};
use projects::config::JarvisConfig;
use std::time::Duration;
use std::thread;
use std::path::PathBuf;

mod projects;
mod build_jobs;

fn main() {
    println!("{}", banner());
    let config = load_config();
    make_sure_workdir_is_usable(&config.workdir);
    if executable_is_run_in_pretest_hook() {
        execute_pretest(&config)
    } else {
        poll_git_and_build_if_changed_loop(&config)
    }
}

fn executable_is_run_in_pretest_hook() -> bool {
    return true;
}

fn load_config() -> JarvisConfig {
    JarvisConfig::read_from_file("jarvis-config.yml")
}

fn make_sure_workdir_is_usable(config_workdir: &String) {
    fs::create_dir_all(config_workdir).expect("Could not create workdir.");
    println!("Workdir is set to '{}'", config_workdir);
}

fn poll_git_and_build_if_changed_loop(config:&JarvisConfig) -> ! {
    loop {
        for project in &config.projects {
            let workdir = PathBuf::from(&config.workdir);
            project.build_if_necessary_inside(&workdir);
        }
        thread::sleep(Duration::from_secs(config.default_polling_interval));
    }
}

fn execute_pretest(config:&JarvisConfig) {
    let result = build_jobs::container::Container::new(String::from("./Dockerfile"), vec![]).build();
    println!("Build result: {:?}", result);
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
