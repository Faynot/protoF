use std::{io, thread, time::Duration};
use walkdir::WalkDir;
use std::io::Write; // Импортируем трейт Write для использования метода flush
use colored::*; // Импортируем библиотеку colored

pub fn find_exe(name: &str) -> io::Result<Option<String>> {
    // Функция для анимации загрузки
    fn animate_loading(running: &std::sync::Arc<std::sync::atomic::AtomicBool>) {
        let frames = vec![
            "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"
        ];
        let green_frames: Vec<String> = frames.iter().map(|frame| frame.green().to_string()).collect();
        for frame in green_frames.iter().cycle() {
            if !running.load(std::sync::atomic::Ordering::SeqCst) {
                break;
            }
            print!("\r{} Searching for file...", frame);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        print!("\r"); // Очистка строки после завершения анимации
    }

    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let running_clone = running.clone();

    // Запуск анимации загрузки в отдельном потоке
    let handle = thread::spawn(move || {
        animate_loading(&running_clone);
    });

    for entry in WalkDir::new("C:\\").into_iter().filter_map(|e| e.ok()) {
        if let Some(file_name) = entry.file_name().to_str() {
            if file_name.to_lowercase().contains(&name.to_lowercase()) && file_name.ends_with(".exe") {
                if let Some(path) = entry.path().to_str() {
                    running.store(false, std::sync::atomic::Ordering::SeqCst);
                    handle.join().unwrap(); // Ожидаем завершения анимации
                    println!("Search complete! path - {}", path); // Выводим сообщение после нахождения пути
                    return Ok(Some(path.to_string()));
                }
            }
        }
    }

    running.store(false, std::sync::atomic::Ordering::SeqCst);
    handle.join().unwrap(); // Ожидаем завершения анимации
    println!("Search complete! No such file found."); // Выводим сообщение, если файл не найден
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
