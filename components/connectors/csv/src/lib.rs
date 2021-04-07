use connector_craits::FileConnector;
use csv as csv_loader;
use csv::{Reader, ReaderBuilder};
use std::error::Error;
use std::fs::File;
use std::io;

struct Csv_Connector {
    reader: ReaderBuilder,
}

impl FileConnector for Csv_Connector {
    type Result = Result<Reader<File>, Box<dyn Error>>;

    fn load_file(&self, file_path: &str) -> Self::Result {
        let file = File::open(file_path)?;
        let rdr = self.reader.from_reader(file);
        Ok(rdr)
    }
}

impl Csv_Connector {
    fn new() -> Self {
        Self {
            reader: ReaderBuilder::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/user_result.csv");

        let file_path = d.to_str().unwrap();
        println!("file_path:{} ", file_path);

        let mut rdr = Csv_Connector::new().load_file(file_path);
        for record in rdr.unwrap().records() {
            println!("{:?}", record);
        }
    }
}
