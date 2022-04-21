use std::{env, fs, io};

fn main() {
    let files = list_file().unwrap();

    for file in files {
        println!("{}", file)
    }
}

fn list_file() -> Result<Vec<String>, io::Error> {
    let current_dir = env::current_dir()?;

    let mut ret: Vec<String> = Vec::new();
    for entry in fs::read_dir(current_dir)? {
        ret.push(entry?.file_name().into_string().expect("os string is invalid"));
    }
    
    Ok(ret)
}
