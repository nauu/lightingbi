use csv as csv_loader;
use csv::{Reader, ReaderBuilder};
use std::error::Error;
use std::fs::File;
use std::io;

struct Csv_Connector {
    reader: ReaderBuilder,
}

impl Csv_Connector {
    fn new() -> Self {
        Self {
            reader: ReaderBuilder::new(),
        }
    }

    fn load_file(&self, file_path: &str) -> Result<Reader<File>, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let rdr = self.reader.from_reader(file);
        Ok(rdr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_file() {
        let file_path =
            "/Users/nauu/CLionProjects/lightingbi/components/connectors/csv/resource/user_result.csv";
        let mut rdr = Csv_Connector::new().load_file(file_path);
        for record in rdr.unwrap().records() {
            println!("{:?}", record);
        }
    }
}
