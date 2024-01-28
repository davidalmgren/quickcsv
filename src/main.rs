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
    let matches = command!()
        .arg_required_else_help(true)
        .subcommand(sort::get_subcommand())
        .subcommand(merge::get_subcommand())
        .subcommand(stats::get_subcommand())
        .get_matches();

    let result = match matches.subcommand() {
        Some(("sort", sub_matches)) => sort::execute(sub_matches),
        Some(("merge", sub_matches)) => merge::execute(sub_matches),
        Some(("stats", sub_matches)) => stats::execute(sub_matches),
        _ => Ok(false),
    };

    match result {
        Ok(success) => {
            if success == true {
                process::exit(0);
            } else {
                process::exit(1);
            };
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}
