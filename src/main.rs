mod command;
mod entity;
mod handler;
mod handler_helpers;
mod helpers;
mod template;
mod url_helpers;
mod use_case;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("memo")
        .subcommand(
            clap::SubCommand::with_name("edit")
                .about("Creates a new memo that obsoletes the specified memo")
                .arg(
                    clap::Arg::with_name("IDLike")
                        .help("the id of the memo to edit")
                        .required(true),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("link")
                .about("Shows a link for memo")
                .arg(
                    clap::Arg::with_name("ID_LIKE_OR_TITLE")
                        .help("the id or title of the memo")
                        .required(true),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("list")
                .about("Lists memos")
                .arg(
                    clap::Arg::with_name("obsoleted")
                        .long("obsoleted")
                        .help("Prints obsoleted memos"),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("list-title")
                .about("Lists memo titles")
                .arg(
                    clap::Arg::with_name("obsoleted")
                        .long("obsoleted")
                        .help("Prints obsoleted memo titles"),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("new")
                .about("Creates a new memo")
                .arg(
                    clap::Arg::with_name("title")
                        .long("title")
                        .value_name("TITLE")
                        .help("Creates a new memo with the specified title"),
                ),
        )
        .subcommand(clap::SubCommand::with_name("server").about("Runs server"))
        .get_matches();
    match matches.subcommand() {
        ("new", Some(sub_matches)) => {
            let title = sub_matches.value_of("title");
            crate::command::new::new(title)?
        }
        ("edit", Some(sub_matches)) => {
            let id_like_string = sub_matches.value_of("IDLike").expect("IDLike required");
            crate::command::edit::edit(id_like_string)?
        }
        ("link", Some(sub_matches)) => {
            let id_like_or_title_string = sub_matches
                .value_of("ID_LIKE_OR_TITLE")
                .expect("ID or TITLE required");
            crate::command::link::link(id_like_or_title_string)?
        }
        ("list", Some(sub_matches)) => {
            let obsoleted = sub_matches.is_present("obsoleted");
            crate::command::list::list(obsoleted)?
        }
        ("list-title", Some(sub_matches)) => {
            let obsoleted = sub_matches.is_present("obsoleted");
            crate::command::list_title::list_title(obsoleted)?
        }
        ("server", _) => crate::command::server::server()?,
        _ => {}
    }
    Ok(())
}
