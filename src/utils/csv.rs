use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub struct CSVFile {
    header: Vec<String>,
    data: Vec<Vec<String>>,
}

impl CSVFile {
    pub fn new() -> Self {
        CSVFile { header: Vec::new(), data: Vec::new() }
    }

    fn read(&mut self, mut rdr: csv::Reader<impl std::io::Read>) -> Result<(), Box<dyn Error>> {
        self.header = rdr.headers()?.iter().map(ToString::to_string).collect();
        for result in rdr.records() {
            self.data.push(result?.iter().map(ToString::to_string).collect());
        }
        Ok(())
    }

    pub fn read_file(&mut self, file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;
        self.read(ReaderBuilder::new().has_headers(true).from_reader(file))
    }

    pub fn read_stdin(&mut self) -> Result<(), Box<dyn Error>> {
        self.read(ReaderBuilder::new().has_headers(true).from_reader(io::stdin()))
    }

    fn compare_columns(
        &self, c1: &str, c2: &str, descending: bool, numerical: bool,
    ) -> Result<bool, Box<dyn Error>> {
        if numerical {
            let c1_numerical = c1.parse::<f64>()?;
            let c2_numerical = c2.parse::<f64>()?;
            Ok((descending && c1_numerical > c2_numerical)
                || (!descending && c1_numerical < c2_numerical))
        } else {
            Ok((descending && c1 > c2) || (!descending && c1 < c2))
        }
    }

    pub fn sort_by_column(
        &mut self, key: &str, descending: bool, numerical: bool,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(cmp_col_idx) = self.header.iter().position(|s| s == key) {
            for i in 0..self.data.len() {
                for j in 0..self.data.len() - i - 1 {
                    if self.compare_columns(
                        &self.data[j + 1][cmp_col_idx],
                        &self.data[j][cmp_col_idx],
                        descending,
                        numerical,
                    )? {
                        let tmp = self.data[j][cmp_col_idx].clone();
                        self.data[j][cmp_col_idx] = self.data[j + 1][cmp_col_idx].clone();
                        self.data[j + 1][cmp_col_idx] = tmp;
                    }
                }
            }
            Ok(())
        } else {
            let error_message = format!("Column '{}' not found in header.", key);
            Err(error_message.into())
        }
    }

    pub fn print(&self) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_writer(io::stdout());

        wtr.write_record(&self.header)?;
        for row in &self.data {
            wtr.write_record(row)?;
        }
        wtr.flush()?;

        Ok(())
    }

    pub fn merge(&mut self, other: CSVFile) -> Result<(), Box<dyn Error>> {
        if self.header != other.header {
            let error_message = format!("Headers are not equal.");
            Err(error_message.into())
        } else {
            for row in other.data {
                self.data.push(row)
            }
            Ok(())
        }
    }
}
