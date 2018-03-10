use std::path::Path;

pub struct BuildJob;

impl BuildJob {

    pub fn new(build_descriptor_file:&Path) -> BuildJob {
        BuildJob
    }

    pub fn run(&self) -> BuildResult {
        BuildResult::Succeeded
    }
}

pub enum BuildResult {
    Succeeded,
    Failed
}