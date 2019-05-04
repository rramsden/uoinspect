use std::{
    path::{Path}
};

pub fn print_json<T>(entries: Vec<T>, s: &Fn(&T) -> std::result::Result<std::string::String, serde_json::error::Error>) {
    print!("[");
    for i in 0..entries.len() {
        let entry = &entries[i];
        let json = s(&entry).unwrap();

        print!("{}", json);
        if i != entries.len() - 1 {
            print!(",")
        }
    }
    print!("]\n")
}

pub fn base_path(path: &str) -> String {
    let file_name = Path::new(path).file_name().unwrap().to_str().unwrap();
    str::replace(path, file_name, "")
}
