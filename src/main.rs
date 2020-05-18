use glob::glob;
use pulldown_cmark::{Event, Parser, Tag};
use std::fs;
use std::path::{Path, PathBuf};

const URL_SCHEMAS: &[&str] = &["https://", "http://"];

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
    let client = reqwest::blocking::Client::new();
    for entry in glob("../../kide.doc/src//**/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let md = fs::read_to_string(&path).expect("Cannot read file");

                println!("FILE: {}", path.display());
                println!();

                let links = extract_links(&md);
                let parent = path.parent();
                links.iter().for_each(|l| {
                    let is_url = URL_SCHEMAS.into_iter().any(|schema| l.starts_with(schema));

                    let result = if !is_url {
                        let to_check = match parent {
                            Some(p) => p.join(Path::new(l)),
                            None => PathBuf::from(l),
                        };
                        to_check.exists()
                    } else {
                        let res = client.head(l).send();
                        match res {
                            Err(_) => false,
                            _ => true,
                        }
                    };

                    if !result {
                        println!("{}: {}", l, "ERROR");
                    }
                });

                println!();
                println!("{} links checked.", links.len());
            }
            Err(e) => println!("{:?}", e),
        }
        println!("\n\n");
    }
}
