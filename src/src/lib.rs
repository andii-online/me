#![recursion_limit = "512"]

use std::path::PathBuf;
use std::io::Read;
use std::path::Path;
use std::fs;
use std::fs::{DirEntry, File, Metadata};
use regex::Regex;
use filetime::FileTime;
use chrono::offset::Utc;
use chrono::{DateTime, NaiveDateTime, Local};
use html::root::{Html, Body};
use html::content::{Header, Main, Footer};
use html::text_content::Division;
use html::metadata::Head;

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
        let mut file_contents = String::new();
        page_file.file.read_to_string(&mut file_contents).unwrap();

        let name = String::from(
            page_file.file_path.file_stem().unwrap().to_str().unwrap()
        );

        let modified_time= FileTime::from_last_modification_time(&page_file.metadata);
        let naive_time = NaiveDateTime::from_timestamp(modified_time.seconds(), modified_time.nanoseconds());
        let date_edited = DateTime::<Utc>::from_utc(naive_time, Utc);

        Ok(WebPage {
            name,
            content: replace_file_links(&file_contents, generate_link),
            date_edited,
        })
    }

    /// Converts a String into a WebPage
    pub fn from_string(name: String, content: String) -> WebPage {
        let date_edited: DateTime<Utc> = Local::now().into();
        WebPage {
            name,
            content,
            date_edited, 
        }
    }

    pub fn get_formatted_time(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.date_edited.with_timezone(&Local).format("%I:%M%p, %b %e, %Y").to_string().to_lowercase())
    }

    /// Consumes the WebPage and converts its content into the built version of the WebPage
    /// containing html header, navigation, and other page features.
    pub fn build(self, dest_dir: &Path) -> Result<(), &'static str> {

        let body = Body::builder()
            .push(self.get_header())
            .push(self.get_main())
            .push(self.get_footer())
            .build();

        let html = Html::builder()
            .lang("en")
            .push(self.get_head())
            .push(body)
            .build();

       println!(
            "Writing {} to {}",
            self.name,
            dest_dir.join(&self.name).display()
        );

        // create a new file with that file name in ../site/
        fs::write(
            dest_dir.join(format!("{}.html", &self.name)), 
            html.to_string().into_bytes()
        ).unwrap();

        Ok(())
    }

    fn get_header(&self) -> Header {
        let mut header = Header::builder();
        let div = Division::builder()
            .class("special")
            .heading_1(|h1| h1
                .anchor(|a| a
                    .href("special.html")
                    .text("*")
                    )
                )
                .text(SITE_NAME)
            .build();
        header.push(div);
        // add back to home nav for all non-home pages.
        if self.name != "home" {
            let back_to_home = Division::builder()
                .class("mini-indent")
                .anchor(|a| a
                    .href("home.html")
                    .text("back to home")
                    )
                .build();
            header.push(back_to_home);
        }
        
        header.build()
    }

    fn get_head(&self) -> Head {
        let head = Head::builder()
            .title(|title| title 
                .text(format!("{} - {}", SITE_NAME, self.name))
                )
            .meta(|meta| meta
                .name("description")
                .content(format!("welcome to {}!!", SITE_NAME))
                )
            .link(|link| link
                .rel("apple-touch-icon")
                .sizes("180x180")
                .href("../icons/apple-touch-icon.png")
                )
            .link(|link| link
                .rel("manifest")
                .href("../site.manifest")
                )
            .meta(|meta| meta
                .name("viewport")
                .content("width=device-width, initial-scale=1.0")
                )
            .link(|link| link
                .href("../styles/style.css")
                .rel("stylesheet")
                )
            .build();

        head
    }

    fn get_main(&self) -> Main {
        let main = Main::builder()
            .division(|inner| inner 
                .class("inner")
                .division(|indent| indent
                    .class("indent")
                    .text(self.content.clone())
                    )
                )
            .build();

        main
    }

    fn get_footer(&self) -> Footer {
        let gh_link = "https://github.com/andii-online/me";
        let last_edited_msg = format!("edited on {}", self.get_formatted_time().unwrap());
        let footer = Footer::builder()
            .division(|div| div
                .class("left")
                .paragraph(|p| p
                    .anchor(|a| a
                        .text("*website src")
                        .href(gh_link)
                        )
                    )
                )
            .division(|div| div
                .class("right")
                .paragraph(|p| p
                    .text(last_edited_msg)
                    )
                )
            .build();

        footer
    }
}
