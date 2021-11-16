use std::path::PathBuf;

use use_case::HasPageRepository;

use crate::adapter::FsPageRepository;

pub struct App {
    fs_repository: FsPageRepository,
}

impl App {
    pub fn new(data_dir: PathBuf) -> Self {
        let fs_repository = FsPageRepository::new(data_dir);
        Self { fs_repository }
    }
}

impl HasPageRepository for App {
    type PageRepository = FsPageRepository;

    fn page_repository(&self) -> &Self::PageRepository {
        &self.fs_repository
    }
}
