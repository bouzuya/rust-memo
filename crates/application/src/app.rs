use std::path::PathBuf;

use use_case::{HasEditUseCase, HasNewPageUseCase, HasPageRepository};

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

impl HasEditUseCase for App {
    type EditUseCase = App;

    fn edit_use_case(&self) -> &Self::EditUseCase {
        self
    }
}

impl HasNewPageUseCase for App {
    type NewPageUseCase = App;

    fn new_page_use_case(&self) -> &Self::NewPageUseCase {
        self
    }
}
