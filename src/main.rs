use clap::command;
use clap::Parser;
use console::Style;
use glob::glob;
use pulldown_cmark::{Event, Tag};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const URL_SCHEMAS: &[&str] = &["https://", "http://"];

fn extract_links(md: &str) -> Vec<String> {
    let mut links: Vec<String> = Vec::new();
    pulldown_cmark::Parser::new(md).for_each(|event| match event {
        Event::Start(Tag::Link(_, link, _)) => links.push(link.into_string()),
        Event::Start(Tag::Image(_, link, _)) => links.push(link.into_string()),
        _ => (),
    });

    links
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[arg(long, short)]
    /// Print successful links
    print_successes: bool,
    #[arg(long, short)]
    /// Check only local links
    local_only: bool,
    #[arg(long, short)]
    /// Allow only ASCII characters in link paths
    ascii_only: bool,
    #[arg(default_value = ".")]
    /// Starting directory for recursive search
    starting_directory: String,
}

fn parse_options() -> Cli {
    Cli::parse()
}

fn check_local(parent: Option<&Path>, l: &str) -> bool {
    let link_path = Path::new(l);
    let to_check = match parent {
        Some(p) if !link_path.is_absolute() => p.join(link_path),
        _ => PathBuf::from(l),
    };
    to_check.exists()
}

fn check_remote(url: &str) -> bool {
    let res = ureq::head(url).call();
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
                            let is_ascii = if opts.ascii_only {
                                l.chars().all(|c| c.is_ascii())
                            } else {
                                true
                            };
                            let is_url = URL_SCHEMAS.iter().any(|schema| l.starts_with(schema));
                            let r = is_ascii
                                && if !is_url {
                                    checked += 1;
                                    check_local(parent, l)
                                } else if !opts.local_only {
                                    checked += 1;
                                    check_remote(l)
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

                    if opts.print_successes || !correct {
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
