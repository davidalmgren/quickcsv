use std::error::Error;
use std::path::PathBuf;

use clap::{arg, value_parser, ArgAction, ArgMatches, Command};

pub fn get_subcommand() -> Command {
    Command::new("stats").about("CSV statistics").subcommand(
        Command::new("sum").about("Sum of columns").arg(
            arg!(-f --file "CSV file")
                .required(false)
                .value_parser(value_parser!(PathBuf))
                .num_args(1)
                .action(ArgAction::Set),
        ),
    )
}

pub fn execute(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    Ok(())
}