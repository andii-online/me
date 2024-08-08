use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::fs;
use std::fs::{DirEntry, File, Metadata};
use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;

pub const SITE_NAME: &str = "chloe land";

/// Represents a source file for website pages
pub struct WebPageFile {
    pub file_path: PathBuf, // idk about having this be public :/
    file: File,
    metadata: Metadata,
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
}

/// Represents a page on the website
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WebPage {
    name: String,
    content: String,
    date_edited: DateTime<Utc>,
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

        let system_time = page_file.metadata.modified().unwrap();
        let date_edited: DateTime<Utc> = system_time.into();

        Ok(WebPage {
            name,
            content: file_contents,
            date_edited,
        })
    }

    /// Converts a String into a WebPage
    pub fn from_string(name: String, content: String) -> WebPage {
        let date_edited: DateTime<Utc> = SystemTime::now().into();
        WebPage {
            name,
            content,
            date_edited, // while this is kindof correct, the date should only
                         // be changed when the content of the page changes.
        }
    }

    /// Consumes the WebPage and converts its content into the built version of the WebPage
    /// containing html header, navigation, and other page features.
    pub fn build(self, dest_dir: &Path) -> Result<(), &'static str> {
        let mut content = String::new();
        content.push_str(&self.get_header());
        content.push_str("<body>\n");
        content.push_str("<!-----------------GENERATED DO NOT EDIT----------------->\n");
        content.push_str("<main>\n");
        content.push_str("<div class='inner'>");
        if self.name != "home" {
            content.push_str("<div class='mini-indent'>");
            content.push_str("<a href='home.html'>back to home</a>");
            content.push_str("</div>");
        }
        content.push_str("<div class='indent'>");
        content.push_str(&self.content);
        content.push_str("</div>");
        content.push_str("</div>");
        content.push_str("</main>\n");
        content.push_str(&self.get_footer());
        content.push_str("</body>\n");

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
</head><header><h1>*chloe land</h1></header>\n", SITE_NAME));

        content
    }

    fn get_footer(&self) -> String {
        let mut footer = String::new();
        footer.push_str("<footer>");
        footer.push_str("<div class='left'>");
        footer.push_str("<p>");
        footer.push_str("<a href='home.html'>chloe land</a>");
        footer.push_str("© 2022");
        footer.push_str("<a href ='https://creativecommons.org/licenses/by-nc-sa/4.0/'> by-nc-sa 4.0</a>
");
        footer.push_str("<a href ='https://github.com/andii-online/me'> *website src</a>");
        footer.push_str("</p>");
        footer.push_str("</div>");
        footer.push_str("<div class='right'>");
        footer.push_str("<p>");
        footer.push_str(format!("edited on {}", self.date_edited.format("%a %b %e %T %Y")).as_str());
        footer.push_str("</p>");
        footer.push_str("</div>");
        footer.push_str("</footer>");

        footer
    }
}
