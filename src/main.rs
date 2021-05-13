use rand::Rng;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

// get contents from file
fn get_file(filename: String) -> String {
    let path = filename;
    let mut file = File::open(&path).expect("invalid path");
    let mut text = String::new();
    file.read_to_string(&mut text)
        .expect("cannot read the file");

    println!("file had {} bytes", text.len());
    text
    //println!("{}", text);
}

// get file from list of files
fn get_filename() -> String {
    let path = "booklist.txt";
    let mut file = File::open(&path).expect("invalid path");
    let mut text = String::new();
    file.read_to_string(&mut text)
        .expect("cannot read the file");

    let lines : Vec<&str> = text.split('\n').collect();
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(0..lines.len()-1);
    println!("{}", rand);
    let line = lines[rand];
    let line = ["books", line].join("/");
    println!("Filename: {}", line);
    line
}

fn main() {
    let it = Instant::now();
    let text = get_file(get_filename());
    let words: Vec<&str> = text.split('\n').collect();
    let words = words.join(" ");
    let words: Vec<&str> = words
        .split('\r')
        .map(|string| string.trim())
        .filter(|&lines| !lines.is_empty())
        .collect();
    //println!("{:?}", works);
    println!("{}", words.len());
    println!("{}", words[50]);
    /*
    for line in words.iter() {
        println!("{}", line);
    }
    */
    //println!("{:?}", works2);
    println!("{}", it.elapsed().as_secs_f64());
}
