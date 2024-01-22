use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;

mod utils {
    pub mod csv_utils;
}

fn sort_and_print(file_paths: Vec<&PathBuf>, column: &str, order: &str, method: &str) {
    let descending: bool = if order == "descending" { true } else { false };
    let numerical: bool = if method == "numerical" { true } else { false };

    for path in file_paths {
        let mut csv_file = utils::csv_utils::CSVFile::new();

        if let Err(err) = csv_file.read_file(path) {
            eprintln!("Failed to read file: {:?}", err);
            continue;
        }

        if let Err(err) = csv_file.sort_by_column(column, descending, numerical) {
            eprintln!("Failed to sort: {:?}", err);
            continue;
        }

        if let Err(err) = csv_file.print() {
            eprintln!("Displaying output failed {:?}", err);
        }
    }
}

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("sort")
                .about("Sort CSV file by column key")
                .arg(
                    arg!(-f --file "CSV file")
                        .required(true)
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
        let file_paths: Vec<&PathBuf> =
            matches.get_many::<PathBuf>("file").unwrap_or_default().collect();
        let key: &str = matches.get_one::<String>("key").unwrap();
        let order: &str = matches.get_one::<String>("order").unwrap();
        let method: &str = matches.get_one::<String>("method").unwrap();

        sort_and_print(file_paths, key, order, method);
    }
}
