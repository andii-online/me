use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::fs;
use std::fs::{DirEntry, File, Metadata};
use std::time::SystemTime;
use regex::Regex;
use filetime::FileTime;
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
    date_edited: FileTime,
}

fn replace_file_links(input: &str, generate_link: fn(&str, Option<&str>) -> String) -> String {
    // regex to match {file_name, optional[pretty_name]}
    let re = Regex::new(r"\{([^,}]+)(?:,([^}]+))?\}").unwrap();
    let result = re.replace_all(input, |caps: &regex::Captures| {
        let file_name = &caps[1];
        let pretty_name = caps.get(2).map(|m| m.as_str());
        generate_link(file_name, pretty_name)
    });

    result.to_string()
}

fn generate_link(file_name: &str, pretty_name: Option<&str>) -> String {
    match pretty_name {
        Some(name) => format!("<a href='{}.html'>{}</a>", file_name, name),
        None => format!("<a href='{}.html'>{}</a>", file_name, file_name),
    }
}

impl WebPage {
    /// Constructs a new WebPage from an .htm source
    pub fn from_web_page_file(mut page_file: WebPageFile) -> Result<WebPage, &'static str> {
        let mut file_contents = String::new();
        page_file.file.read_to_string(&mut file_contents).unwrap();

        let name = String::from(
            page_file.file_path.file_stem().unwrap().to_str().unwrap()
        );

        let date_edited = FileTime::from_last_modification_time(&page_file.metadata);

        Ok(WebPage {
            name,
            content: replace_file_links(&file_contents, generate_link),
            date_edited,
        })
    }

    /// Converts a String into a WebPage
    pub fn from_string(name: String, content: String) -> WebPage {
        let date_edited: FileTime = FileTime::now();
        WebPage {
            name,
            content,
            date_edited, // while this is kindof correct, the date should only
                         // be changed when the content of the page changes.
        }
    }

    pub fn get_formatted_time(&self) -> Result<String, Box<dyn std::error::Error>> {
        let system_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(self.date_edited.unix_seconds() as u64);
        let datetime: DateTime<Utc> = system_time.into();
        Ok(datetime.format("%a %b %e %T %Y").to_string())
    }

    /// Consumes the WebPage and converts its content into the built version of the WebPage
    /// containing html header, navigation, and other page features.
    pub fn build(self, dest_dir: &Path) -> Result<(), &'static str> {
        let mut content = String::new();
        content.push_str(&self.get_header());
        content.push_str("<body>\n");
        content.push_str("<!-----------------GENERATED DO NOT EDIT----------------->\n");
        content.push_str(&self.get_main());
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
        content.push_str(&format!("<title>{} - {}</title>", SITE_NAME, &self.name));
        content.push_str(&format!("<meta name='description' content='welcome to {}!!'>", SITE_NAME));
        content.push_str("<link rel='apple-touch-icon' sizes='180x180' href='../icons/apple-touch-icon.png'>");
        content.push_str("<link rel='manifest' href='../site.webmanifest'>");
        content.push_str("<meta name='viewport' content='width=device-width, initial-scale=1.0'>");
        content.push_str("<link href='../styles/style.css' rel='stylesheet'>");
        content.push_str("</head>");
        content.push_str("<header>");
        content.push_str("<h1>");
        content.push_str("*chloe land");
        content.push_str("</h1>");
        // add back to home nav for all non-home pages.
        if self.name != "home" {
            content.push_str("<div class='mini-indent'>");
            content.push_str("<a href='home.html'>back to home</a>");
            content.push_str("</div>");
        }
        content.push_str("</header>");

        content
    }

    fn get_main(&self) -> String {
        let mut main = String::new();

        main.push_str("<main>\n");
        main.push_str("<div class='inner'>");
        main.push_str("<div class='indent'>");
        if self.name != "home" {
            main.push_str(format!("<h1>{}</h1>", &self.name).as_str());
            main.push_str("<div class='indent'>");
        }
        main.push_str(&self.content);
        main.push_str("</div>");
        main.push_str("</div>");
        main.push_str("</div>");
        main.push_str("</main>\n");

        main 
    }

    fn get_footer(&self) -> String {
        let mut footer = String::new();
        footer.push_str("<footer>");
        footer.push_str("<div class='left'>");
        footer.push_str("<p>");
        footer.push_str("<a href ='https://github.com/andii-online/me'> *website src</a>");
        footer.push_str("</p>");
        footer.push_str("</div>");
        footer.push_str("<div class='right'>");
        footer.push_str("<p>");
        footer.push_str(format!("edited on {}", self.get_formatted_time().unwrap()).as_str());
        footer.push_str("</p>");
        footer.push_str("</div>");
        footer.push_str("</footer>");

        footer
    }
}
