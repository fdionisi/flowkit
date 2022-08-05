use std::path::PathBuf;

use async_std::fs::File;

pub struct FileField {
    path: PathBuf,
    file: Option<File>,
}

impl FileField {
    pub fn get(&mut self) {
        todo!()
    }

    pub fn new_file(&mut self) {
        todo!()
    }

    pub fn put(&mut self) {
        todo!()
    }

    pub fn replace(&mut self) {
        todo!()
    }

    pub fn write(&mut self) {
        todo!()
    }

    pub fn writelines(&mut self) {
        todo!()
    }

    pub fn close(&mut self) {
        todo!()
    }
}
