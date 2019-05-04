use std::{
    path::{Path, PathBuf}
};
use super::index;
use super::utils::base_path;

pub fn from_index(file_path: &str, entry_id: usize) {
    let index_path = get_index_path(file_path);
    index::read_binary(file_path, &index_path[..], entry_id);
}

fn get_index_path(path: &str) -> String {
    let file_name = Path::new(path).file_name().unwrap().to_str().unwrap();
    let index_file_name = match &file_name[..] {
        "Gumpart.mul" => "Gumpidx.mul",
        _ => panic!("Could not find index file for {}.", file_name)
    };
    let new_path = PathBuf::from(base_path(path)).join(index_file_name);

    String::from(new_path.to_str().unwrap())
}
