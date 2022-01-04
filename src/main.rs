use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::os::unix::fs::PermissionsExt;
use std::fs::Permissions;
use std::fmt;

struct File {
    path: String,
    name: String,
    size_in_bytes: u64,
    permissions: u32,
    is_file: String
}

fn main() {
    let current_working_directory =  env::var("PWD").unwrap();

    println!("Directory {}\n\n", current_working_directory);

    let mut entries = fs::read_dir(current_working_directory).unwrap();

    println!("File or Dir\tPermissions\tSize\tName");

    for entry in entries {
        let file = entry.unwrap();
        let file_metadata = get_file_info(&file.path());
        println!("{}\t\t{:?}\t\t{}\t{:?}", file_metadata.is_file, file_metadata.permissions, file_metadata.size_in_bytes, file_metadata.name);
    }
}

fn get_file_info(full_name_and_path: &PathBuf) -> File {

    let file_metadata = fs::metadata(&full_name_and_path).unwrap();
    let mut is_file: String = String::from("Dir");

    if file_metadata.is_file() {
        is_file = String::from("File")
    }

    let file: File = File {
        // path: full_name_and_path.into_os_string().into_string().unwrap(),
        path: String::from("fff"),
        name: String::from("main.rs"),
        size_in_bytes: file_metadata.len(),
        permissions: file_metadata.permissions().mode(),
        is_file: is_file
    };

    return file
}