use glob::glob;
use pulldown_cmark::{Event, Parser, Tag};
use std::fs;

fn extract_links(md: &str) -> Vec<String> {
    let mut links: Vec<String> = Vec::new();
    Parser::new(md).for_each(|event| match event {
        Event::Start(Tag::Link(_, link, _)) => links.push(link.into_string()),
        Event::Start(Tag::Image(_, link, _)) => links.push(link.into_string()),
        _ => (),
    });

    links
}

fn main() {
    for entry in glob("/home/radekk/kide.doc/src//**/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                let md = fs::read_to_string(path).expect("Cannot read file");
                let links = extract_links(&md);
                println!("{:?}", links);
                /* FIXME: check links, print output */
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
