use std::io::Write;

pub fn read_line() -> String {
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

pub fn prompt_question(
    question: &str
) {
    print!("{} ", question);
    if let Err(e) = std::io::stdout().flush() {
        eprintln!("Error flushing stdout: {}", e);
        std::process::exit(1);
    }
}
