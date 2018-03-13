use std::path::{Path, PathBuf};
use build_jobs::build_descriptor::{BuildDescriptor, BuildDescriptorError};
use build_jobs::container::Container;

pub struct BuildJob {
    build_descriptor_file_path: PathBuf
}

impl BuildJob {
    pub fn new(build_descriptor_file:&Path) -> BuildJob {
        BuildJob {build_descriptor_file_path: PathBuf::from(build_descriptor_file)}
    }

    pub fn run(&self) -> Result<BuildSuccess, BuildFailure> {
        let descriptor = BuildDescriptor::read_from(self.build_descriptor_file_path.as_path())?;
        let container = Container::new( descriptor.container, descriptor.pre_script );
        container.build();
        container.run_script(descriptor.script);
        Ok(BuildSuccess)
    }
}

#[derive(Debug)]
pub struct BuildSuccess;

#[derive(Debug)]
pub enum  BuildFailure {
    BuildDescriptorError(BuildDescriptorError)
}

impl From<BuildDescriptorError> for BuildFailure {
    fn from(e: BuildDescriptorError) -> Self {
        BuildFailure::BuildDescriptorError(e)
    }
}