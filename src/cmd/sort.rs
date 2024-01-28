use std::error::Error;
use std::path::PathBuf;

use clap::{arg, value_parser, ArgAction, ArgMatches, Command};

fn parse_order(order: &str) -> Result<crate::utils::csv::CSVSortOrder, Box<dyn Error>> {
    if order == "descending" {
        Ok(crate::utils::csv::CSVSortOrder::Descending)
    } else if order == "ascending" {
        Ok(crate::utils::csv::CSVSortOrder::Ascending)
    } else {
        let err = format!("Invalid argument {}", order);
        Err(err.into())
    }
}

fn parse_method(method: &str) -> Result<crate::utils::csv::CSVSortMethod, Box<dyn Error>> {
    if method == "numerical" {
        Ok(crate::utils::csv::CSVSortMethod::Numerical)
    } else if method == "alphabetical" {
        Ok(crate::utils::csv::CSVSortMethod::Alphabetical)
    } else {
        let err = format!("Invalid parse method {}", method);
        Err(err.into())
    }
}

pub fn get_subcommand() -> Command {
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
        )
}

pub fn execute(matches: &ArgMatches) -> Result<bool, Box<dyn Error>> {
    let key = matches.get_one::<String>("key").unwrap();
    let order = parse_order(matches.get_one::<String>("order").unwrap())?;
    let method = parse_method(matches.get_one::<String>("method").unwrap())?;

    let mut primary_csv: crate::utils::csv::CSVFile = match matches.try_get_one::<PathBuf>("file").ok() {
        Some(file_path) => crate::utils::csv::CSVFile::new().read_file(file_path.unwrap())?,
        None => crate::utils::csv::CSVFile::new().read_stdin()?
    };

    primary_csv.sort_by_column(key, order, method)?;

    primary_csv.print()?;

    Ok(true)
}