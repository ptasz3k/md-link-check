use console::Style;
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
    let print_success = true;
    let starting_directory = "../../kide.doc/src/**/*.md";

    let client = reqwest::blocking::Client::new();
    let style_info = Style::new().cyan();
    let style_err = Style::new().red();
    let style_success = Style::new().green();

    for entry in glob(starting_directory).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let md = fs::read_to_string(&path).expect("Cannot read file");

                println!("FILE: {}", style_info.apply_to(path.display()));
                println!();

                let links = extract_links(&md);
                let parent = path.parent();
                links.iter().for_each(|l| {
                    let is_url = URL_SCHEMAS.iter().any(|schema| l.starts_with(schema));

                    let result = if !is_url {
                        let link_path = Path::new(l);
                        let to_check = if link_path.is_absolute() {
                            PathBuf::from(l)
                        } else {
                            match parent {
                                Some(p) => p.join(link_path),
                                None => PathBuf::from(l),
                            }
                        };
                        to_check.exists()
                    } else {
                        let res = client.head(l).send();
                        match res {
                            Err(_) => false,
                            _ => true,
                        }
                    };

                    if print_success || !result {
                        let mark = if result {
                            style_success.apply_to("✓")
                        } else {
                            style_err.apply_to("✗")
                        };
                        println!("  [{}] {}", mark, l);
                    }
                });

                println!();
                println!("{} links checked.", links.len());
            }
            Err(e) => println!("{:?}", e),
        }

        println!();
    }
}
