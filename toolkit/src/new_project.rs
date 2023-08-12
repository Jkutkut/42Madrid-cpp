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
    // Obtain all the directories called cpp0* in the current directory
    // Get the highest number
    // Ask the user to create a new repository
    // Clone the repository
    // TODO: What now?
    eprintln!("TODO: new_project");
}
