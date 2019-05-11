use std::path::Path;
use std::fs::File;
use std::io;
use serde_yaml;

#[derive(Debug, PartialEq, PartialOrd, Deserialize)]
pub struct BuildDescriptor {
    pub container: String,
    pub pre_script: Vec<Script>,
    pub script: Vec<Script>
}

type Script = String;

impl BuildDescriptor {
    pub fn read_from(build_descriptor_file:&Path) -> Result<BuildDescriptor, BuildDescriptorError> {
        let file = File::open(build_descriptor_file)?;
        let build_descriptor : BuildDescriptor = serde_yaml::from_reader(file)?;
        Ok(build_descriptor)
    }

}

#[derive(Debug)]
pub enum BuildDescriptorError {
    IoError(io::Error),
    DeserialiationError(serde_yaml::Error)
}

impl From<io::Error> for BuildDescriptorError {
    fn from(e: io::Error) -> Self {
        BuildDescriptorError::IoError(e)
    }
}

impl From<serde_yaml::Error> for BuildDescriptorError {
    fn from(e: serde_yaml::Error) -> Self {
        BuildDescriptorError::DeserialiationError(e)
    }
}