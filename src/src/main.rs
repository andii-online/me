use std::fs;
use std::io::prelude::*;
use std::path::Path;

const HEADER: &str = "<!DOCTYPE html>
<html lang='en'>
<head>
  <title>andrew straus - home</title>
  <meta name='description' content='welcome to me, andrew straus. this is my resume, portfolio, and personal showcase space. please take a look around. this is me saying welcome from the etherial plane of existance.'> 
  <link rel='apple-touch-icon' sizes='180x180' href='icons/apple-touch-icon.png'>
  <link rel='icon' type='image/png' sizes='32x32' href='icons/favicon-32x32.png'> 
  <link rel='icon' type='image/png' sizes='16x16' href='icons/favicon-16x16.png'>
  <link rel='manifest' href='/site.webmanifest'>
  <meta name='viewport' content='width=device-width, initial-scale=1.0'>
  <link href='styles/style.css' rel='stylesheet'>
  <link rel='stylesheet' media='screen' href='https://fontlibrary.org/face/hanken' type='text/css'>
</head>
<body>
<header>
<h1>
<a href='/'>andrew straus</a>
</h1>
<ul class='nav'>
<a href='site/home.html'>about</a>
<a href='site/projects.html'>projects</a>
</ul>
</header>
";

// This program will generate html pages for the website.
fn main() {
    // Flow
    // Go through all of the files in pages/
    // .htm -> load contents -> paste s");
    let pages_path = Path::new("./pages/");

    if !pages_path.exists() {
        panic!("No pages/ directory found!");
    }

    let result = match fs::read_dir(pages_path) {
        Ok(val) => val,
        Err(e) => panic!("Error reading pages/: {}", e),
    };
   
    for file in result {
        let file = file.unwrap();
        
        let file_content = match fs::read_to_string(file.path()) {
            Ok(val) => val,
            Err(e) => panic!("Error reading file: {}", e)
        };

        let mut content = HEADER.to_owned();
        content.push_str("<main>\n");
        content.push_str(file_content.as_str());
        content.push_str("</main>");

        // Create path to the new file
        let mut path_to_new_file = Path::new("./../site/").join(file.file_name());
        path_to_new_file.set_extension("html");

        println!("{}", path_to_new_file.display());

        // create a new file with that file name in ../site/
        fs::write(path_to_new_file, content.into_bytes()).unwrap();
    }
}
