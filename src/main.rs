use clap::command;
use cmd::{merge, sort, stats};
use std::process;

mod cmd {
    pub mod merge;
    pub mod sort;
    pub mod stats;
}

mod utils {
    pub mod csv;
}

fn main() {
    let cmd = command!()
    .subcommand(sort::get_subcommand())
    .subcommand(merge::get_subcommand())
    .subcommand(stats::get_subcommand());

    let matches = cmd.get_matches();

    let result = match matches.subcommand() {
        Some(("sort", sub_matches)) => sort::execute(sub_matches),
        Some(("merge", sub_matches)) => merge::execute(sub_matches),
        Some(("stats", sub_matches)) => stats::execute(sub_matches),
        _ => {
            cmd.print_help();
            Err("No argument provided, not sure how you ended up here...".into())
        },
    };

    if result.is_ok() {
        process::exit(0);
    } else {
        eprintln!("Error: {}", result.err().unwrap());
        process::exit(1);
    };
}
