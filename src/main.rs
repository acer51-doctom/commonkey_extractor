use std::env;
use std::fs::{File, metadata};
use std::io::{self, Read, Write, Seek};
use std::path::Path;
use std::{thread, time::Duration};
use colored::*;
use std::process::Command;

const COMMON_KEY_OFFSET: u64 = 0xE0;
const COMMON_KEY_SIZE: usize = 16;
const OTP_SIZE: u64 = 1024;

fn clear_screen() {
    if cfg!(target_os = "windows") {
        // Clippy fix: needless_borrows_for_generic_args
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();
    }
}

fn extract_common_key(path: &Path) -> io::Result<[u8; COMMON_KEY_SIZE]> { // Changed signature to &Path
    let mut file = File::open(path)?;
    file.seek(io::SeekFrom::Start(COMMON_KEY_OFFSET))?;

    let mut key = [0u8; COMMON_KEY_SIZE];
    file.read_exact(&mut key)?;
    Ok(key)
}

fn is_valid_otp(path: &Path) -> bool { // Changed signature to &Path
    if path.extension().and_then(|s| s.to_str()) != Some("bin") {
        return false;
    }

    match metadata(path) { // Clippy fix: needless_borrow - `path` is already a &Path
        Ok(meta) => meta.len() == OTP_SIZE,
        Err(_) => false,
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let path_str = args[1].trim().trim_matches(['\'', '"'].as_ref());
        let path = Path::new(path_str); // Create Path reference once

        if !path.exists() {
            eprintln!("{}", "ERROR! Path does not exist.".red());
            std::process::exit(1);
        }

        // Clippy fix: needless_borrow - `path` is already a &Path
        if !is_valid_otp(path) {
            eprintln!("{}", "ERROR! The file you entered is NOT a .bin file or is not exactly 1024 bytes.".red());
            std::process::exit(1);
        }

        // Clippy fix: needless_borrow - `path` is already a &Path
        match extract_common_key(path) {
            Ok(key) => {
                print!("Your Common Key is: ");
                for byte in &key {
                    // Clippy fix: uninlined_format_args
                    print!("{byte:02X}");
                }
                println!();
            }
            Err(e) => {
                eprintln!("{} {}", "ERROR!".red(), e);
                std::process::exit(1);
            }
        }

        return Ok(());
    }

    loop {
        clear_screen();
        println!("Where is your OTP path?");
        println!("You can drag and drop it in Finder / File Explorer.");
        print!("> ");
        io::stdout().flush()?; // flush prompt

        let mut path_input = String::new();
        io::stdin().read_line(&mut path_input)?;
        let path_str = path_input.trim().trim_matches(['\'', '"'].as_ref());
        let path = Path::new(path_str); // Create Path reference once

        if !path.exists() {
            eprintln!(
                "{}",
                "ERROR! Path does not exist. Did you misspell something? Trying again in 5 seconds..."
                    .red()
            );
            thread::sleep(Duration::from_secs(5));
            continue;
        }

        // Clippy fix: needless_borrow - `path` is already a &Path
        if !is_valid_otp(path) {
            eprintln!(
                "{}",
                "ERROR! The file you entered is NOT a .bin file. Trying again in 5 seconds..."
                    .red()
            );
            thread::sleep(Duration::from_secs(5));
            continue;
        }

        // Clippy fix: needless_borrow - `path` is already a &Path
        match extract_common_key(path) {
            Ok(key) => {
                println!("\nWii U Common Key:");
                for byte in &key {
                    // Clippy fix: uninlined_format_args
                    print!("{byte:02X}");
                }
                println!("\n");
                println!("Press Ctrl+C to quit...");
            }
            Err(e) => {
                eprintln!("{} {}", "ERROR!".red(), e);
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        }

        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }
}