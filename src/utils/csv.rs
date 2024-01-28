use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(PartialEq,Copy, Clone)]
pub enum CSVSortOrder {
    Descending,
    Ascending,
}

#[derive(PartialEq,Copy, Clone)]
pub enum CSVSortMethod {
    Numerical,
    Alphabetical,
}

pub struct CSVFile {
    header: Vec<String>,
    data: Vec<Vec<String>>,
}

impl CSVFile {
    pub fn new() -> Self {
        CSVFile { header: Vec::new(), data: Vec::new() }
    }

    fn read(mut self, mut rdr: csv::Reader<impl std::io::Read>) -> Result<Self, Box<dyn Error>> {
        self.header = rdr.headers()?.iter().map(ToString::to_string).collect();
        for result in rdr.records() {
            self.data.push(result?.iter().map(ToString::to_string).collect());
        }
        Ok(self)
    }

    pub fn read_file(self, file_path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        self.read(ReaderBuilder::new().has_headers(true).from_reader(file))
    }

    pub fn read_stdin(self) -> Result<Self, Box<dyn Error>> {
        self.read(ReaderBuilder::new().has_headers(true).from_reader(io::stdin()))
    }

    fn compare_columns(
        &self, c1: &str, c2: &str, sort_order: CSVSortOrder, sort_method: CSVSortMethod,
    ) -> Result<bool, Box<dyn Error>> {
        match sort_method {
            CSVSortMethod::Numerical => {
                let c1_n = c1.parse::<f64>()?;
                let c2_n = c2.parse::<f64>()?;
                Ok(match sort_order {
                    CSVSortOrder::Descending => c1_n > c2_n,
                    CSVSortOrder::Ascending => c1_n < c2_n,
                })
            }
            CSVSortMethod::Alphabetical => {
                Ok(match sort_order {
                    CSVSortOrder::Descending => c1 > c2,
                    CSVSortOrder::Ascending => c1 < c2,
                })
            }
        }
    }

    pub fn sort_by_column(
        &mut self, key: &str, sort_order: CSVSortOrder, sort_method: CSVSortMethod,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(cmp_col_idx) = self.header.iter().position(|s| s == key) {
            for i in 0..self.data.len() {
                for j in 0..self.data.len() - i - 1 {
                    if self.compare_columns(
                        &self.data[j + 1][cmp_col_idx],
                        &self.data[j][cmp_col_idx],
                        sort_order,
                        sort_method,
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
