use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::io::{BufRead, BufReader};

pub fn write_to_bash_history(file_to_export_to: &mut std::fs::File, command: String) {
    let time_string = get_current_time();
    let command = format!("#{} {}\n", time_string, command);
    if let Err(e) = file_to_export_to.write_all(command.as_bytes()) {
        eprintln!("Failed to write to bash history file: {}", e);
    }
}

pub fn get_current_time() -> String {
    let now = chrono::Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_string_from_file(file: &mut File, param_index: i32) -> String {
    println!(
        "--- Get String from file called with index: {}",
        param_index
    );

    file.seek(std::io::SeekFrom::Start(0)).unwrap();

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap_or_default()).collect();

    if param_index <= 0 || param_index as usize > lines.len() {
        eprintln!("Invalid index: {}", param_index);
        return String::new();
    }

    let line = &lines[param_index as usize - 1];
    println!("--- Get String from file returning: {}", line);
    line.clone()
}

pub fn append_string_to_file(file: &mut File, string: String) -> io::Result<()> {
    println!("--- Append String to file called with: {}", string);

    file.seek(SeekFrom::Start(0))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut lines: Vec<&str> = content.lines().collect();

    if lines.len() >= 500 {
        lines.remove(0);
    }

    lines.push(&string);

    if lines.len() > 500 {
        lines = lines[lines.len() - 500..].to_vec();
    }

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    file.flush()?;

    println!("File updated successfully");
    Ok(())
}
