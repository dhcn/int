use chrono::{Local};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::ops::Add;

pub struct FileWriter {
    file_date: String,
    title: String,
    file: File,
}

impl FileWriter {
    pub fn new(title: String) -> Self {
        let file_date = Local::now().format("%Y-%m-%d").to_string();
        let file_name = title.clone().add(file_date.as_str());
        let result = OpenOptions::new().append(true).open(&file_name);
        let file = match result {
            Ok(file) => file,
            Err(_error) => std::fs::File::create(file_name).expect("create failed"),
        };

        return FileWriter {
            file_date: file_date,
            title: title,
            file: file,
        };
    }
    pub fn re_init(&mut self) {
        self.file_date = Local::now().format("%Y-%m-%d").to_string();
        let file_name = self.title.clone().add(self.file_date.as_str());
        let result = OpenOptions::new().append(true).open(file_name.clone());
        self.file = match result {
            Ok(file) => file,
            Err(_error) => std::fs::File::create(file_name).expect("create failed"),
        };
    }

    pub fn append_data(&mut self, data: &[u8]) {
        let file_date = Local::now().format("%Y-%m-%d").to_string();
        if self.file_date != file_date {
            self.re_init();
        }
        let _ = self.file.write_all(data).unwrap();
        let _ = self.file.write_all("\n".as_bytes()).unwrap();
    }
}
