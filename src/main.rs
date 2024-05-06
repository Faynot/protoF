use std::env;
use std::fs::File;
use std::io::{self, Write};

fn main() {
    const bash: [&str; 7] = [
        "shutdown",
        "timer",
        "open",
        "close",
        "create",
        "delete",
        "test"
    ];

    let mut protos: Vec<serde_json::Value> = Vec::new();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <command> [args]", args[0]);
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "debug" => debug(&mut protos),
        "help" => help(),
        "new" => new(&args[2..], &mut protos),
        _ => {
            println!("Unknown command: {}", command);
            println!("Available commands: command1, command2");
        }
    }
}

fn debug(protos: &mut Vec<serde_json::Value>) {
    println!("{:?}", protos);
}

fn new(args: &[String], protos: &mut Vec<serde_json::Value>) {
    if args.len() != 1 {
        println!("Please enter this command as - protof new <name>");
    } else if !args[0].is_empty() && !args[0].chars().all(|c| c.is_whitespace()) {
        println!("Creating a new protocol with name: {}", args[0]);
        protos.push(serde_json::Value::String(args[0].clone()));

        let file_name = format!("{}.json", args[0]);
        let mut file = match File::create(&file_name) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Error creating file {}: {}", file_name, err);
                return;
            }
        };

        println!("Enter commands (type 'quit()' to exit):");

        loop {
            let mut command = String::new();
            io::stdin().read_line(&mut command).expect("Failed to read line");

            if command.trim() == "quit()" {
                break;
            }

            if let Err(err) = writeln!(file, "{}", command.trim()) {
                eprintln!("Error writing to file {}: {}", file_name, err);
                return;
            }
        }
        println!("Commands saved in file: {}", file_name);
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