use pulldown_cmark::{html, Event, Options, Parser, Tag};

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
    /* FIXME: find recursively all md, read, extract links, check links, print output */
    let markdown_input = "Hello, world!, this is a [link](../dupa.md), ![obrazek](images/img1.png)";
    let links = extract_links(markdown_input);
    println!("{:?}", links);
}
