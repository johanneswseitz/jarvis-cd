use shiplift::{PullOptions, Docker, BuildOptions, ContainerOptions, ExecContainerOptions};
use shiplift;

pub struct Container {
    base_image: String,
    build_instructions: Vec<String>
}

impl Container {
    pub fn new(base_image:String, build_instructions:Vec<String>) -> Container {
        Container {base_image, build_instructions}
    }


    pub fn build(&self) -> Result<(), BuildError> {
        let docker = Docker::new();
        let info = docker.containers()
            .create(&ContainerOptions::builder(self.base_image.as_ref())
            .entrypoint("/bin/bash -c uname -a").build())?;
        let containers = docker.containers();
        let container = containers.get(&info.Id);
        container.start();
        let result = container.wait()?;
        println!("{:?}", result);
        if result.StatusCode != 0 {
            Err(BuildError::StatusCode(result.StatusCode))
        } else {
            Ok(())
        }
    }

    pub fn run_script(&self, script: Vec<String>){

    }
}

#[derive(Debug)]
pub enum BuildError {
    DockerNotRunning,
    DockerError(shiplift::Error),
    StatusCode(u64)
}

impl From<shiplift::Error> for BuildError {
    fn from(e: shiplift::Error) -> Self {
        match e {
            shiplift::Error::Http(e) => BuildError::DockerNotRunning,
            _ => BuildError::DockerError(e)
        }
    }
}

