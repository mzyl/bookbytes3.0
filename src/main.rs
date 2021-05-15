mod book;
mod selector;

use std::time::Instant;
use kuchiki::traits::*;

// main should basically handle webserver stuff and nothing else, I think.
fn main() {
    let it = Instant::now();
    let filename = selector::get_filename();
    let book = book::Book {
        filename: filename.clone(),
        full_html: book::get_html(filename),
    };
    println!("{}", book.filename);

    let selector = "h2";
    let document = kuchiki::parse_html().one(book.full_html);

    for css_match in document.select(selector).unwrap() {
        let as_node = css_match.as_node();
        match book::get_text(&as_node) {
            Some(value) => println!("{}", value),
            None => {}
        };
    }

    println!("{:?}", it.elapsed());
}

