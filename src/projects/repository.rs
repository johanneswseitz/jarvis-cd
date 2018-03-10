use std::path::{Path,PathBuf};

use git2::Repository as GitRepository;
use git2::{Oid, MergeOptions};
use git2;

pub struct Repository {
    path: PathBuf,
    url: String
}

impl Repository {
    pub fn new(path: &Path, url:String) -> Repository {
        Repository {path: PathBuf::from(path), url }
    }

    pub fn is_unchanged(&self) -> bool {
        if self.is_new_repository() {
            println!("New repository, cloning...");
            self.clone();
            return false;
        } else {
            println!("Pulling for changes...");
            self.pull().expect("Git pull failed") == ScmStatus::Unchanged
        }
    }

    fn is_new_repository(&self) -> bool {
        !self.path.exists()
    }

    fn clone(&self){
        GitRepository::clone(&self.url, self.path.clone()).expect("Clone failed");
    }

    fn pull(&self) -> Result<ScmStatus, git2::Error> {
        let repository = GitRepository::open(self.path.clone())?;

        let head_before_pull = repository.head()?;
        repository.find_remote("origin")?.fetch(&["master"], None, None)?;
        let fetch_head = repository.find_reference("FETCH_HEAD")?;
        let fetch_head_commit = repository.reference_to_annotated_commit(&fetch_head)?;

        let oid = repository.refname_to_id("refs/remotes/origin/master")?;
        let object = repository.find_object(oid, None).unwrap();
        repository.reset(&object, git2::ResetType::Hard, None)?;

        let head_after_pull = repository.head()?;

        if head_before_pull == head_after_pull {
            println!("Nothing changed.");
            Result::Ok(ScmStatus::Unchanged)
        } else {
            println!("Changes detected!");
            Result::Ok(ScmStatus::Changed)
        }
    }

    pub fn build_descriptor_file(&self) -> PathBuf {
        let mut buf = self.path.clone();
        buf.push(".jarvis.yml");
        buf
    }
}

#[derive(PartialEq)]
enum ScmStatus {
    Changed,
    Unchanged
}
