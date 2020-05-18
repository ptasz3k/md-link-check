use clap::{App, Arg};
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
    let matches = App::new("md-link-check")
        .version("0.1")
        .author("Radek Krahl <radek@krahl.pl>")
        .about("Check for broken links in markdown documents")
        .arg("-s, --print-successes 'Prints links that are ok also'")
        .arg("-l, --local-only 'Check only local files'")
        .arg(
            Arg::new("starting-dir")
                .about("Start checking in that directory")
                .default_value("."),
        )
        .get_matches();

    let print_success = matches.is_present("print-successes");
    let local_only = matches.is_present("local-only");
    let starting_directory = matches.value_of("starting-dir").unwrap();

    let client = reqwest::blocking::Client::new();
    let style_info = Style::new().cyan();
    let style_err = Style::new().red();
    let style_success = Style::new().green();

    let mut errors = 0;

    for entry in
        glob(&format!("{}{}", starting_directory, "/**/*.md")).expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                let md = fs::read_to_string(&path).expect("Cannot read file");

                println!("FILE: {}", style_info.apply_to(path.display()));

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
                    } else if !local_only {
                        let res = client.head(l).send();
                        match res {
                            Err(_) => false,
                            _ => true,
                        }
                    } else {
                        true
                    };

                    if !result {
                        errors += 1;
                    }

                    if print_success || !result {
                        let mark = if result {
                            style_success.apply_to("✓")
                        } else {
                            style_err.apply_to("✗")
                        };
                        println!("  [{}] {}", mark, l);
                    }
                });

                println!("{} links checked.", links.len());
            }
            Err(e) => println!("{:?}", e),
        }

        println!();
    }

    std::process::exit(match errors {
        0 => 0,
        _ => 1,
    });
}
