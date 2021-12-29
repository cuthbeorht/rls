use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

fn main() {
    println!("Iterating over items in / directory");

    let mut entries = fs::read_dir("/").unwrap();

    for entry in entries {
        println!("{}", entry.unwrap().path().display());
    }
}
