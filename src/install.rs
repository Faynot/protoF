use std::process::Command;

pub fn install(program_name: &str) {
    // Определение операционной системы
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "unknown"
    };

    // Массивы для имен
    let original_names = [
        "git",
        "firefox",
        "code",
        "vim",
        "gzip",
        "unzip",
        "fastfetch",
    ];

    let windows_names = [
        "Git.Git",
        "firefox",
        "Microsoft.VisualStudioCode",
        "Vim",
        "gzip",
        "unzip",
        "fastfetch",
    ];

    let linux_names = [
        "git",
        "firefox-developer",
        "vscode",
        "vim",
        "gzip",
        "unzip",
        "fastfetch",
    ];
    let macos_names = [
        "git",
        "firefox-developer",
        "vscode",
        "vim",
        "gzip",
        "unzip",
        "fastfetch",
    ];

    // Проверка, нужно ли изменять имя программы
    let mut install_name = program_name.to_string();
    match os {
        "windows" => {
            for (i, name) in original_names.iter().enumerate() {
                if program_name == *name {
                    install_name = windows_names[i].to_string();
                    break;
                }
            }
        }
        "linux" => {
            for (i, name) in original_names.iter().enumerate() {
                if program_name == *name {
                    install_name = linux_names[i].to_string();
                    break;
                }
            }
        }
        "macos" => {
            for (i, name) in original_names.iter().enumerate() {
                if program_name == *name {
                    install_name = macos_names[i].to_string();
                    break;
                }
            }
        }
        _ => {}
    }

    match os {
        "windows" => {
            // Использование winget
            let output = Command::new("winget")
                .arg("install")
                .arg(&install_name)
                .output()
                .expect("Failed to execute command");
            println!("Output: {}", String::from_utf8_lossy(&output.stdout));
            println!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
        "linux" => {
            // Использование apt install или pacman в зависимости от дистрибутива
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!("if [ -f /etc/apt/sources.list ]; then sudo apt install {}; else sudo pacman -S {}; fi", &install_name, &install_name))
                .output()
                .expect("Failed to execute command");
            println!("Output: {}", String::from_utf8_lossy(&output.stdout));
            println!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
        "macos" => {
            // Использование brew (предполагается, что brew уже установлен)
            let output = Command::new("brew")
                .arg("install")
                .arg(&install_name)
                .output()
                .expect("Failed to execute command");
            println!("Output: {}", String::from_utf8_lossy(&output.stdout));
            println!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
        _ => {
            println!("Unsupported operating system");
        }
    }
}
