use std::fs::read_to_string;
use std::io::Write;

pub fn new_class(args: Vec<String>) {
    if !std::path::Path::new("Makefile").exists() {
        eprintln!("Makefile not found");
        eprintln!("Please make sure you are in the root of the exercise");
        return;
    }
    let class_name: String;
    let mut class_directory: String;
    match args.len() {
        3 => {
            class_name = args[2].clone();
            class_directory = String::from("./");
        },
        4 => {
            class_name = args[2].clone();
            let d = std::path::Path::new(&args[3]);
            if !d.exists() || !d.is_dir() {
                eprintln!("{} is not a valid directory", args[3]);
                return;
            }
            class_directory = String::from(args[3].clone());
            if !class_directory.ends_with("/") {
                class_directory.push_str("/");
            }
        },
        _ => {
            eprintln!("Usage: {} {} <class_name> [class_directory]", args[0], args[1]);
            return;
        }
    }
    if class_name.ends_with(".cpp") {
        eprintln!("Class name should not end with .cpp");
        return;
    }
    create_class(&class_name, &class_directory);
    update_makefile(&class_name, &class_directory);
}

fn create_class(class_name: &str, class_directory: &str) {
    println!("Creating class {} in {}", class_name, class_directory);
    // let class_to_variable_name = |s: &str| {
    //     let mut new_s = s.to_string();
    //     if let Some(c) = new_s.chars().next() {
    //         if c.is_uppercase() {
    //             new_s[0..1].make_ascii_lowercase();
    //         }
    //     }
    //     new_s
    // };
    // let variable_name = class_to_variable_name(class_name);
    let cpp_file_name = format!("{}{}.cpp", class_directory, class_name);
    println!("Creating template for {}", cpp_file_name);
    let mut cpp_file = match std::fs::File::create(&cpp_file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file {}: {}", &cpp_file_name, e);
            return;
        }
    };
    let hpp_file_name = format!("{}{}.hpp", class_directory, class_name);
    println!("Creating template for {}", hpp_file_name);
    let mut hpp_file = match std::fs::File::create(&hpp_file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file {}: {}", &hpp_file_name, e);
            return;
        }
    };
    hpp_file.write(b"#pragma once\n").unwrap();
    hpp_file.write(format!("#ifndef {}_HPP\n", class_name.to_uppercase()).as_bytes()).unwrap();
    hpp_file.write(format!("# define {}_HPP\n", class_name.to_uppercase()).as_bytes()).unwrap();
    hpp_file.write(b"\n").unwrap();
    hpp_file.write(b"//# include <iostream>\n").unwrap();
    hpp_file.write(b"\n").unwrap();
    hpp_file.write(format!("class {} {{\n", class_name).as_bytes()).unwrap();
    hpp_file.write(b"\tpublic:\n").unwrap();
    hpp_file.write(format!("\t\t{}();\n", class_name).as_bytes()).unwrap();
    hpp_file.write(format!("\t\t~{}();\n", class_name).as_bytes()).unwrap();
    hpp_file.write(format!("\t\t{}({} const & src);\n", class_name, class_name).as_bytes()).unwrap();
    hpp_file.write(format!("\t\t{} & operator=({} const & rhs);\n", class_name, class_name).as_bytes()).unwrap();
    hpp_file.write(b"\n").unwrap();
    hpp_file.write(b"\t//private:\n").unwrap();
    hpp_file.write(b"\t//protected:\n").unwrap();
    hpp_file.write(b"};\n").unwrap();

    cpp_file.write(format!("#include \"{}.hpp\"\n", class_name).as_bytes()).unwrap();
    cpp_file.write(b"\n").unwrap();
    eprintln!("TODO: Implement cpp template");
}

fn update_makefile(class_name: &str, class_directory: &str) {
    let cpp_file_name = format!("{}{}.cpp", class_directory, class_name);
    let makefile_name = format!("{}Makefile", class_directory);
    println!("Adding rule in {} to compile {}", makefile_name, cpp_file_name);
    let makefile = match read_to_string(&makefile_name) {
        Ok(makefile) => makefile,
        Err(e) => {
            eprintln!("Failed to read {}: {}", &makefile_name, e);
            return;
        }
    };
    let mut makefile_lines = makefile.lines().map(String::from).collect();
    let mut executable_rule_index = None;
    for (i, line) in makefile_lines.iter().enumerate() {
        if line.starts_with("SRCS") {
            executable_rule_index = Some(i);
            break;
        }
    }
    if executable_rule_index.is_none() {
        eprintln!("Failed to find the rule for the executable in {}", &makefile_name);
        return;
    }
    let mut new_rule_index = executable_rule_index.unwrap();
    while new_rule_index < makefile_lines.len() && makefile_lines[new_rule_index].ends_with("\\") {
        new_rule_index += 1;
    }
    // TODO add \\
    // TODO add rule
}
