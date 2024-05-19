mod search;

use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use colored::Colorize;
const ERROR_MSG: &str = "Error:";

fn main() {


    const WELCOME: &str = r#"

        ╔══════════════════════════════════════════════════════════════════╗
        ║                                                                  ║
        ║                         welcome to PROTOF!                       ║
        ║           version:                            0.0.2              ║
        ║           author:                             faynot             ║
        ║                                                                  ║
        ║                                                  ©MIT License    ║
        ║                                                                  ║
        ╚══════════════════════════════════════════════════════════════════╝

"#;
    println!("{}", WELCOME.cyan().bold());
    const BASH: [&str; 8] = [
        "shutdown",
        "echo",
        "mkdir",
        "rmdir",
        "New-Item",
        "rm",
        "start",
        "taskkill"
    ];

    let mut commands: Vec<String> = Vec::new();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <command> [args]", args[0]);
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "list" => list_files(),
        "start" => {
            if args.len() < 3 {
                println!("Usage: {} start <protocol_name>", args[0]);
            } else {
                start_protocol(&args[2]);
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("Usage: {} delete <protocol_name>", args[0]);
            } else {
                delete_protocol(&args[2]);
            }
        }
        "debug" => debug(&commands),
        "help" => help(),
        "new" => new(&args[2..], &mut commands, &BASH),
        "clean" => {
            if args.len() < 3 {
                println!("Usage: {} clean <filename>", args[0]);
            } else {
                let filename = &args[2];
                let (script_name, script_extension) = extract_script_info(filename);
                let cleaning_file = vec![script_name, script_extension];
                println!("Cleaning file set to: {:?}", cleaning_file);
                run_clean_script(&cleaning_file);
            }
        },
        _ => {
            println!("{} Unknown command: {}", ERROR_MSG.bright_red(), command.bright_red());
            println!("Available commands: command1, command2");
        }
    }
}

fn extract_script_info(filename: &str) -> (String, String) {
    let path = Path::new(filename);
    let script_name = path.file_stem().and_then(|name| name.to_str()).unwrap_or("").to_string();
    let script_extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("").to_string();
    (script_name, script_extension)
}

fn run_clean_script(cleaning_file: &Vec<String>) {
    let output = Command::new("node")
        .arg("clean.js")
        .arg(&cleaning_file[0])
        .arg(&cleaning_file[1])
        .output()
        .expect("Failed to execute clean.js");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
}

fn delete_protocol(protocol_name: &str) {
    let appdata_dir = match dirs::data_local_dir() {
        Some(path) => path.join("protof"),
        None => {
            eprintln!("{} Failed to get AppData directory", ERROR_MSG.bright_red());
            return;
        }
    };

    let script_name = format!("{}.sh", protocol_name);
    let script_path = appdata_dir.join(&script_name);

    if script_path.exists() {
        match fs::remove_file(&script_path) {
            Ok(()) => println!("Protocol '{}' successfully deleted.", protocol_name),
            Err(err) => eprintln!("{} deleting protocol '{}': {}", ERROR_MSG.bright_red(), protocol_name.bright_red(), err),
        }
    } else {
        println!("{} Protocol '{}' not found.", ERROR_MSG.bright_red(), protocol_name);
    }
}
fn start_protocol(protocol_name: &str) {
    let appdata_dir = match dirs::data_local_dir() {
        Some(path) => path.join("protof"),
        None => {
            eprintln!("{} Failed to get AppData directory", ERROR_MSG.bright_red());
            return;
        }
    };

    if !appdata_dir.exists() {
        println!("No protocols found.");
        return;
    }

    let script_name = format!("{}.sh", protocol_name);
    let script_path = appdata_dir.join(&script_name);

    if script_path.exists() {
        println!("Starting protocol: {}", protocol_name);
        // Используем powershell для запуска .sh файла
        if let Err(err) = Command::new("powershell")
            .arg("-Command")
            .arg(&format!("& '{}'", script_path.to_string_lossy()))
            .status()
        {
            eprintln!("Failed to start protocol: {}", err);
        }
    } else {
        println!("Protocol '{}' not found.", protocol_name.bright_red().bold());
    }
}


fn debug(commands: &Vec<String>) {
    println!("{:?}", commands);
}

fn new(args: &[String], commands: &mut Vec<String>, bash: &[&str; 8]) {
    if args.len() != 1 {
        eprintln!("Please enter this command as - protof new <name>");
        return;
    }

    let protocol_name = args[0].trim();
    if protocol_name.is_empty() || protocol_name.chars().all(|c| c.is_whitespace()) {
        println!("Protocol name cannot be empty");
        return;
    }

    let appdata_dir = match dirs::data_local_dir() {
        Some(path) => path.join("protof"),
        None => {
            eprintln!("Failed to get AppData directory");
            return;
        }
    };

    if !appdata_dir.exists() {
        if let Err(err) = fs::create_dir(&appdata_dir) {
            eprintln!("Error creating directory: {}", err);
            return;
        }
    }

    let file_path = appdata_dir.join(format!("{}.sh", protocol_name));

    let mut file = match File::create(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating file {}: {}", file_path.display(), err);
            return;
        }
    };

    let comms: &str = r#"
        List of arguments - PROTOF
        ╔═════════════════════════════════════════════════════════════╗
        ╠ 0                =                Turn off computer         ╣
        ╠ 1                =                Write something           ╣
        ╠ 2                =                Create directory          ╣
        ╠ 3                =                Delete directory          ╣
        ╠ 4                =                Create some file          ╣
        ║                                   arg to 4 = <path>         ║
        ║                                                             ║
        ╠ 5                =                Delete some file          ╣
        ║                                   arg to 5 = <path>         ║
        ║                                                             ║
        ╠ 6                =                Open some programm        ╣
        ║                                   arg to 6 = <name > <path> ║
        ║                                                             ║
        ╠ 7                =                Close some program        ╣
        ║                                   arg to 7 = <name>.exe     ║
        ╚═════════════════════════════════════════════════════════════╝
    "#;

    println!("{}", comms.bright_magenta());
    println!("Enter command numbers (type 'quit()' to exit):");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "quit()" {
            break;
        }

        if let Ok(index) = input.trim().parse::<usize>() {
            if let Some(&command) = bash.get(index) {
                match command {
                    "start" => {
                        println!("Enter <name>, the name must match the exe file: ");
                        let mut name = String::new();
                        io::stdin().read_line(&mut name).expect("Failed to read line");

                        println!("Enter path (type 'skip' to search): ");
                        let mut path = String::new();
                        io::stdin().read_line(&mut path).expect("Failed to read line");

                        let command_line = if path.trim() == "skip" {
                            if let Ok(Some(found_path)) = search::find_exe(name.trim()) {
                                format!("start \"{}\" \"{}\"", name.trim(), found_path)
                            } else {
                                println!("Program not found");
                                continue;
                            }
                        } else {
                            format!("start \"{}\" \"{}\"", name.trim(), path.trim())
                        };

                        writeln!(file, "{}", command_line).expect("Error writing to file");
                        commands.push(command_line);
                    }
                    "echo" | "mkdir" | "rmdir" => {
                        println!("Enter a value:");
                        let mut value = String::new();
                        io::stdin().read_line(&mut value).expect("Failed to read line");

                        let command_line = format!("{} {}", command, value.trim());
                        writeln!(file, "{}", command_line).expect("Error writing to file");
                        commands.push(command_line);
                    }
                    "taskkill" => {
                        println!("Enter the name of the program you are going to close:");
                        let mut program_name = String::new();
                        io::stdin().read_line(&mut program_name).expect("Failed to read line");

                        let command_line = format!("taskkill /IM {}.exe /F", program_name.trim());
                        writeln!(file, "{}", command_line).expect("Error writing to file");
                        commands.push(command_line);
                    }
                    "rm" => {
                        println!("Enter the name and extension of file to delete:");
                        let mut rm_name = String::new();
                        io::stdin().read_line(&mut rm_name).expect("Failed to read line");

                        let command_line = format!("rm {}", rm_name.trim());
                        writeln!(file, "{}", command_line).expect("Error writing to file");
                        commands.push(command_line);
                    }
                    "New-Item" => {
                        println!("Enter the name and extension of new file:");
                        let mut newitem_name = String::new();
                        io::stdin().read_line(&mut newitem_name).expect("Failed to read line");

                        let command_line = format!("New-Item {} -ItemType file", newitem_name.trim());
                        writeln!(file, "{}", command_line).expect("Error writing to file");
                        commands.push(command_line);
                    }
                    "shutdown" => {
                        writeln!(file, "shutdown").expect("Error writing to file");
                        commands.push("shutdown".to_string());
                    }
                    _ => {
                        println!("Invalid command: {}", command);
                    }
                }
            } else {
                println!("Invalid command number: {}", index);
            }
        } else {
            println!("Invalid input, please enter a number");
        }
    }

    println!("Commands saved in file: {}", file_path.display());
}

fn help() {
    const HELP: &str = r#"
        ╔══════════════════════════════════════════════════════════════════╗
        ║                    List of commands - PROTOF                     ║
        ║   help                      =             list of commands       ║
        ║   new <name>                =             create a new protocol  ║
        ║   start <name>                            start a protocol       ║
        ║   delete <name>             =             remove a protocol      ║
        ║   list                      =             list of protocols      ║
        ║   password <name> <proto>   =             create/change password ║
        ║                                           of protocol            ║
        ║                                                                  ║
        ║   clean                     =             formate yout code      ║
        ║                                           into prettier code     ║
        ╚══════════════════════════════════════════════════════════════════╝
"#;
    println!("{}", HELP.bright_cyan());
}

fn list_files() {
    let appdata_dir = match dirs::data_local_dir() {
        Some(path) => path.join("protof"),
        None => {
            eprintln!("Failed to get AppData directory");
            return;
        }
    };

    if !appdata_dir.exists() {
        println!("No protocols found.");
        return;
    }

    println!("List of protocols:");
    let sh_files = fs::read_dir(&appdata_dir)
        .map(|entries| entries.filter_map(|entry| entry.ok()))
        .map(|entries| entries.filter(|entry| entry.path().extension().map(|ext| ext == "sh").unwrap_or(false)))
        .map(|filtered_entries| filtered_entries.map(|entry| entry.file_name().into_string().unwrap()).collect::<Vec<String>>());

    match sh_files {
        Ok(files) if files.is_empty() => println!("No .sh files found in 'protof' directory."),
        Ok(files) => {
            let max_len = files.iter().map(|file| file.len()).max().unwrap_or(0);
            println!("╔{}╗", "═".repeat(max_len + 2));
            for file in &files {
                println!("║ {:<width$} ║", file, width = max_len);
            }
            println!("╚{}╝", "═".repeat(max_len + 2));
        }
        Err(err) => eprintln!("Error checking 'protof' directory: {}", err),
    }
}