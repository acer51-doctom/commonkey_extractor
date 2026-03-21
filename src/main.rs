mod core;
mod gui;

use colored::*;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::{thread, time::Duration};

fn clear_screen() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();
    }
}

fn main() -> io::Result<()> {
    // Set up the Ctrl+C handler for a graceful exit message
    ctrlc::set_handler(move || {
        println!("\n\n{}", "Exiting... Thanks for using the Wii U Common Key Extractor!".yellow().bold());
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    let args: Vec<String> = env::args().collect();

    // 1. If no arguments are passed, launch the GUI by default
    if args.len() == 1 {
        gui::launch_gui(); //
        return Ok(());
    }

    // 2. Check for the TUI / Command Line Mode flags
    let first_arg = args[1].to_lowercase();
    if first_arg == "--tui" || first_arg == "--command-line-mode" {
        return run_tui_loop();
    }

    // 3. Otherwise, treat the argument as a direct path to an otp.bin file
    let path_str = args[1].trim().trim_matches(['\'', '"'].as_ref());
    let path = Path::new(path_str);

    if !path.exists() {
        eprintln!("{}", "ERROR! Path does not exist.".red());
        std::process::exit(1);
    }

    if !core::is_valid_otp(path) { //
        eprintln!(
            "{}",
            format!("ERROR! File must be a .bin and exactly {} bytes.", core::OTP_SIZE).red() //
        );
        std::process::exit(1);
    }

    match core::extract_common_key(path) { //
        Ok(key) => {
            print!("Your Common Key is: ");
            for byte in &key {
                print!("{byte:02X}");
            }
            println!();
        }
        Err(e) => {
            eprintln!("{} {}", "ERROR!".red(), e);
            std::process::exit(1);
        }
    }

    Ok(())
}

/// The interactive terminal loop
fn run_tui_loop() -> io::Result<()> {
    loop {
        clear_screen();
        println!("{}", "--- Wii U OTP Interactive Mode ---".cyan().bold());
        println!("Please enter the path to your otp.bin (or drag and drop the file):");
        print!("> ");
        io::stdout().flush()?;

        let mut path_input = String::new();
        io::stdin().read_line(&mut path_input)?;
        let path_str = path_input.trim().trim_matches(['\'', '"'].as_ref());
        let path = Path::new(path_str);

        if !path.exists() {
            eprintln!("{}", "ERROR! Path does not exist. Retrying in 3 seconds...".red());
            thread::sleep(Duration::from_secs(3));
            continue;
        }

        if !core::is_valid_otp(path) { //
            eprintln!("{}", "ERROR! Invalid .bin file. Retrying in 3 seconds...".red());
            thread::sleep(Duration::from_secs(3));
            continue;
        }

        match core::extract_common_key(path) { //
            Ok(key) => {
                println!("\n{}", "Success! Wii U Common Key:".green());
                for byte in &key {
                    print!("{byte:02X}");
                }
                println!("\n\nPress Ctrl+C to quit or enter a new path.");
                // Brief pause so the user can actually see the key before a potential clear_screen
                thread::sleep(Duration::from_secs(2)); 
            }
            Err(e) => {
                eprintln!("{} {}", "ERROR!".red(), e);
                thread::sleep(Duration::from_secs(3));
            }
        }
    }
}