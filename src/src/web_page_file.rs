use std::fs;
use std::fs::{DirEntry, File, Metadata};
use std::io::Read;
use std::path::PathBuf;

/// Represents a source file for website pages
pub struct WebPageFile {
    pub file_path: PathBuf, // idk about having this be public :/
    pub file: File,
    pub metadata: Metadata,
}

impl WebPageFile {
    pub fn from_file(dir_entry: DirEntry) -> Result<WebPageFile, &'static str> {
        let file_path = dir_entry.path();
        let file = File::open(&file_path).unwrap();
        let metadata = fs::metadata(&file_path).unwrap();

        Ok(WebPageFile {
            file_path,
            file,
            metadata,
        })
    }

    pub fn get_file_name(&self) -> &str {
        self.file_path.to_str().unwrap()
    }

    pub fn get_page_contents(&mut self) -> std::io::Result<String> {
        let mut file_contents = String::new();

        self.file
            .read_to_string(&mut file_contents)
            .expect(&format!(
                "Failed to read page {} contents.",
                self.get_file_name()
            ));

        Ok(file_contents)
    }
}
