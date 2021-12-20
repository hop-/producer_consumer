use std::fs::File;
use std::io::{BufWriter, Write};

pub struct DataFile {
    file_name: String,
    file: Option<BufWriter<File>>,
}

impl DataFile {
    pub fn new(file_name: &str) -> DataFile {
        DataFile {
            file_name: file_name.to_string(),
            file: None,
        }
    }

    pub fn open(&mut self) {
        let f = File::create(self.file_name.clone()).unwrap();
        self.file = Some(BufWriter::new(f));
    }

    pub fn write(&mut self, data: u32) {
        self.file.as_mut().unwrap().write_all(format!("{},", data).as_bytes()).unwrap();
    }

    pub fn flush(&mut self) {
        self.file.as_mut().unwrap().flush().unwrap();
    }
}