use std::path::PathBuf;

use adapter_fs::FsPageRepository;
use use_case::{
    HasEditPageUseCase, HasEnsureLinksUseCase, HasListPagesUseCase, HasListTitlesUseCase,
    HasNewPageUseCase, HasPageRepository,
};

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

impl HasEditPageUseCase for App {
    type EditPageUseCase = App;

    fn edit_page_use_case(&self) -> &Self::EditPageUseCase {
        self
    }
}

impl HasEnsureLinksUseCase for App {
    type EnsureLinksUseCase = App;

    fn ensure_links_use_case(&self) -> &Self::EnsureLinksUseCase {
        self
    }
}

impl HasListPagesUseCase for App {
    type ListPagesUseCase = App;

    fn list_pages_use_case(&self) -> &Self::ListPagesUseCase {
        self
    }
}

impl HasListTitlesUseCase for App {
    type ListTitlesUseCase = App;

    fn list_titles_use_case(&self) -> &Self::ListTitlesUseCase {
        self
    }
}

impl HasNewPageUseCase for App {
    type NewPageUseCase = App;

    fn new_page_use_case(&self) -> &Self::NewPageUseCase {
        self
    }
}
