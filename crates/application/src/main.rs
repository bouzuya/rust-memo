mod app;
mod command;
mod helpers;
mod template;
mod url_helpers;

use std::env;

use app::App;
use entity::Query;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "create", about = "Creates a new memo")]
    Create {
        #[structopt(
            long = "title",
            name = "TITLE",
            help = "Creates a new memo with the specified title"
        )]
        title: Option<String>,
    },
    #[structopt(
        name = "edit",
        about = "Creates a new memo that obsoletes the specified memo"
    )]
    Edit {
        #[structopt(
            name = "ID_LIKE_OR_TITLE",
            help = "the id or title of the memo to edit"
        )]
        id_like_or_title: String,
    },
    #[structopt(name = "ensure-links", about = "Ensures the links in the memo")]
    EnsureLinks {
        #[structopt(name = "ID_LIKE", help = "the id of the memo to edit")]
        id_like: Option<String>,
    },
    #[structopt(name = "link", about = "Shows a link for memo")]
    Link {
        #[structopt(name = "ID_LIKE_OR_TITLE", help = "the id or title of the memo")]
        id_like_or_title: String,
    },
    #[structopt(name = "list", about = "Lists memos")]
    List {
        #[structopt(long = "obsoleted", help = "Prints obsoleted memos")]
        obsoleted: bool,
    },
    #[structopt(name = "list-title", about = "Lists memo titles")]
    ListTitle {
        #[structopt(long = "obsoleted", help = "Prints obsoleted memo titles")]
        obsoleted: bool,
    },
    #[structopt(name = "search", about = "Searchs by query")]
    Search {
        #[structopt(name = "QUERY", help = "the query")]
        query: Query,
        #[structopt(long = "obsoleted", help = "Prints obsoleted memo titles")]
        obsoleted: bool,
    },
    #[structopt(name = "server", about = "Runs server")]
    Server,
    #[structopt(name = "title", about = "Print the title of the memo")]
    Title {
        #[structopt(name = "ID_LIKE", help = "the id of the memo")]
        id_like: String,
    },
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let data_dir = env::current_dir()?;
    let app = App::new(data_dir);
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::Create { title } => command::create(app, title.as_deref()),
        Subcommand::Edit { id_like_or_title } => command::edit(app, id_like_or_title.as_str()),
        Subcommand::EnsureLinks { id_like } => command::ensure_links(app, id_like),
        Subcommand::Link { id_like_or_title } => command::link(id_like_or_title.as_str()),
        Subcommand::List { obsoleted } => command::list(app, obsoleted),
        Subcommand::ListTitle { obsoleted } => command::list_title(app, obsoleted),
        Subcommand::Search { obsoleted, query } => command::search(app, query, obsoleted),
        Subcommand::Server => command::server(app).await,
        Subcommand::Title { id_like } => command::title(app, id_like.as_str()),
    }
}
