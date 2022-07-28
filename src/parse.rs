use std::fs;

pub fn Parse(path_asm: &str) -> String{
    match fs::read(&path_asm) {
        Ok(v) => String::from_utf8(v).expect("Invalid UTF-8"),
        Err(_) => String::from("Failed to read file"),
    }
}