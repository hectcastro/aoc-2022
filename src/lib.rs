use std::{env, fs};

pub fn read_file_input(file: String) -> String {
    let cwd = env::current_dir().unwrap();
    let path = cwd.join("src/inputs").join(file);

    fs::read_to_string(path).unwrap()
}
