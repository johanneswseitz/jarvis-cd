use projects::project::Project;
use std::fs::File;
use serde_yaml;

#[derive(Debug, PartialEq, PartialOrd, Deserialize)]
pub struct JarvisConfig {
    pub projects : Vec<Project>,
    pub workdir : String
}

impl JarvisConfig {
    pub fn read_from_file(config_file:&str) -> JarvisConfig {
        let mut file = File::open(config_file).expect("Config file not found");
        let config: JarvisConfig= serde_yaml::from_reader(file).unwrap();
        config
    }
}


mod test{
    fn it_should_not_allow_duplicate_names() {

    }

    fn it_should_not_allow_names_that_are_not_kebap_case() {

    }
}