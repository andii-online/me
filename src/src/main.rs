use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

const NAV: &str = "
<nav id='menu'>
<h1>
<a href='home.html'>andy land</a>
</h1>
<ul>
<li class='category'>
<h3>playground</h3>
<ul>

<li class='item'>
<a href='about.html'><img src='../content/about.png'> about</a>
</li>

<li class='item'>
<a href='projects.html'><img src='../content/projects.png'> projects</a>
</li>

<li class='item'>
<a href='contact.html'><img src='../content/contact.png'> contact</a>
</li>
</ul>
</li>

<li class='category'>
<h3>links 4 u</h3>
<ul>

<li class='item'>
<a href='https://radlynn.itch.io'><img src='../content/itch.png'> itch.io</a>
</li>

<li class='item'>
<a href='https://twitter.com/thickfatherandy'><img src='../content/twitter.png'> tweetsers</a>
</li>

</ul>
</li>
</ul>
</nav>
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
        let page_name = file.file_name(); 
        let mut path_to_new_file = Path::new("./../site/").join(&page_name);
        path_to_new_file.set_extension("html");
        
        let page_name = match page_name.to_str(){
            Some(val) => match val.split(".").next() {
                Some(s) => s,
                None => panic!("Filename has no name before extensions"),
            },
            None => panic!("OsStr cannot be converted to Str"),
        };

        build_file(&file.path(), &path_to_new_file, page_name);
    }
}

fn build_file(src_path: &PathBuf, dest_path: &PathBuf, name: &str) {
    let file_content = match fs::read_to_string(src_path) {
        Ok(val) => val,
        Err(e) => panic!("Error reading file: {}", e)
    };
    let mut content = String::new();
    content.push_str("<!DOCTYPE html><html lang='en'><head>");
    content.push_str(&format!("<title>andrew straus - {}</title>", name));
    content.push_str("<meta name='description' content='welcome to me, andrew straus. this is my resume, portfolio, and personal showcase space. please take a look around. this is me saying welcome from the etherial plane of existance.'> <link rel='apple-touch-icon' sizes='180x180' href='../icons/apple-touch-icon.png'> <link rel='icon' type='image/png' sizes='32x32' href='../icons/favicon-32x32.png'> <link rel='icon' type='image/png' sizes='16x16' href='../icons/favicon-16x16.png'> <link rel='manifest' href='../site.webmanifest'> <meta name='viewport' content='width=device-width, initial-scale=1.0'> <link href='../styles/style.css' rel='stylesheet'> <link rel='stylesheet' media='screen' href='https://fontlibrary.org/face/hanken' type='text/css'> </head> <body>");
    content.push_str(NAV);
    content.push_str("<main>\n");
    content.push_str(file_content.as_str());
    content.push_str("</main>");
    content.push_str("</body>");

    println!("Writing {} to {}", name, dest_path.display());

    // create a new file with that file name in ../site/
    fs::write(dest_path, content.into_bytes()).unwrap();
}
