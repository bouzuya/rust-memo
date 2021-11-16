use std::path::PathBuf;

use use_case::HasPageRepository;

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

impl HasPageRepository for App {
    type PageRepository = FsRepository;

    fn page_repository(&self) -> &Self::PageRepository {
        &self.fs_repository
    }
}
