use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

use spider_rs_common::result::Error;
use spider_rs_common::result::Result;

fn extract_links(html: &str) -> Result<Vec<String>> {
    let document = Document::from(html);

    let mut links = Vec::with_capacity(64);

    for node in document.select(Name("a")) {
        links.push(node.attr("href").unwrap().to_string());
    }

    Ok(links)
}
