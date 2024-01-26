use std::error::Error;
use std::path::PathBuf;

use clap::{arg, value_parser, ArgAction, ArgMatches, Command};

pub fn get_subcommand() -> Command {
    Command::new("merge").about("Merge one or more CSV files").arg(
        arg!(-f --file "CSV file")
            .required(true)
            .value_parser(value_parser!(PathBuf))
            .num_args(1..10)
            .action(ArgAction::Append),
    )
}

pub fn execute(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    merge_and_print(matches.get_many::<PathBuf>("file").unwrap_or_default().collect());
    Ok(())
}

fn merge_and_print(file_paths: Vec<&PathBuf>) {
    let mut csv_file = crate::utils::csv::CSVFile::new();
    if let Err(err) = csv_file.read_file(file_paths[0]) {
        eprintln!("Failed to read file: {:?}", err);
        return;
    }

    for file_path in file_paths.iter().skip(1).collect::<Vec<_>>() {
        let mut ext_csv_file = crate::utils::csv::CSVFile::new();
        if let Err(err) = ext_csv_file.read_file(file_path) {
            eprintln!("Failed to read file: {:?}", err);
            return;
        }

        if let Err(err) = csv_file.merge(ext_csv_file) {
            eprintln!("Failed to merge files: {:?}", err);
            return;
        }
    }

    if let Err(err) = csv_file.print() {
        eprintln!("Displaying output failed {:?}", err);
    }
}
