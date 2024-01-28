use std::error::Error;
use std::path::PathBuf;

use clap::{arg, value_parser, Arg, ArgAction, ArgMatches, Command};

pub fn get_subcommand() -> Command {
    Command::new("stats").about("CSV statistics").arg_required_else_help(true).subcommand(
        Command::new("sum")
            .about("Sum of columns")
            .arg(
                arg!(-d --dada "foo")
                    .required(false)
                    .value_parser(value_parser!(PathBuf))
                    .num_args(1)
                    .action(ArgAction::Set),
            )
            .arg(Arg::new("file").required(true)),
    )
}

pub fn stats_sum(matches: &ArgMatches) -> Result<bool, Box<dyn Error>> {
    Ok(true)
}

pub fn execute(matches: &ArgMatches) -> Result<bool, Box<dyn Error>> {
    match matches.subcommand() {
        Some(("sum", sub_matches)) => stats_sum(sub_matches),
        _ => Err("The stats subcommand requires arguments".into()),
    }
}
