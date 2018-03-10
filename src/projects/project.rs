use url::Url;
use std::fs;
use std::fs::{DirEntry};
use std::path::{PathBuf, Path};
use projects::repository::{Repository};
use projects::build_job::BuildJob;

#[derive(Debug, PartialEq, PartialOrd, Deserialize)]
pub struct Project {
    name: String,
    url: String,
}

impl Project {
    pub fn new(name:&str, url:&str) -> Project {
        Project {name: String::from(name), url: String::from(url)}
    }

    pub fn build_if_necessary_inside(&self, workdir: &Path) {
        let repo = Repository::new(&self.repo_path(workdir), self.url.clone());
        if repo.is_unchanged() {
            return;
        }
        println!("Repo changed, running build!");
        let build_job = BuildJob::new(repo.build_descriptor_file().as_path());
        build_job.run();
    }

    fn repo_path(&self, workdir:&Path) -> PathBuf {
        let mut buf = PathBuf::from(workdir);
        buf.push(self.name.clone());
        buf
    }

}

