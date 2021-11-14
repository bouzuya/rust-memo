use std::path::PathBuf;

use use_case::HasRepository;

use crate::adapter::FsRepository;

pub struct App {
    fs_repository: FsRepository,
}

impl App {
    pub fn new(data_dir: PathBuf) -> Self {
        let fs_repository = FsRepository::new(data_dir);
        Self { fs_repository }
    }
}

impl HasRepository for App {
    type Repository = FsRepository;

    fn repository(&self) -> &Self::Repository {
        &self.fs_repository
    }
}
