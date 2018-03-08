extern crate libc;
extern crate hc256;
extern crate clap;
extern crate bs58;
extern crate time;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

mod skein3fish;
mod ctr;
mod repo;
mod pbkdf2;

use std::{env, io, path, process};
use clap::{Arg, SubCommand};
use slog::Drain;

fn create_logger(verbosity: u32, timing_verbosity: u32) -> slog::Logger {
    match (verbosity, timing_verbosity) {
        (0, 0) => slog::Logger::root(slog::Discard, o!()),
        (v, tv) => {
            let v = match v {
                0 => slog::Level::Warning,
                1 => slog::Level::Info,
                2 => slog::Level::Debug,
                _ => slog::Level::Trace,
            };
            let tv = match tv {
                0 => slog::Level::Warning,
                1 => slog::Level::Info,
                2 => slog::Level::Debug,
                _ => slog::Level::Trace,
            };
            let drain = slog_term::term_full();
            if verbosity > 4 {
                // at level 4, use synchronous logger so not to loose any
                // logging messages
                let drain = std::sync::Mutex::new(drain);
                let drain =
                    slog::Filter::new(drain, move |record: &slog::Record| {
                        if record.tag() == "slog_perf" {
                            record.level() >= tv
                        } else {
                            record.level() >= v
                        }
                    });
                let log = slog::Logger::root(drain.fuse(), o!());
                info!(
                    log,
                    "Using synchronized logging, that we'll be slightly slower."
                );
                log
            } else {
                let drain = slog_async::Async::default(drain.fuse());
                let drain =
                    slog::Filter::new(drain, move |record: &slog::Record| {
                        if record.tag() == "slog_perf" {
                            record.level().is_at_least(tv)
                        } else {
                            record.level().is_at_least(v)
                        }
                    });
                slog::Logger::root(drain.fuse(), o!())
            }
        }
    }
}

fn run() -> io::Result<()> {
    let mut home_dir = match env::home_dir() {
        Some(path) => path.into_os_string().into_string().unwrap(),
        None => panic!("Unable to get your home directory!"),
    };
    home_dir.push(path::MAIN_SEPARATOR);
    home_dir += &String::from(".rsbup");
    home_dir.push(path::MAIN_SEPARATOR);

    let matches = clap::App::new("rsbup")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Trung Pham <i.am@tru.ng>")
        .about("Rust local/cloud backup program")
        .arg(Arg::with_name("VERBOSE").short("v").multiple(true).help("Increase debugging level for general messages"))
        .arg(Arg::with_name("VERBOSE_TIMINGS").short("t").multiple(true).help("Increase debugging level for timings"))
        .subcommand(SubCommand::with_name("init").display_order(0)
            .about("Create a new repository")
            .version(env!("CARGO_PKG_VERSION"))
            .arg(Arg::with_name("CHUNK_BITS").short("b").long("chunk-bits").takes_value(true).value_name("N")
                .default_value("23").help("Set number of chunk bits"))
            .arg(Arg::with_name("COMPRESSION_LEVEL").short("l").long("comp-level").takes_value(true).value_name("N")
                .default_value("3").help("Set zstd compression level"))
            .arg(Arg::with_name("REPO_NAME").short("n").long("repo-name").takes_value(true).value_name("NAME")
                .help("Set name of the new repository"))
            .arg(Arg::with_name("DESTINATION").short("d").long("dest").takes_value(true).value_name("PATH")
                .help("Set backup destination"))
        	.arg(Arg::with_name("RSBUP_CONFIG_DIR").short("c").long("config-dir").takes_value(true).value_name("PATH")
        		.default_value(&home_dir)
             	.help("Set path to rsbup config directory")))
        .subcommand(SubCommand::with_name("backup").display_order(0)
            .about("Backup a directory")
            .version(env!("CARGO_PKG_VERSION"))
            .arg(Arg::with_name("SRC_DIR").short("s").long("src-dir").takes_value(true).value_name("PATH")
                .help("Path to source directory to backup"))
            .arg(Arg::with_name("REPO_NAME").short("n").long("repo-name").takes_value(true).value_name("NAME")
                .help("Choose an existing repository to backup"))
            .arg(Arg::with_name("RSBUP_CONFIG_DIR").short("c").long("config-dir").takes_value(true).value_name("PATH")
        		.default_value(&home_dir)
             	.help("Set path to rsbup config directory")))
        .subcommand(SubCommand::with_name("restore").display_order(0)
            .about("Restore from a repository")
            .version(env!("CARGO_PKG_VERSION"))
            .arg(Arg::with_name("RESTORE_DIR").short("r").long("restore-dir").takes_value(true).value_name("PATH")
                .help("Path to a directory to restore"))
            .arg(Arg::with_name("INCLUDE").short("i").long("include").takes_value(true).value_name("PATTERN")
                .help("Include files/directories"))
            .arg(Arg::with_name("EXCLUDE").short("e").long("exclude").takes_value(true).value_name("PATTERN")
                .help("Exclude files/directories"))
            .arg(Arg::with_name("REPO_NAME").short("n").long("repo-name").takes_value(true).value_name("NAME")
                .help("Choose an existing repository to restore from"))
            .arg(Arg::with_name("REVISION").short("r").long("revision").takes_value(true).value_name("N")
                .help("Choose an existing revision to restore from"))
            .arg(Arg::with_name("RSBUP_CONFIG_DIR").short("c").long("config-dir").takes_value(true).value_name("PATH")
        		.default_value(&home_dir)
             	.help("Set path to rsbup config directory")))
        .get_matches();

    let log = create_logger(
        matches.occurrences_of("VERBOSE") as u32,
        matches.occurrences_of("VERBOSE_TIMINGS") as u32,
    );

    match matches.subcommand() {
        ("init", Some(matches)) => {
        }
        ("backup", Some(matches)) => {
        }
        ("restore", Some(matches)) => {
        }
        _ => panic!("Unrecognized subcommand"),
    }

    Ok(())
}


fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(-1);
    }
}
