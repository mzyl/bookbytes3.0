use std::fs::File;
use std::io::Read;

fn get_file() -> String {
    let path = "books/11-h.htm";
    let mut file = File::open(&path).expect("invalid path");
    let mut text = String::new();
    file.read_to_string(&mut text)
        .expect("cannot read the file");

    println!("file had {} bytes", text.len());
    text
    //println!("{}", text);
}

fn main() {
    let text = get_file();
    let words: Vec<&str> = text.split('\n').collect();
    let works = words.join(" ");
    let works2: Vec<&str> = works
        .split('\r')
        .map(|string| string.trim())
        .filter(|&lines| lines != "")
        .collect();
    //println!("{:?}", works);
    println!("{}", words.len());
    println!("{}", words[501]);
    println!("{}", works2.len());
    println!("{}", works2[500]);
    for line in works2 {
        println!("{}", line);
    }
    //println!("{:?}", works2);
}
