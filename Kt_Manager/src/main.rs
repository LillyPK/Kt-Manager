use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const COLOR1: &str = "\x1b[38;2;255;202;87m"; // #ffca57
const COLOR2: &str = "\x1b[38;2;89;122;255m"; // #597aff
const RESET: &str = "\x1b[0m";
const CLEAR_SCREEN: &str = "\x1b[2J\x1b[H"; // ANSI escape code to clear the screen

fn main() {
    let scripts_dir = "scripts";

    // Check if the scripts folder exists
    if !Path::new(scripts_dir).exists() {
        println!("The 'scripts' folder does not exist. Please create it and add your .ps1 files.");
        return;
    }

    // Get all .ps1 files in the scripts folder
    let scripts = fs::read_dir(scripts_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("ps1") {
                Some(path.file_stem()?.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    // If no scripts are found, exit
    if scripts.is_empty() {
        println!("No .ps1 scripts found in the 'scripts' folder.");
        return;
    }

    loop {
        // Clear the screen
        print!("{}", CLEAR_SCREEN);
        io::stdout().flush().unwrap();

        // Display the menu with alternating colors
        println!("Select a script to run (or multiple scripts separated by spaces):");
        for (i, script) in scripts.iter().enumerate() {
            let color = if i % 2 == 0 { COLOR1 } else { COLOR2 };
            println!("{}{}. {}{}", color, i + 1, script, RESET);
        }
        println!("Type 'exit' to quit.");

        // Get user input
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Handle exit command
        if input.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        // Parse user input
        let selections: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        // Validate selections
        if selections.is_empty() {
            println!("Invalid input. Please enter valid option numbers.");
            continue;
        }

        // Launch selected scripts
        for &selection in &selections {
            if selection > 0 && selection <= scripts.len() {
                let script_name = &scripts[selection - 1];
                let script_path = PathBuf::from(scripts_dir).join(format!("{}.ps1", script_name));

                println!("Launching script: {}", script_name);

                // Use `cmd.exe /c start` to open a new window for each PowerShell script
                let status = Command::new("cmd.exe")
                    .args(&[
                        "/c",
                        "start",
                        "powershell",
                        "-NoProfile",
                        "-ExecutionPolicy",
                        "Bypass",
                        "-File",
                        script_path.to_str().unwrap(),
                    ])
                    .spawn();

                match status {
                    Ok(_) => println!(
                        "Script '{}' launched successfully in a new window.",
                        script_name
                    ),
                    Err(e) => println!("Failed to launch script '{}': {}", script_name, e),
                }
            } else {
                println!(
                    "Invalid option: {}. Please enter a number between 1 and {}.",
                    selection,
                    scripts.len()
                );
            }
        }

        // Pause to let the user see the output before clearing the screen
        println!("\nPress Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
    }
}
