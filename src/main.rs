use glob::glob;
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use std::fs;

fn extract_links(md: &str) -> Vec<String> {
    let options = Options::empty();
    let mut links: Vec<String> = Vec::new();
    let parser = Parser::new_ext(md, options).map(|event| match event {
        Event::Start(Tag::Link(x, link, z)) => {
            links.push(link.clone().into_string());
            Event::Start(Tag::Link(x, link, z))
        }
        Event::Start(Tag::Image(x, link, z)) => {
            links.push(link.clone().into_string());
            Event::Start(Tag::Image(x, link, z))
        }
        _ => event,
    });

    let mut html_output = String::new();
    /* FIXME: do not render html, just parse md */
    html::push_html(&mut html_output, parser);
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
