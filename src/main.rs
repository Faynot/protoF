// Hello, its my program for create a protocols!
// i created this program at 4 hours

// lib
use std::env;
use std::fs::File;
use std::io::{self, Write};

// main function, to protof commands
fn main() {
    // an array that stores shell commands
    const BASH: [&str; 7] = [
        "shutdown",
        "echo",
        "mkdir",
        "rmdir",
        "del",
        "copy",
        "start"
    ];

    // commans vector
    let mut commands: Vec<String> = Vec::new();
    let args: Vec<String> = env::args().collect();

    // if user input a command not correct - output this
    if args.len() < 2 {
        println!("Usage: {} <command> [args]", args[0]);
        return;
    }

    // commands
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

// idk lol
fn debug(commands: &Vec<String>) {
    println!("{:?}", commands);
}

// command "new"
fn new(args: &[String], commands: &mut Vec<String>, bash: &[&str; 7]) {
    // checking for data correctness or creating a sh script
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

        // list of arguments
        let comms: &str = r#"
                           List of arg - PROTOF
        ╔═════════════════════════════════════════════════════════════╗
        ╠ 0                =                Turn off computer         ╣
        ╠ 1                =                Write something           ╣
        ╠ 2                =                Create directory          ╣
        ╠ 3                =                Delete directory          ╣
        ╠ 4                =                Delete some file          ╣
        ║                                   arg to 4 = <path>         ║
        ║                                                             ║
        ╠ 5                =                Copy some file            ╣
        ║                                   arg to 5 = <path>         ║
        ║                                                             ║
        ╠ 6                =                Open some programm        ╣
        ║                                   arg to 6 = <name > <path> ║
        ║                                                             ║
        ╠ 7                =                Close some program        ╣
        ║                                   arg to 7 = <name>.exe     ║
        ╚═════════════════════════════════════════════════════════════╝
        "#;
        println!("{}", comms);
        println!("Enter command numbers (type 'quit()' to exit):");

        // fucking fuck machine of hell
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            if input.trim() == "quit()" {
                break;
            }

            if let Ok(index) = input.trim().parse::<usize>() {
                if index < bash.len() {
                    let command = bash[index];
                    let mut command_line = String::new();
                    if command == "start" {
                        println!("Enter name: ");
                        let mut name = String::new();
                        io::stdin().read_line(&mut name).expect("Failed to read line");

                        println!("Enter path: ");
                        let mut path = String::new();
                        io::stdin().read_line(&mut path).expect("Failed to read line");

                        command_line = format!("start \"{}\" \"{}\"", name.trim(), path.trim());
                    } else if command == "echo" || command == "mkdir" || command == "rmdir" || command == "del" || command == "copy" {
                        println!("Enter arg to {}: ", index);
                        let mut arg = String::new();
                        io::stdin().read_line(&mut arg).expect("Failed to read line");
                        command_line = format!("{} \"{}\"", command, arg.trim());
                    } else {
                        command_line = command.to_string();
                    }
                    writeln!(file, "{}", command_line).expect("Error writing to file");
                    commands.push(command_line);
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



// help commans
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


//                             idk

//                   add elements to vector
// my_vector.push(serde_json::Value::String("Hello".to_string()));
// my_vector.push(serde_json::Value::Number(serde_json::Number::from(42)));
// my_vector.push(serde_json::Value::Bool(true));


//                  add commands
// "example" => example(&args[2..]),

// fn example(args: &[String]) {
//      println!("Command 1 executed with args: {:?}", args);
// }
