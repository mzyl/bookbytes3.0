use crate::selector::*;
// Add the extra data types so we can put together
// a function to recursively build a string.
use kuchiki::{NodeData, NodeRef, ElementData};

// book struct
pub struct Book<'a> {
    pub filename: String, // this isn't in the Go version
    pub full_html: Vec<&'a str>,
    pub book_text: String,
// chapter ref index
// paragraph ref index
}

// create book function

pub fn get_html(filename: String) -> Vec<&'static str> {
    let text = crate::selector::get_file(filename);
    let text = String::from_utf8_lossy(&text);
    let words: Vec<&str> = text.split('\n').collect();
    let words = words.join(" ");
    let html: Vec<&str> = words
        .split('\r')
        .map(|string| string.trim())
        .filter(|&lines| !lines.is_empty())
        .collect();
    html
}

pub fn get_book_text(html: Vec<&str>) -> String {
    let book_text: String = html.join(" ");
    book_text
}

/// Setter functions

// set book title
/// takes book text as input and returns title as a string

// set book author
/// takes book text as input and returns author as a string

// set book language
/// takes book text as input and returns language as a string

// set chapter references
/// takes book text as input and returns a slice/vec of integers for each
/// chapter found

// set chapter
/// takes filename and paragraph integer as input and returns whole 
/// chapter as a string and updated chapter reference as an integer

// strip license 
/// takes html and returns html without the license

// split text
/// takes html as input and returns book text as a string slice/vec

/// Getter functions

// get chapters

// get paragraphs


// Take a reference to the node
pub fn get_text(node: &NodeRef) -> Option<String> {

    // Match on the node data to see what kind we have.
    // If we have an element we want to look at all it's children
    // And if we have a text node we just want to return the inner text.
    match node.data() {

        // It's an element, not a text node,
        // so we check all the children until we find a text node.
        NodeData::Element(el) => {

            // Empty string (mutable) so we can build this up with all the text
            // from the children
            let mut s = String::new();

            for child in node.children() {
                // We pass each child to the `get_text` function,
                // so this is recursive.
                // If we get some text back we push it on to our `s` string.
                // otherwise we just continue
                match get_text(&child) {
                    Some(text) => s.push_str(&text),
                    None => continue,
                }
            }

            // Finally we return the text string
            return Some(s);
        }
        NodeData::Text(text) => {
            // It's a text node, so we just take the text and return it
            return Some(text.borrow().to_string());
        }
        _ => {
            // We don't care about the other node types
            println!("{:?}", node.data())
        }
    }

    // Finally we return None here, which means we never ran into
    // a valid element or text node.
    //
    // Rust has implicit return so we don't really have to type retur here.
    None
}
