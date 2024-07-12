mod search;
pub mod commandline;
pub mod editor;
pub mod keymapper;
pub mod render;
pub mod util;

use commandline::{argparser, from_path};
use editor::{Editor, Mode};
use keymapper::*;
use render::*;
use util::usub;

use ropey::Rope;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write, Read, stdout, Stdout};
use std::path::Path;
use std::process::Command;
use colored::Colorize;
use crossterm::{event::{self}};
use std::time::Duration;
const ERROR_MSG: &str = "Error:";

fn main() {
    const WELCOME: &str = r#"
        ╔══════════════════════════════════════════════════════════════════╗
        ║                                                                  ║
        ║                         welcome to PROTOF!                       ║
        ║           version:                            0.0.3              ║
        ║           author:                             faynot             ║
        ║                                                                  ║
        ║                                                  ©MIT License    ║
        ║                                                                  ║
        ╚══════════════════════════════════════════════════════════════════╝
    "#;
    println!("{}", WELCOME.cyan().bold());

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
        "debug" => debug(),
        "help" => help(),
        "editor" => {
            if args.len() < 3 {
                println!("Usage: {} editor <filename>", args[0]);
            } else {
                let filename = args[2].clone();
                if let Err(err) = vim(Some(filename)) {
                    eprintln!("{} Error: {:?}", ERROR_MSG.bright_red(), err);
                }
            }
        }
        "new" => new(&args[2..]),
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
            println!("Available commands: list, start, delete, debug, help, new, clean, open");
        }
    }
}

fn vim(file_path: Option<String>) -> crossterm::Result<()> {
    let mut writer = stdout();
    let (rope, path) = commandline::from_path(file_path);
    let mut editor = Editor::new(rope, path);
    let key_map = key_builder();
    render_enter_alt_screen(&mut writer);
    render(&mut writer, &editor);
    while editor.is_running {
        if event::poll(Duration::from_millis(50))? {
            if let event::Event::Key(key) = event::read()? {
                if let Some(handle) = key_map.get_mapping(&editor.mode, &key) {
                    handle(&mut editor);
                }
            }
            render(&mut writer, &editor);
        }
    }
    render_exit_alt_screen(&mut writer);
    Ok(())
}

fn extract_script_info(filename: &str) -> (String, String) {
    let path = Path::new(filename);
    let script_name = path.file_stem().and_then(|name| name.to_str()).unwrap_or("").to_string();
    let script_extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("").to_string();
    (script_name, script_extension)
}

fn run_clean_script(cleaning_file: &Vec<String>) {
    let current_exe = env::current_exe().expect("Failed to get current executable path");
    let exe_dir = current_exe.parent().expect("Failed to get parent directory of executable");
    let clean_script_path = exe_dir.join("clean.js");

    let output = Command::new("node")
        .arg(clean_script_path)
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

fn debug() {
    println!("Debugging commands not implemented.");
}


fn new(args: &[String]) {
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
        ╠ 6                =                Open some program         ╣
        ║                                   arg to 6 = <name> <path>  ║
        ║                                                             ║
        ╠ 7                =                Close some program        ╣
        ║                                   arg to 7 = <name>.exe     ║
        ╚═════════════════════════════════════════════════════════════╝
    "#;

    println!("{}", comms.bright_magenta());
    println!("Enter command numbers (type 'quit()' to exit):");

    let mut commands: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "quit()" {
            break;
        }

        if let Ok(index) = input.trim().parse::<usize>() {
            match index {
                0 => commands.push("shutdown /s".to_string()),
                1 => {
                    println!("Enter text to write: ");
                    let mut text = String::new();
                    io::stdin().read_line(&mut text).expect("Failed to read line");
                    commands.push(format!("echo {}", text.trim()));
                }
                2 => {
                    println!("Enter directory path: ");
                    let mut path = String::new();
                    io::stdin().read_line(&mut path).expect("Failed to read line");
                    commands.push(format!("mkdir {}", path.trim()));
                }
                3 => {
                    println!("Enter directory path: ");
                    let mut path = String::new();
                    io::stdin().read_line(&mut path).expect("Failed to read line");
                    commands.push(format!("rmdir {}", path.trim()));
                }
                4 => {
                    println!("Enter file path: ");
                    let mut path = String::new();
                    io::stdin().read_line(&mut path).expect("Failed to read line");
                    commands.push(format!("New-Item -ItemType File {}", path.trim()));
                }
                5 => {
                    println!("Enter file path: ");
                    let mut path = String::new();
                    io::stdin().read_line(&mut path).expect("Failed to read line");
                    commands.push(format!("rm {}", path.trim()));
                }
                6 => {
                    println!("Enter program name: ");
                    let mut name = String::new();
                    io::stdin().read_line(&mut name).expect("Failed to read line");
                    println!("Enter program path (or type 'skip' to search): ");
                    let mut path = String::new();
                    io::stdin().read_line(&mut path).expect("Failed to read line");
                    let path = path.trim();
                    if path == "skip" {
                        match search::find_exe(name.trim()) {
                            Ok(Some(found_path)) => commands.push(format!("start \"{}\" \"{}\"", name.trim(), found_path)),
                            Ok(None) => println!("Program not found"),
                            Err(err) => eprintln!("Error: {}", err),
                        }
                    } else {
                        commands.push(format!("start \"{}\" \"{}\"", name.trim(), path));
                    }
                }
                7 => {
                    println!("Enter process name: ");
                    let mut proc_name = String::new();
                    io::stdin().read_line(&mut proc_name).expect("Failed to read line");
                    commands.push(format!("taskkill /IM {} /F", proc_name.trim()));
                }
                _ => println!("Invalid command number."),
            }
        } else {
            println!("Invalid input.");
        }
    }

    for command in commands {
        if let Err(err) = writeln!(file, "{}", command) {
            eprintln!("Error writing to file {}: {}", file_path.display(), err);
            return;
        }
    }

    println!("Script saved to {}", file_path.display());
}



fn list_files() {
    let appdata_dir = match dirs::data_local_dir() {
        Some(path) => path.join("protof"),
        None => {
            eprintln!("Failed to get AppData directory");
            return;
        }
    };

    if let Ok(entries) = fs::read_dir(appdata_dir) {
        println!("List of protocols:");
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name() {
                        if let Some(name_str) = name.to_str() {
                            println!("{}", name_str);
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read directory");
    }
}

fn help() {
    const HELP: &str = r#"
        ╔══════════════════════════════════════════════════════════════════╗
        ║                         protof help                              ║
        ╠══════════════════════════════════════════════════════════════════╣
        ║ list                   =   list all files                        ║
        ║ new <name>             =   create new protocol                   ║
        ║ start <name>           =   start existing protocol               ║
        ║ delete <name>          =   delete protocol                       ║
        ║ help                   =   show help menu                        ║
        ║ debug                  =   show all commands in protocol         ║
        ║ clean <filename>       =   clean script                          ║
        ║ editor <filename>      =   open file in text editor (like vim)   ║
        ╚══════════════════════════════════════════════════════════════════╝
    "#;
    println!("{}", HELP.bright_yellow());
}
