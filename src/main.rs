use std::{
    fs::{self, File},
    io::{stdin, stdout, Write},
    sync::Arc,
    thread,
};
use colored::*;
use clearscreen::clear;

fn main() {
    println!("{}", "Welcome to Nekolker - Advanced File Encryption!".yellow());
    println!("{}", "Please choose one of the following options:".yellow());
    print!("(1) Encrypt\n(2) Decrypt\n(3) Read Important Information\n: ");
    stdout().flush().unwrap();
    let mut choice = String::new();
    stdin().read_line(&mut choice).unwrap();

    println!("{}", "Enter your password. If you are decrypting, enter the decryption password key. If encrypting, enter the encryption password key. If unsure, just press Enter:".green());
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let password = password.trim().to_string();

    loop {
        match choice.trim() {
            "3" => {
                clear().unwrap();
                println!("{}", "THIS IS EXTREMELY IMPORTANT INFORMATION!".bold().bright_green());
                println!("{}", "If you encounter issues with the decrypted file, it could be due to:".bright_green());
                println!("{}", "1. Manual editing resulting in missing data blocks.".bright_green());
                println!("{}", "2. Using the wrong decryption key.".bright_green());
                println!("{}", "3.  Do Not Forget Your Key After Encryption!!!".bright_green());
                println!("*****************************************************************");
                println!("{}{}", "If you are encrypting a large file, it will take some time.".bright_green(), "Please be patient!!".bright_red());
                println!("*PRESS ENTER TO CONTINUE*");
                println!("*****************************************************************");
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                stdout().flush().unwrap();
                restrt();

                //continue;
                ////main();
            }
            "1" => {
                println!("{}", "Nekolker - Encrypt File");
                print!("Enter the name of the file you want to encrypt (including its extension): ");
                stdout().flush().unwrap();
                let mut file_name = String::new();
                stdin().read_line(&mut file_name).unwrap();
                let file_name = file_name.trim().to_string();
                let password_arc = Arc::new(password.clone());

                let handle = thread::spawn(move || {
                    let input_data = fs::read(&file_name).unwrap();
                    let encrypted_data: Vec<u8> = input_data
                        .iter()
                        .map(|&byte| {
                            let seed = password_arc
                                .as_bytes()
                                .iter()
                                .fold(0, |acc: u8, &byte| acc.wrapping_add(byte));
                            byte ^ seed + 1
                        })
                        .collect();
                    let output_file = file_name.to_string() + ".neko";
                    let mut file = File::create(output_file).unwrap();
                    file.write_all(&encrypted_data).unwrap();
                    print!("{}{:?}{}","File Encrypted At - ", file, " ");
                    print!("{}{:?}{}","With Password - ", password_arc, "\n");
                    stdout().flush().unwrap();
                });

                handle.join().unwrap();
                //main();
                stdout().flush().unwrap();
                println!("{}","*\n\npress enter*".red());
                let mut buf = String::new();
                stdin().read_line(&mut buf).unwrap();
                restrt();
            }
            "2" => {
                print!("{}", "Enter the input file name you want to decrypt (without extension): ".bright_green());
                stdout().flush().unwrap();
                let mut file_name = String::new();
                let mut output_path = String::new();
                stdin().read_line(&mut file_name).unwrap();
                print!("Enter the output folder path (not file): ");
                stdout().flush().unwrap();
                stdin().read_line(&mut output_path).unwrap();
                let file_name = file_name.trim().to_string();
                let password_arc = Arc::new(password.clone());

                let handle = thread::spawn(move || {
                    let input_data = fs::read(&file_name).unwrap();
                    let decrypted_data: Vec<u8> = input_data
                        .iter()
                        .map(|&byte| {
                            let seed = password_arc
                                .as_bytes()
                                .iter()
                                .fold(0, |acc: u8, &byte| acc.wrapping_add(byte));
                            byte ^ seed + 1
                        })
                        .collect();
                    let mut file_extension = String::new();
                    print!("{}", "Please input the file extension to save the decrypted file in (e.g., .mp4): ");
                    stdout().flush().unwrap();
                    stdin().read_line(&mut file_extension).unwrap();
                    let output_file = output_path.trim().to_string() + "/" + &file_name + &file_extension.trim();
                    let mut file = File::create(output_file).unwrap();
                    file.write_all(&decrypted_data).unwrap();
                    print!("{}{:?}{}","File Decrypted At - ", file, " ");
                    print!("{}{:?}{}","With Password - ", password_arc, "\n");
                    stdout().flush().unwrap();
                });

                handle.join().unwrap();
                //main();
                stdout().flush().unwrap();
                restrt();
            }
            _ => {
                println!("{}", "Please choose correctly!".red());
                //exit(1);
                println!("{}","*press enter*".red());
                let mut buf = String::new();
                stdin().read_line(&mut buf).unwrap();
                //main();
                //stdout().flush().unwrap();
                restrt();
            }
        }
    }
}

fn restrt() {
    clear().unwrap();
    main();
}
