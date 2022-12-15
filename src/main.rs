use std::fs;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() <= 1 {
        println!("Usage: mkbak [-f]? <files> [-f]?");
        return;
    }

    println!("");
    let force: bool; //force overwrite of already existing .bak files
    let startindex: usize;
    let mut endindex: usize = args.len();

    if &args[1] == "-f" {
        force = true;
        startindex = 2;
    }
    else if &args[args.len() - 1] == "-f" {
        force = true;
        startindex = 1;
        endindex = args.len() - 1;
    }
    else {
        force = false;
        startindex = 1;
    }

    for file in &args[startindex..endindex] {
        match make_bak(file, force) {
            Ok(_) => println!("[mkbak] {} -> {}.bak", file, file),
            Err(e) => println!("[error] {}: {}", file, e),
        }
    }
    println!("");
}

fn make_bak(file: &str, force: bool) -> Result<(), std::io::Error> {
    let mut bak = file.to_string();
    bak.push_str(".bak");

    if fs::metadata(&bak).is_ok() { //if .bak file already exists
        if !force { //if force is not enabled log error, else continue to overwrite
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Backup file already exists (-f to overwrite)",
            ));
        }
    }

    fs::copy(file, bak)?;
    Ok(())
}
