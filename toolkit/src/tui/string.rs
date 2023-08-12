use super::*;

pub fn ask(
    question: &str
) -> String {
    prompt_question(question);
    read_line()
}

pub fn confirm(
    question: &str
) -> bool {
    let r = ask(format!("{} [y/n]", question).as_str());
    match r.to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false,
    }
}

pub fn acknowledge(
    question: &str
) {
    prompt_question(question);
    let _ = read_line();
}

pub fn ask_or_default(
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
