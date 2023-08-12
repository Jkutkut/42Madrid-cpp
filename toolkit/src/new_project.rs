use open;

use crate::*;

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
    let highest = match get_project_number() {
        Some(num) => num,
        None => return,
    };
    let repo_name = format!("cpp0{}", highest);

    // Ask the user to create a new repository
    println!("Let's link the repository. Do you want to open the browser to do so?");
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

fn get_project_number(
) -> Option<i32> {
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
        return None;
    }
    Some(highest)
}
