pub trait FileConnector {
    type Result;

    fn load_file(&self, file_path: &str) -> Self::Result;
}
