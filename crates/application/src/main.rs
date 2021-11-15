mod adapter;
mod app;
mod command;
mod handler;
mod handler_helpers;
mod helpers;
mod template;
mod url_helpers;
mod use_case;

use std::env;

use app::App;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(
        name = "edit",
        about = "Creates a new memo that obsoletes the specified memo"
    )]
    Edit {
        #[structopt(name = "ID_LIKE", help = "the id of the memo to edit")]
        id_like: String,
    },
    #[structopt(name = "insert-links", about = "Inserts links into the memo")]
    InsertLinks {
        #[structopt(name = "ID_LIKE", help = "the id of the memo to edit")]
        id_like: String,
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
    #[structopt(name = "new", about = "Creates a new memo")]
    New {
        #[structopt(
            long = "title",
            name = "TITLE",
            help = "Creates a new memo with the specified title"
        )]
        title: Option<String>,
    },
    #[structopt(name = "server", about = "Runs server")]
    Server,
    #[structopt(name = "title", about = "Print the title of the memo")]
    Title {
        #[structopt(name = "ID_LIKE", help = "the id of the memo")]
        id_like: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = env::current_dir()?;
    let app = App::new(data_dir);
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::Edit { id_like } => crate::command::edit(app, id_like.as_str())?,
        Subcommand::InsertLinks { id_like } => crate::command::insert_links(app, id_like.as_str())?,
        Subcommand::Link { id_like_or_title } => crate::command::link(id_like_or_title.as_str())?,
        Subcommand::List { obsoleted } => crate::command::list(obsoleted)?,
        Subcommand::ListTitle { obsoleted } => crate::command::list_title(obsoleted)?,
        Subcommand::New { title } => crate::command::new(app, title.as_deref())?,
        Subcommand::Server => crate::command::server()?,
        Subcommand::Title { id_like } => crate::command::title(id_like.as_str())?,
    }
    Ok(())
}
