use shiplift::{PullOptions, Docker, BuildOptions, ContainerOptions, ExecContainerOptions};

pub struct Container {
    base_image: String,
    build_instructions: Vec<String>
}

impl Container {
    pub fn new(base_image:String, build_instructions:Vec<String>) -> Container {
        Container {base_image, build_instructions}
    }


    pub fn build(&self) {
        let docker = Docker::new();
        let info = docker
            .images()
            .pull(&PullOptions::builder().image(self.base_image.clone()).build())
            .unwrap();
        let info = docker.containers()
            .create(&ContainerOptions::builder(self.base_image.as_ref())
                .entrypoint("/bin/bash -c uname -a").build())
            .unwrap();
        println!("CONTAINER CREATION: {:?}", info);
        let containers = docker.containers();
        let container = containers.get(&info.Id);
        container.start();
        let result = container.wait();
        println!("{:?}", result);
    }

    pub fn run_script(&self, script: Vec<String>){

    }
}