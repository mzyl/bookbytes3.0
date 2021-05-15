use rand::Rng;
use std::fs::File;
use std::io::Read;

// get contents from file
pub fn get_file(filename: String) -> Vec<u8> {
    let path = filename;
    let mut file = File::open(&path).expect("invalid path");
    let mut text = Vec::new();
    file.read_to_end(&mut text)
        .expect("cannot read the file");

    println!("file had {} bytes", text.len());
    text
    //println!("{}", text);
}

// get file from list of files
pub fn get_filename() -> String {
    let path = "../gutenberg/bookbytes2.0/booklist.txt";
    let mut file = File::open(&path).expect("invalid path");
    let mut text = String::new();
    file.read_to_string(&mut text)
        .expect("cannot read the file");

    let lines : Vec<&str> = text.split('\n').collect();
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(0..lines.len()-1);
    println!("{}", rand);
    let line = lines[rand];
    //let line = ["books", line].join("/");
    let line = ["../gutenberg/library/htmlmirror", &line[1..]].join("");
    //let line: String = "../gutenberg/bookbytes2.0/books/11-h.htm".to_string();
    println!("Filename: {}", line);
    line
}

// need new paragraph function eventually
