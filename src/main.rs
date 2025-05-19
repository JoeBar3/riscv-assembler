use std::fs::File;
use std::io::{Error, BufRead, BufReader};
use std::process::exit;
use std::path::Path;
use std::env;

fn main() {
    // Get command line arg for the path.
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1){
        None => {eprint!("Error: not enough command line arguments.\n"); exit(1);},
        Some(path) => path
    };
    let file = match read_file(&path){
        Err(error) => {eprint!("Error: {}\n", error); exit(1);},
        Ok(file) => file,
    };
    println!("{}",file);
}

// This method reads in the file, and preprocesses it (handles INCLUDE statements).
fn read_file<P>(path: P) -> Result<String, Error>
where P: AsRef<Path>{
    // Get the base directory, so we can pass it as an arg for include statements.
    let path = path.as_ref();
    let base_path = path.parent().unwrap_or_else(|| Path::new(""));
    let file = File::open(path)?;
    let mut file_contents = String::new();
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;
        if line.to_uppercase().starts_with("INCLUDE "){
            let new_path_end = line[7..].trim();
            let new_path = base_path.join(new_path_end);
            let new_contents = read_file(&new_path)?;
            file_contents.push_str(&new_contents);
        } else {
            file_contents.push_str(&line);
            file_contents.push('\n');
        }
    }
    // Get rid of the extra newline.
    //file_contents.pop();
    Ok(file_contents)
}