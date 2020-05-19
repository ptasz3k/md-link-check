use clap::{App, Arg};
use console::Style;
use glob::glob;
use pulldown_cmark::{Event, Parser, Tag};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const URL_SCHEMAS: &[&str] = &["https://", "http://"];

struct Options {
    print_success: bool,
    local_only: bool,
    starting_directory: String,
}

fn extract_links(md: &str) -> Vec<String> {
    let mut links: Vec<String> = Vec::new();
    Parser::new(md).for_each(|event| match event {
        Event::Start(Tag::Link(_, link, _)) => links.push(link.into_string()),
        Event::Start(Tag::Image(_, link, _)) => links.push(link.into_string()),
        _ => (),
    });

    links
}

fn parse_options() -> Options {
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

    Options {
        print_success: matches.is_present("print-successes"),
        local_only: matches.is_present("local-only"),
        starting_directory: String::from(matches.value_of("starting-dir").unwrap()),
    }
}

fn check_local(parent: Option<&Path>, l: &str) -> bool {
    let link_path = Path::new(l);
    let to_check = match parent {
        Some(p) if !link_path.is_absolute() => p.join(link_path),
        _ => PathBuf::from(l),
    };
    to_check.exists()
}

fn check_remote(client: &reqwest::blocking::Client, url: &str) -> bool {
    let res = client.head(url).send();
    match res {
        Err(_) => false,
        _ => true,
    }
}

fn main() {
    let opts = parse_options();

    let style_info = Style::new().cyan();
    let style_err = Style::new().red();
    let style_success = Style::new().green();

    let mut errors = 0;
    let client = reqwest::blocking::Client::new();
    let mut mem = HashMap::<String, bool>::new();

    for entry in glob(&format!("{}{}", opts.starting_directory, "/**/*.md"))
        .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                let md = fs::read_to_string(&path).expect("Cannot read file");

                println!("FILE: {}", style_info.apply_to(path.display()));

                let links = extract_links(&md);
                let parent = path.parent();
                let mut checked = 0;
                links.iter().for_each(|l| {
                    let correct = match mem.get(l) {
                        Some(r) => {
                            checked += 1;
                            *r
                        }
                        None => {
                            let is_url = URL_SCHEMAS.iter().any(|schema| l.starts_with(schema));
                            let r = if !is_url {
                                checked += 1;
                                check_local(parent, l)
                            } else if !opts.local_only {
                                checked += 1;
                                check_remote(&client, l)
                            } else {
                                true
                            };

                            mem.insert(l.clone(), r);
                            r
                        }
                    };

                    if !correct {
                        errors += 1;
                    }

                    if opts.print_success || !correct {
                        let mark = if correct {
                            style_success.apply_to("✓")
                        } else {
                            style_err.apply_to("✗")
                        };
                        println!("  [{}] {}", mark, l);
                    }
                });

                println!("{}/{} links checked.", checked, links.len());
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
