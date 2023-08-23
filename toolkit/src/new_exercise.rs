// use crate::*;
use std::io::Write;

pub fn new_exercise(_args: Vec<String>) {
    if !is_git_repo() {
        eprintln!("Not on the root of a git repository");
        return;
    }

    // Create directory
    let exercise_dir = format!("ex{:0>2}", calc_exercise_name());
    println!("Creating directory {}...", exercise_dir);
    match std::fs::create_dir(&exercise_dir) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error creating directory: {}", e);
            return;
        }
    }

    // name binary
    let mut bin_name = String::new();
    println!("Enter the name of the binary: ");
    std::io::stdin().read_line(&mut bin_name).unwrap();
    bin_name.trim().to_string();

    // create main.cpp
    create_main(&exercise_dir);

    // update .gitignore
    update_gitignore(&bin_name);

    // Create Makefile
    create_makefile(&exercise_dir, &bin_name);
}

fn is_git_repo() -> bool {
    let dot_git = std::path::Path::new(".git");
    dot_git.exists() && dot_git.is_dir()
}

fn create_main(dir: &str) {
    println!("Creating main.cpp...");
    let mut main = std::fs::File::create(format!("{}/main.cpp", dir)).unwrap();
    main.write(b"//#include \".hpp\"\n").unwrap();
    main.write(b"//#include <iostream>\n").unwrap();
    main.write(b"\n").unwrap();
    main.write(b"int\tmain(void) {\n").unwrap();
    main.write(b"\t\n").unwrap();
    main.write(b"\treturn 0;\n").unwrap();
    main.write(b"}\n").unwrap();
}

fn update_gitignore(bin_name: &str) {
    println!("Updating .gitignore...");
    let mut gitignore = std::fs::OpenOptions::new()
        .append(true)
        .open(".gitignore")
        .unwrap();
    gitignore.write(format!("{}", bin_name).as_bytes()).unwrap();
}

fn calc_exercise_name() -> i32 {
    let dirs = std::fs::read_dir(".").unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().unwrap().is_dir())
        .filter(|e| e.file_name().to_str().unwrap().starts_with("ex"))
        .collect::<Vec<_>>();
    let mut highest: i32 = -1;
    for dir in dirs.iter() {
        let name = dir.file_name();
        let num = match name.to_str().unwrap()[2..].parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if num > highest {
            highest = num;
        }
    }
    highest + 1
}

fn create_makefile(dir: &str, bin_name: &str) {
    let mut makefile = std::fs::File::create(format!("{}/Makefile", dir)).unwrap();
    makefile.write(b"CC = g++\n").unwrap();
    makefile.write(b"FLAGS = -Wall -Wextra -Werror -std=c++98\n").unwrap();
    makefile.write(b"\n").unwrap();
    makefile.write(b"BIN = bin\n").unwrap();
    makefile.write(b"\n").unwrap();
    makefile.write(format!("NAME = {}\n", bin_name).as_bytes()).unwrap();
    makefile.write(b"\n").unwrap();
    makefile.write(b"SRCS =\tmain.cpp\n").unwrap();
    makefile.write(b"\n").unwrap();
    makefile.write(b"OBJS = $(SRCS:%.cpp=${BIN}/%.o)\n").unwrap();
    makefile.write(b"\n").unwrap();
    makefile.write(b"all: $(NAME)\n").unwrap();
    makefile.write(b"\n").unwrap();
    makefile.write(b"debug: FLAGS += -D DEBUG\n").unwrap();
    makefile.write(b"debug: re\n").unwrap();
    makefile.write(b"\n").unwrap();
    makefile.write(b"$(NAME): $(OBJS)\n").unwrap();
    makefile.write(b"\t@echo \"Generating $@\\c\"\n").unwrap();
    makefile.write(b"\t@$(CC) $(FLAGS) $(OBJS) -o $(NAME)\n").unwrap();
    makefile.write(b"\t@echo \" [\\033[32mOK\\033[0m]\"\n").unwrap();
    makefile.write(b"\n").unwrap();
    makefile.write(b"${BIN}/%.o: %.cpp\n").unwrap();
    makefile.write(b"\t@mkdir -p $(BIN)\n").unwrap();
    makefile.write(b"\t@echo \"[Compiling] $< -> $@\\c\"\n").unwrap();
    makefile.write(b"\t@$(CC) $(FLAGS) -c $< -o $@\n").unwrap();
    makefile.write(b"\t@echo \" [\\033[32mOK\\033[0m]\"\n").unwrap();
    makefile.write(b"\n").unwrap();
    makefile.write(b"clean:\n").unwrap();
    makefile.write(b"\t@echo \"\\033[1;31mCleaning\\033[0m objects\\c\"\n").unwrap();
    makefile.write(b"\t@rm -rf $(BIN)\n").unwrap();
    makefile.write(b"\t@echo \" [\\033[32mOK\\033[0m]\"\n").unwrap();
    makefile.write(b"\n").unwrap();
    makefile.write(b"fclean: clean\n").unwrap();
    makefile.write(b"\t@echo \"\\033[1;31mCleaning\\033[0m $(NAME)\\c\"\n").unwrap();
    makefile.write(b"\t@rm -f $(NAME)\n").unwrap();
    makefile.write(b"\t@echo \" [\\033[32mOK\\033[0m]\"\n").unwrap();
    makefile.write(b"\n").unwrap();
    makefile.write(b"re: fclean all\n").unwrap();
}
