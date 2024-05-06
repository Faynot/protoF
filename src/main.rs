use std::env;
use std::fs::File;
use std::io::{self, Write};

fn main() {
    const BASH: [&str; 7] = [
        "shutdown",
        "echo",
        "open", // not implemented
        "close", // not implemented
        "create", // not implemented
        "delete", // not implemented
        "test" // not implemented
    ];

    let mut commands: Vec<String> = Vec::new();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <command> [args]", args[0]);
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "debug" => debug(&commands),
        "help" => help(),
        "new" => new(&args[2..], &mut commands, &BASH),
        _ => {
            println!("Unknown command: {}", command);
            println!("Available commands: command1, command2");
        }
    }
}

fn debug(commands: &Vec<String>) {
    println!("{:?}", commands);
}

fn new(args: &[String], commands: &mut Vec<String>, bash: &[&str; 7]) {
    if args.len() != 1 {
        println!("Please enter this command as - protof new <name>");
    } else if !args[0].is_empty() && !args[0].chars().all(|c| c.is_whitespace()) {
        println!("Creating a new protocol with name: {}", args[0]);
        let mut file = match File::create(format!("{}.sh", args[0])) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Error creating file {}.sh: {}", args[0], err);
                return;
            }
        };
        let comms: &str = r#"
              List of arg - PROTOF
        0                =                Turn off computer
        1                =                Write something
        2                =                Open some program
        3                =                Close some program
        4                =                Create some file
        5                =                Delete some file
        6                =                idk
        "#;
        println!("{}", comms);
        println!("Enter command numbers (type 'quit()' to exit):");

        loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Failed to read line");

            if input.trim() == "quit()" {
                break;
            }

            if let Ok(index) = input.trim().parse::<usize>() {
                if index < bash.len() {
                    let command = bash[index];
                    if command == "echo" {
                        println!("Enter arg to {}: ", index);
                        let mut arg = String::new();
                        io::stdin().read_line(&mut arg).expect("Failed to read line");
                        writeln!(file, "{} \"{}\"", command, arg.trim()).expect("Error writing to file");
                        commands.push(format!("{} \"{}\"", command, arg.trim()));
                    } else {
                        writeln!(file, "{}", command).expect("Error writing to file");
                        commands.push(command.to_string());
                    }
                } else {
                    println!("Invalid command number: {}", index);
                }
            } else {
                println!("Invalid input, please enter a number");
            }
        }
        println!("Commands saved in file: {}.sh", args[0]);
    } else {
        println!("Protocol name cannot be empty");
    }
}

fn help() {
    let help: &str = r#"
              List of commands - PROTOF
help                      =             list of commands
new <name>                =             create a new protocol
remove <name>             =             remove a protocol
list                      =             list of protocols
password <name> <proto>   =             create/change password
                                        of protocol
"#;
    println!("{}", help);
}


//      Добавление элементов разных типов в вектор
// my_vector.push(serde_json::Value::String("Hello".to_string()));
// my_vector.push(serde_json::Value::Number(serde_json::Number::from(42)));
// my_vector.push(serde_json::Value::Bool(true));

// "example" => example(&args[2..]),

// fn example(args: &[String]) {
//      println!("Command 1 executed with args: {:?}", args);
// }
