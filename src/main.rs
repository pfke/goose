#[macro_use]
extern crate clap;
use clap::App;

extern crate slog;
extern crate slog_async;
extern crate slog_term;
use slog::*;

fn main() {
	let yml = load_yaml!("assets/cli.yml");
    let matches = App::from_yaml(yml).get_matches();

//    let matches = App::new("goos[seberry]")
//        .version("0.0.1")
//        .author("Heiko Blobner")
//        .about("edit, search and use a developer diary")
//        .arg(Arg::with_name("config")
//            .short("c")
//            .long("config")
//            .value_name("FILE")
//            .help("Sets a custom config file")
//            .takes_value(true)
//        )
//        .arg(Arg::with_name("v")
//            .short("v")
//            .multiple(true)
//            .help("Sets the level of verbosity")
//        )
//        .subcommand(SubCommand::with_name("add")
//            .about("add entry")
//        )
//        .subcommand(SubCommand::with_name("delete")
//            .about("delete entry")
//        )
//        .subcommand(SubCommand::with_name("list")
//            .about("list entries")
//            .version("0.0.1")
//        )
//        .get_matches();

    let _config = matches.value_of("config").unwrap_or("default.conf");

    let log_level = match matches.occurrences_of("verbose") {
        0 => slog::Level::Critical,
        1 => slog::Level::Error,
        2 => slog::Level::Warning,
        3 => slog::Level::Info,
        4 => slog::Level::Debug,
        5 | _ => slog::Level::Trace,
    };

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator)
        .use_original_order()
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let drain = slog::LevelFilter::new(drain, log_level).fuse();

    let logger = slog::Logger::root(drain, o!("version" => "0.0.1"));

    crit!(logger, "started");
    error!(logger, "started");
    warn!(logger, "started");
    info!(logger, "started");
    debug!(logger, "started");
    trace!(logger, "started");
}

