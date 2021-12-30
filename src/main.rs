use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::env;

struct File {
    path: String,
    name: String,
    size_in_bytes: u32,
    permissions: String,
    is_file: bool
}

fn main() {
    let current_working_directory =  env::var("PWD").unwrap();

    println!("Directory {}\n\n", current_working_directory);

    let mut entries = fs::read_dir(current_working_directory).unwrap();

    for entry in entries {
        let file = entry.unwrap().path();
        let file_metadata = fs::metadata(&file);
        println!("{}", file.display());
        println!("{} has the following metadata: {:?}", file.display(), file_metadata);
    }
}

fn get_file_info(full_name_and_path: String) -> File {
    let file: File = File {
        path: String::from("/home/davids/src/rls/src/main.rs"),
        name: String::from("main.rs"),
        size_in_bytes: 4096,
        permissions: String::from("----------"),
        is_file: true
    };

    return file
}