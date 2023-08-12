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
    let mut highest = -1;
    for dir in dirs.iter() {
        let name = dir.file_name();
        let num = match name.to_str().unwrap()[4..].parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if num > highest {
            highest = num;
        }
    }
    highest += 1;
    if highest >= 9 {
        eprintln!("All projects have been created");
        return;
    }
    let repo_name = format!("cpp0{}", highest);
    println!("Next project: {}", repo_name);

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
    let repo = ask_or_default(
        "Repository URL?",
        format!("git@github.com:Jkutkut/42Madrid-cpp0{}.git", highest)
    );

    // TODO submodule instead of clone
    let clone_status = std::process::Command::new("git")
        .arg("clone")
        .arg(repo)
        .arg(repo_name.clone())
        .status();

    match clone_status {
        Ok(status) => {
            if !status.success() {
                eprintln!("Error cloning repository. Error code: {}", status.code().unwrap());
                return;
            }
        },
        Err(e) => {
            eprintln!("Error cloning repository: {}", e);
            return;
        }
    }

    // TODO: What now?
    eprintln!("TODO: new_project");
}


use std::io::Write;
fn read_line() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error reading stdin: {}", e);
            std::process::exit(1);
        }
    }
    input.trim().to_string()
}

fn prompt_question(
    question: &str
) {
    print!("{} ", question);
    if let Err(e) = std::io::stdout().flush() {
        eprintln!("Error flushing stdout: {}", e);
        std::process::exit(1);
    }
}

fn ask(
    question: &str
) -> String {
    prompt_question(question);
    read_line()
}

fn confirm(
    question: &str
) -> bool {
    let r = ask(format!("{} [y/n]", question).as_str());
    match r.to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false,
    }
}

fn acknowledge(
    question: &str
) {
    prompt_question(question);
    let _ = read_line();
}

fn ask_or_default(
    question: &str,
    default: String
) -> String {
    let r = ask(format!("{} [{}]", question, default).as_str());
    if r.is_empty() {
        default
    } else {
        r
    }
}
