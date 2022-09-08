use std::fs;
use std::path::{Path, PathBuf};

//TODO: move this to its own file
const NAV: &str = "
<nav id='menu'>
<div class='home'><a href='home.html'><img src='../content/home.png'></a></div>
<ul>
<li class='category'>
<h3>playground</h3>
<ul>

<li class='item'>
<a href='about.html'><img src='../content/about.png'>about</a>
</li>

<li class='item'>
<a href='thoughts.html'><img src='../content/thoughts.png'>thoughts</a>
</li>

<li class='item'>
<a href='projects.html'><img src='../content/projects.png'>projects</a>
</li>

<li class='item'>
<a href='contact.html'><img src='../content/contact.png'>contact</a>
</li>

</ul>
</li>

<li class='category'>
<h3>external links</h3>
<ul>

<li class='item'>
<a href='https://radlynn.itch.io'><img src='../content/itch.png'>itch.io</a>
</li>

<li class='item'>
<a href='https://twitter.com/andii-online'><img src='../content/twitter.png'>tweetsers</a>
</li>

</ul>
</li>

<li class='category'>
<h3>meta</h3>
<ul>
<li class='item'>
<a href='index.html'><img src='../content/index.png'>index</a>
</li>
</ul>
</li>

</ul>
</nav>
";

const FOOTER: &str = "
<footer>
<p>
<a href='home.html'>andii land</a>
 © 2022
<a href ='https://creativecommons.org/licenses/by-nc-sa/4.0/'> by-nc-sa 4.0</a>
<a href ='https://github.com/andrewstraus99/me'> *website src</a>
</p>
</footer>
";

// generate /site/index.html
// list of links to pages
// NAME.htm -> <a href=/site/NAME.html>NAME</a>

// Represents a page on the website
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Webpage {
    filepath: PathBuf,
    content: String,
}

impl Webpage {
    // Constructs a new Webpage from an .htm source
    pub fn from_src(src_path: PathBuf) -> Webpage {
        assert_eq!(
            "htm",
            src_path.extension().unwrap(),
            "Webpage cannot be constructed from a non-htm source."
        );

        let file_content = match fs::read_to_string(&src_path) {
            Ok(val) => val,
            Err(e) => panic!("error reading file: {}", e),
        };

        let mut path = String::from(src_path.file_stem().unwrap().to_str().unwrap());
        path.push_str(".html");

        Webpage {
            filepath: PathBuf::from(path),
            content: file_content,
        }
    }

    // Converts a String into a Webpage
    pub fn from_string(filename: String, content: String) -> Webpage {
        Webpage {
            filepath: PathBuf::from(filename),
            content: content,
        }
    }

    // Consumes the Webpage and converts its content into the built version of the webpage
    // containing html header, navigation, and other page features.
    fn build(self, dest_dir: &Path) {
        let mut content = String::new();

        // add html header to top of the file
        content.push_str("<!DOCTYPE html><html lang='en'><head>");
        content.push_str(&format!("<title>andii land - {}</title>", self.name()));
        content.push_str("<meta name='description' content='welcome to andii land! this is a very personal website where showcase the many interesting facets of my life!'> <link rel='apple-touch-icon' sizes='180x180' href='../icons/apple-touch-icon.png'> <link rel='icon' type='image/png' sizes='32x32' href='../icons/favicon-32x32.png'> <link rel='icon' type='image/png' sizes='16x16' href='../icons/favicon-16x16.png'> <link rel='manifest' href='../site.webmanifest'> <meta name='viewport' content='width=device-width, initial-scale=1.0'> <link href='../styles/style.css' rel='stylesheet'> <link rel='stylesheet' media='screen' href='https://fontlibrary.org/face/hanken' type='text/css'> </head> <body>");
        // navigation bar
        content.push_str(NAV);

        // page content
        content.push_str("<main>\n");
        content.push_str(&self.content);
        content.push_str("</main>");
        content.push_str("</body>");
        content.push_str(FOOTER);

        println!(
            "Writing {} to {}",
            self.name(),
            dest_dir.join(&self.filepath).display()
        );

        // create a new file with that file name in ../site/
        fs::write(dest_dir.join(&self.filepath), content.into_bytes()).unwrap();
    }

    fn name(&self) -> &str {
        self.filepath.file_stem().unwrap().to_str().unwrap()
    }
}

// Creates an index.html file in the site path
fn generate_index(pages: &Vec<Webpage>, dest_dir: &Path) {
    println!("Generating site index...");
    let mut index = String::new();
    index.push_str("<h2>Site Index</h2><ul>");

    for webpage in pages {
        index.push_str("<li><a href='");
        index.push_str(webpage.filepath.to_str().unwrap());
        index.push_str("'>");
        index.push_str(webpage.filepath.file_stem().unwrap().to_str().unwrap());
        index.push_str("</a></li>");
    }

    index.push_str("</ul>");

    Webpage::from_string(String::from("index.html"), index).build(&dest_dir);
}

// This program will generate html pages for the website.
fn main() {
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

    // Convert all the files in the src_directory into Webpage objects
    let mut pages: Vec<Webpage> = src_dir
        .map(|file| Webpage::from_src(file.unwrap().path()))
        .collect();
    pages.sort();

    let site_dir = Path::new("../site/");

    // Build the index page where all webpages are accessible
    generate_index(&pages, &site_dir);

    println!("Generating site pages...");
    // Build all the pages of the site
    for webpage in pages {
        webpage.build(&site_dir);
    }
}
