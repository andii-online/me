#![recursion_limit = "512"]

use me_site_builder::{SITE_NAME, write_site};
use me_site_builder::index::get_site_index;
use me_site_builder::web_page::WebPage;
use me_site_builder::web_page_file::WebPageFile;
use std::fs;
use std::path::Path;

fn main() {
    println!("Building {}...\n", SITE_NAME);

    // Flow
    // Go through all of the files in pages/
    // .htm -> load contents -> paste s");
    let pages_path = Path::new("../pages/");
    let site_dir = Path::new("../site/");

    if !pages_path.exists() {
        panic!("No pages/ directory found!");
    }

    let src_dir = match fs::read_dir(pages_path) {
        Ok(val) => val,
        Err(e) => panic!("Error reading pages/: {}", e),
    };

    // Convert all the DirEntries in the src_directory into WebPageFiles
    let mut files: Vec<WebPageFile> = src_dir
        .map(|entry| WebPageFile::from_file(entry.unwrap()).unwrap())
        .collect();
    files.sort_by(|a, b| a.file_path.file_name().cmp(&b.file_path.file_name()));

    let index = get_site_index(&files);

    // Convert the files into pages
    let results: Result<Vec<WebPage>, String> = files
        .into_iter()
        .map(|file| WebPage::from_web_page_file(file, &index))
        .collect();
    match results {
        Ok(mut pages) => {
            pages.push(index.page);
            pages.sort();
            write_site(&pages, &site_dir);
        },
        Err(e) => eprintln!("Error while collecting pages:\n {}", e),
    }
}
