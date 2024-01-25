use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;

mod utils {
    pub mod csv_utils;
}

fn sort_and_print(file_path: Option<&PathBuf>, column: &str, order: &str, method: &str) {
    let descending: bool = if order == "descending" { true } else { false };
    let numerical: bool = if method == "numerical" { true } else { false };

    let mut csv_file = utils::csv_utils::CSVFile::new();

    if let Some(file_path) = file_path {
        if let Err(err) = csv_file.read_file(file_path) {
            eprintln!("Failed to read file: {:?}", err);
            return;
        }
    } else {
        if let Err(err) = csv_file.read_stdin() {
            eprintln!("Failed to read from STDIN: {:?}", err);
            return;
        }
    }

    if let Err(err) = csv_file.sort_by_column(column, descending, numerical) {
        eprintln!("Failed to sort: {:?}", err);
        return;
    }

    if let Err(err) = csv_file.print() {
        eprintln!("Displaying output failed {:?}", err);
    }
}

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("sort")
                .about("Sort CSV file by column key")
                .arg(
                    arg!(-f --file "CSV file")
                        .required(false)
                        .value_parser(value_parser!(PathBuf))
                        .num_args(1)
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-c --key "Column key")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .num_args(1)
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-o --order "Sort order")
                        .value_parser(["ascending", "descending"])
                        .action(ArgAction::Set)
                        .num_args(1)
                        .default_value("descending"),
                )
                .arg(
                    arg!(-m --method "Sort method")
                        .value_parser(["numerical", "alphabetical"])
                        .action(ArgAction::Set)
                        .num_args(1)
                        .default_value("numerical"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("sort") {
        let file_path = match matches.try_get_one::<PathBuf>("file") {
            Ok(v) => v,
            Err(_) => None,
        };

        sort_and_print(
            file_path,
            matches.get_one::<String>("key").unwrap(),
            matches.get_one::<String>("order").unwrap(),
            matches.get_one::<String>("method").unwrap(),
        );
    }
}
