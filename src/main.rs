use std::process::Command;
use std::process::{Stdio};
use std::io::{Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

extern crate termcolor;

mod tools;

// Definition of variables
const VERSION: f64 = 1.0; // Version of the script
const SIZEOF_TOOLS: f64 = 3.5; // Gigabytes of tools to be installed


fn main() {
    
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut buffer = Vec::new();

    writeln!(buffer, r#"
 ___  __    ________  ___       ___  ________  ___  ___  ________   ________  ________
|\  \|\  \ |\   __  \|\  \     |\  \|\   __  \|\  \|\  \|\   ___  \|\   ____\|\   __  \
\ \  \/  /|\ \  \|\  \ \  \    \ \  \ \  \|\ /\ \  \\\  \ \  \\ \  \ \  \___|\ \  \|\  \
 \ \   ___  \ \   __  \ \  \    \ \  \ \   __  \ \  \\\  \ \  \\ \  \ \  \  __\ \   __  \
  \ \  \\ \  \ \  \ \  \ \  \____\ \  \ \  \|\  \ \  \\\  \ \  \\ \  \ \  \|\  \ \  \ \  \
   \ \__\\ \__\ \__\ \__\ \_______\ \__\ \_______\ \_______\ \__\\ \__\ \_______\ \__\ \__\
    \|__| \|__|\|__|\|__|\|_______|\|__|\|_______|\|_______|\|__| \|__|\|_______|\|__|\|__|
                                                                                                
    WARNING: Must be run as root to install tools.
             I take no responsibility for any damage caused by this script.
             Use at your own risk. Intended for educational purposes only.
    ___________________________
    Designed by: @rockenman1234, Alex J. - GaTech '26 🐝
    Homepage: alexj.io
    Version: {:.1}
    sizeof(tools): {:.1} GB
"#, VERSION, SIZEOF_TOOLS).unwrap();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
    stdout.write_all(&buffer).unwrap();
    stdout.reset().unwrap();

    menu();
}

fn menu() {

    let is_root = is_user_root(); // Check whether the user is root or not

    if !is_root {
        println!("ERROR: Must be run as root to install tools.");
        return;
    } else {
        println!("User is root. Proceeding...");
    }


    println!("
    1. Install all tools
    2. Remove all tools
    3. List all tools
    4. Help
    5. Exit
    ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim().parse().unwrap();

    if input == 1 {
        install();
    } else if input == 2 {
        remove();
    } else if input == 3 {
        list();
    } else if input == 4 {
        help();
    } else if input == 5 {
        exit();
    } else {
        println!("Invalid input. Please try again.");
        menu();
    }
}

fn install() {

    println!("Installing all tools...");

    let package_names = tools::get_package_names();

    let mut dnf_process = Command::new("dnf")
        .arg("install")
        .arg("-y")
        .args(&package_names)  // Pass package names as arguments
        .stdout(Stdio::inherit())  // Pipe stdout to the parent process's stdout
        .stderr(Stdio::inherit())  // Pipe stderr to the parent process's stderr
        .spawn()
        .expect("failed to execute process");

    // Wait for the process to finish
    dnf_process.wait().expect("failed to wait for process");

    println!("All tools installed!");

    exit();
}

fn remove() {

    println!("Removing all tools...");

    let package_names = tools::get_package_names();

    let mut dnf_process = Command::new("dnf")
        .arg("remove")
        .arg("-y")
        .args(&package_names)  // Pass package names as arguments
        .stdout(Stdio::inherit())  // Pipe stdout to the parent process's stdout
        .stderr(Stdio::inherit())  // Pipe stderr to the parent process's stderr
        .spawn()
        .expect("failed to execute process");

    // Wait for the process to finish
    dnf_process.wait().expect("failed to wait for process");

    println!("All tools removed!");

    exit();
}

fn list() {
    let package_names = tools::get_package_names();

    println!("List of tools:");

    for package_name in package_names {
        println!("{}", package_name);
    }

    println!("\nWhen you're ready to continue, press Enter.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "" {
        main();
    }
}


fn help() {
    println!("
    
    Welcome to Kalibunga! This script is designed to install a variety of tools on Fedora Linux.
    The goal of this script is to make a vanilla Fedora installation more useful for ethical hacking.

    For more information on what each package does, please run 'dnf info <package_name>' or 'man <package_name>'.

    This script was created by Alex Jenkins, a student at Georgia Tech (Go Jackets! 🐝). 
    For more information about the developer, please visit https://alexj.io.

    Please consider donating to support the development of this script: https://buymeacoffee.com/alexjenkins    
    ");

    println!("\nWhen you're ready to continue, press Enter.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "" {
        main();
    }
}

fn exit() {
    println!("Exiting...");
    std::process::exit(0);
}


fn is_user_root() -> bool {
    let output = Command::new("whoami")
        .output()
        .expect("Failed to execute command");

    let username = String::from_utf8_lossy(&output.stdout);

    username.trim() == "root"
}