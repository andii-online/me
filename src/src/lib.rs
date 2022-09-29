use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::fs;
use std::fs::{DirEntry, File};

pub const SITE_NAME: &str = "andii land";

/// Represents a source file for website pages
pub struct WebPageFile {
    pub file_path: PathBuf, // idk about having this be public :/
    file: File,
}

impl WebPageFile {
    pub fn from_file(dir_entry: DirEntry) -> Result<WebPageFile, &'static str> {
        let file_path = dir_entry.path();
        let file = File::open(&file_path).unwrap();

        Ok(WebPageFile {
            file_path, 
            file,
        })
    }
}

/// Represents a page on the website
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WebPage {
    name: String,
    content: String,
}

impl WebPage {
    /// Constructs a new WebPage from an .htm source
    pub fn from_web_page_file(mut page_file: WebPageFile) -> Result<WebPage, &'static str> {
        //TODO: parse `file_contents` for links
        let mut file_contents = String::new();
        page_file.file.read_to_string(&mut file_contents).unwrap();

        let name = String::from(
            page_file.file_path.file_stem().unwrap().to_str().unwrap()
        );

        Ok(WebPage {
            name,
            content: file_contents,
        })
    }

    /// Converts a String into a WebPage
    pub fn from_string(name: String, content: String) -> WebPage {
        WebPage {
            name,
            content,
        }
    }

    /// Consumes the WebPage and converts its content into the built version of the WebPage
    /// containing html header, navigation, and other page features.
    pub fn build(self, dest_dir: &Path) -> Result<(), &'static str> {
        let mut content = String::new();
        content.push_str(&self.get_header());
        content.push_str("<body>\n");
        // navigation bar
        content.push_str(NAV);
        // page content
        content.push_str("<main>\n");
        content.push_str(&self.content);
        content.push_str("</main>\n");
        content.push_str("</body>\n");
        content.push_str(FOOTER);

        println!(
            "Writing {} to {}",
            self.name,
            dest_dir.join(&self.name).display()
        );

        // create a new file with that file name in ../site/
        fs::write(
            dest_dir.join(format!("{}.html", &self.name)), 
            content.into_bytes()
        ).unwrap();

        Ok(())
    }

    fn get_header(&self) -> String {
        let mut content = String::new();

        content.push_str("<!DOCTYPE html><html lang='en'>");
        content.push_str("<head>");
        content.push_str(&format!("<title>{} - {}</title>", SITE_NAME, self.name.clone()));
        // meta information could be moved elsewhere or standardized in some way...
        content.push_str(&format!("
<meta name='description' content='welcome to {}, my personal website!!'> 
<link rel='apple-touch-icon' sizes='180x180' href='../icons/apple-touch-icon.png'> 
<link rel='icon' type='image/png' sizes='32x32' href='../icons/favicon-32x32.png'> 
<link rel='icon' type='image/png' sizes='16x16' href='../icons/favicon-16x16.png'> 
<link rel='manifest' href='../site.webmanifest'> 
<meta name='viewport' content='width=device-width, initial-scale=1.0'> 
<link href='../styles/style.css' rel='stylesheet'> 
<link rel='stylesheet' media='screen' href='https://fontlibrary.org/face/hanken' type='text/css'> 
</head>\n", SITE_NAME));

        content
    }
}

pub const NAV: &str = "
<nav id='menu'>
<div class='home'><a href='home.html'><img src='../content/home.png' loading='lazy'></a></div>
<ul>
<li class='category'>
<h3>playground</h3>
<ul>

<li class='item'>
<a href='about.html'><img src='../content/about.png' loading='lazy'>about</a>
</li>

<li class='item'>
<a href='thoughts.html'><img src='../content/thoughts.png' loading='lazy'>thoughts</a>
</li>

<li class='item'>
<a href='projects.html'><img src='../content/projects.png' loading='lazy'>projects</a>
</li>

<li class='item'>
<a href='contact.html'><img src='../content/contact.png' loading='lazy'>contact</a>
</li>

</ul>
</li>

<li class='category'>
<h3>external links</h3>
<ul>

<li class='item'>
<a href='https://radlynn.itch.io'><img src='../content/itch.png' loading='lazy'>itch.io</a>
</li>

<li class='item'>
<a href='https://twitter.com/andii-online'><img src='../content/twitter.png' loading='lazy'>tweetsers</a>
</li>

</ul>
</li>

<li class='category'>
<h3>meta</h3>
<ul>
<li class='item'>
<a href='index.html'><img src='../content/index.png' loading='lazy'>index</a>
</li>
</ul>
</li>

</ul>
</nav>
";

pub const FOOTER: &str = "
<footer>
<p>
<a href='home.html'>andii land</a>
 © 2022
<a href ='https://creativecommons.org/licenses/by-nc-sa/4.0/'> by-nc-sa 4.0</a>
<a href ='https://github.com/andrewstraus99/me'> *website src</a>
</p>
</footer>
";
