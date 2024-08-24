#![recursion_limit = "512"]

pub mod web_page;

use std::fs;
use std::path::{Path, PathBuf};
use web_page::WebPage;

pub const SITE_NAME: &str = "chloe land";

pub fn write_site(pages: &Vec<WebPage>, site_dir: &Path) {
    let tmp_dir = Path::new(".tmp/");
    let moved_files = match move_files_to_tmp(site_dir, tmp_dir) {
        Ok(val) => Some(val),
        Err(e) => {
            eprintln!("Error while collecting old site: {}", e);
            None 
        },
    };
    println!("Generating site pages...");
    for web_page in pages {
        match write_web_page(&web_page, &site_dir) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("While building site, page {} failed on error: {}", web_page.name, e);
                delete_all_files_in_dir(site_dir).expect("Unable to clean dir while restoring.");
                match &moved_files {
                    Some(files) => {
                        restore_files(files, site_dir, tmp_dir).expect("Failed to restore files.");
                        println!("Old site restored.")
                    },
                    None => println!("Nothing to restore..."),
                };
            }
        };
    }
    fs::remove_dir_all(tmp_dir).unwrap();
}

// Consumes the Webpage and writes it to a file.
fn write_web_page(web_page: &WebPage, dest: &Path) -> std::io::Result<()> {
    let built_page = web_page.build();
    println!(
        "Writing {} to {}",
        &web_page.name,
        dest.join(&web_page.name).display()
    );

    fs::write(
        dest.join(format!("{}.html", &web_page.name)),
        built_page.into_bytes(),
    )
}

fn move_files_to_tmp(src_dir: &Path, tmp_dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut moved_files = Vec::new();

    if !tmp_dir.exists() {
        match fs::create_dir(tmp_dir) {
            Err(e) => {
                eprintln!("Couldn't create tmp dir at: {} beacause error: {}", tmp_dir.to_str().unwrap(), e)
            },
            _ => (),
        };
    }

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let src_path = entry.path();

        // Skip directories and the tmp directory itself
        if src_path.is_dir() || src_path == tmp_dir {
            continue;
        }

        let file_name = entry.file_name();
        let tmp_path = tmp_dir.join(&file_name);
        fs::rename(&src_path, &tmp_path)?;

        moved_files.push(file_name.into());
    }

    Ok(moved_files)
}

fn restore_files(moved_files: &Vec<PathBuf>, src_dir: &Path, tmp_dir: &Path) -> std::io::Result<()> {
    for file_name in moved_files {
        let tmp_path = tmp_dir.join(&file_name);
        let src_path = src_dir.join(&file_name);
        fs::rename(&tmp_path, &src_path)?;
    }

    Ok(())
}

fn delete_all_files_in_dir(dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}
