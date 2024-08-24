use crate::{WebPage, WebPageFile};

/// Represents the index for the website.
///
/// Writes output to file `index.html`.
pub struct SiteIndex {
    pages: Vec<String>,
}

// Creates an index.html file in the site path
pub fn get_site_index(pages: &Vec<WebPageFile>) -> WebPage {
    println!("Generating site index...");
    let mut index = String::new();
    let mut index_pages: Vec<String> = Vec::new();

    index.push_str("<h2>Site Index</h2><ul>");

    for web_page in pages {
        index.push_str("<li><a href='");
        index.push_str(
            web_page
                .file_path
                .with_extension("html")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        );
        index.push_str("'>");
        index.push_str(web_page.file_path.file_stem().unwrap().to_str().unwrap());
        index.push_str("</a></li>");

        index_pages.push(String::from(
            web_page.file_path.file_stem().unwrap().to_str().unwrap(),
        ));
    }

    index.push_str("</ul>");

    WebPage::from_string(String::from("index"), index)
}
