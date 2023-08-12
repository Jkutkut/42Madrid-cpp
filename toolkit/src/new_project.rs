use crate::*;
use open;

pub fn new_project(
    args: Vec<String>
) {
    match args.len() {
        2 => {},
        _ => {
            eprintln!("Usage: {} new_project", args[0]);
            return;
        }
    }
    // Obtain all the directories called cpp0* in the current directory
    let dirs = std::fs::read_dir(".").unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().unwrap().is_dir())
        .filter(|e| e.file_name().to_str().unwrap().starts_with("cpp0"))
        .collect::<Vec<_>>();
    let mut highest = 0;
    for dir in dirs.iter() {
        let name = dir.file_name();
        let num = match name.to_str().unwrap()[4..].parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if num > highest {
            highest = num;
        }
    }
    if highest >= 9 {
        eprintln!("All projects have been created");
        return;
    }
    println!("Next project: cpp0{}", highest);

    // Ask the user to create a new repository
    println!("A new repository is needed. Open the browser to do so?");
    if confirm("Open browser?") {
        let url = "https://github.com/new";
        match open::that(url) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error opening browser: {}", e);
                println!("Please open the browser manually:\n{}", url);
            }
        }
    }
    // Clone the repository
    // TODO create ask ft
    // TODO: What now?
    eprintln!("TODO: new_project");
}

use std::io::Write;
fn confirm(
    question: &str
) -> bool {
    let mut input = String::new();
    print!("{} [y/n]", question);
    match std::io::stdout().flush() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error flushing stdout: {}", e);
            std::process::exit(1);
        }
    }
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error reading stdin: {}", e);
            std::process::exit(1);
        }
    }
    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false,
    }
}
