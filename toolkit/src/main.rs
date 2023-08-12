mod new_project;
mod model;

use new_project::new_project;
use model::*;


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        usage();
        return;
    }

    let cmd = match Command::from(&args[1]) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    match cmd {
        Command::NewProject => new_project(args),
    }
}

fn usage() {
    let program_name = std::env::args().nth(0).unwrap();
    println!("Usage: {} <cmd>", program_name);

    println!("Commands:");
    println!("  new_project: Create a new project");
}
