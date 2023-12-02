use std::fs;
use std::path::Path;

use me::{WebPage, WebPageFile};

/// Represents the index for the website.
/// 
/// Writes output to file `index.html`.
pub struct SiteIndex {
   pages: Vec<String>, 
}

// Creates an index.html file in the site path
fn generate_index(pages: &Vec<WebPageFile>, dest_dir: &Path) -> Result<SiteIndex, &'static str> {
    println!("Generating site index...");
    let mut index = String::new();
    let mut index_pages: Vec<String> = Vec::new();

    index.push_str("<h2>Site Index</h2><ul>");

    for web_page in pages {
        index.push_str("<li><a href='");
        index.push_str(web_page.file_path.with_extension("html").file_name().unwrap().to_str().unwrap());
        index.push_str("'>");
        index.push_str(web_page.file_path.file_stem().unwrap().to_str().unwrap());
        index.push_str("</a></li>");

        index_pages.push(String::from(web_page.file_path.file_stem().unwrap().to_str().unwrap()));
    }

    index.push_str("</ul>");

    // Build our index WebPage
    WebPage::from_string(String::from("index"), index).build(&dest_dir).unwrap();

    Ok(SiteIndex {
        pages: index_pages,
    })
}

// This program will generate html pages for the website.
fn main() {
    println!("Building andii land...\n");

    // Flow
    // Go through all of the files in pages/
    // .htm -> load contents -> paste s");
    let pages_path = Path::new("./pages/");

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

    files.sort_by(|a, b| a.file_path.cmp(&b.file_path));
    let site_dir = Path::new("../site/");
    let _ = generate_index(&files, &site_dir);

    // Convert the files into pages
    let mut pages: Vec<WebPage> = files.into_iter()
        .map(|file| WebPage::from_web_page_file(file).unwrap()) .collect(); pages.sort();
    println!("Generating site pages...");
    // Build all the pages of the site
    for web_page in pages {
        web_page.build(&site_dir).unwrap();
    }
}
