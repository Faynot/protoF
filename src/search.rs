use std::io;
use walkdir::WalkDir;

pub fn find_exe(name: &str) -> io::Result<Option<String>> {
    for entry in WalkDir::new("C:\\").into_iter().filter_map(|e| e.ok()) {
        if let Some(file_name) = entry.file_name().to_str() {
            if file_name.to_lowercase().contains(&name.to_lowercase()) && file_name.ends_with(".exe") {
                if let Some(path) = entry.path().to_str() {
                    println!("Search complete! path - {}", path); // Выводим сообщение после нахождения пути
                    return Ok(Some(path.to_string()));
                }
            }
        }
    }
    Ok(None)
}


fn main() {
    println!("Enter name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let name = input.trim();

    match find_exe(name) {
        Ok(Some(path)) => println!("Path to {}: {}", name, path),
        Ok(None) => println!("Program not found"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
