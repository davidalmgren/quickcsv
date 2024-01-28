use std::error::Error;
use std::path::PathBuf;

use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

pub fn get_subcommand() -> Command {
    Command::new("merge")
        .about("Merge one or more CSV files")
        .arg(Arg::new("file").value_parser(value_parser!(PathBuf)).action(ArgAction::Append))
}

pub fn execute(matches: &ArgMatches) -> Result<bool, Box<dyn Error>> {
    let mut primary_csv = crate::utils::csv::CSVFile::new();

    for file_path in matches.get_many::<PathBuf>("file").unwrap_or_default() {
        primary_csv.merge(crate::utils::csv::CSVFile::new().read_file(file_path)?);
    }

    primary_csv.print()?;

    Ok(true)
}
