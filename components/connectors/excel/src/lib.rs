use calamine::{open_workbook_auto, DataType, Range, Reader};
use chrono::{Duration, NaiveDate};
use connector_craits::FileConnector;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

struct Excel_Connector {}

impl FileConnector for Excel_Connector {
    type Result = Result<(Range<DataType>), Box<dyn Error>>;

    fn load_file(&self, file_path: &str) -> Self::Result {
        let sce = PathBuf::from(file_path);
        match sce.extension().and_then(|s| s.to_str()) {
            Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => (),
            _ => panic!("Expecting an excel file"),
        }

        let mut xl = open_workbook_auto(&sce).unwrap();
        let range = xl.worksheet_range_at(0).unwrap().unwrap();

        Ok(range)
    }
}

impl Excel_Connector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn from_days_since_1900(&self, days_since_1900: i64) -> NaiveDate {
        let d1900 = NaiveDate::from_ymd(1900, 1, 1);
        d1900 + Duration::days(days_since_1900 - 2)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_load_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/test.xlsx");

        let file_path = d.to_str().unwrap();
        println!("file_path:{} ", file_path);

        let excel_connector = Excel_Connector::new();
        let range = excel_connector.load_file(file_path).unwrap();

        for r in range.rows() {
            for (i, c) in r.iter().enumerate() {
                match *c {
                    DataType::Empty => print!("empty, "),
                    DataType::String(ref s) => print!("string: {}, ", s),
                    DataType::Int(ref i) => print!("int: {}, ", i),
                    DataType::Bool(ref b) => print!("bool: {}, ", b),
                    DataType::Float(ref f) => print!("float: {}, ", f),
                    DataType::DateTime(ref d) => {
                        let d = excel_connector.from_days_since_1900(*d as i64);
                        print!("date: {}, ", d);
                    }
                    DataType::Error(ref e) => print!("{:?}, ", e),
                };
            }
            println!();
        }
    }
}
