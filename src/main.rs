use clearscreen::clear;
use colored::*;
//use sysinfo::*;
use std::{
    env::consts::OS,
    fs::{self, File},
    io::{stdin, stdout, Write},
    sync::Arc,
};

fn main() {
    clear().unwrap();
    println!(
        "{}",
        "Welcome to Nekolker - Advanced File Encryption!".yellow()
    );
    println!("{}", "Please choose one of the following options:".yellow());
    print!("(1) Encrypt\n(2) Decrypt\n(3) Read Important Information\n: ");
    stdout().flush().unwrap();
    let mut choice = String::new();
    stdin().read_line(&mut choice).unwrap();

    println!("{}", "Enter your password. If you are decrypting, enter the decryption password key. If encrypting, enter the encryption password key. If unsure, just press Enter:".green());
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let password: String = password.trim().to_string();

    loop {
        match choice.trim() {
            "3" => {
                clear().unwrap();
                println!(
                    "{}",
                    "THIS IS EXTREMELY IMPORTANT INFORMATION!"
                        .bold()
                        .bright_green()
                );
                println!(
                    "{}",
                    "If you encounter issues with the decrypted file, it could be due to:"
                        .bright_green()
                );
                println!(
                    "{}",
                    "1. Manual editing resulting in missing data blocks.".bright_green()
                );
                println!("{}", "2. Using the wrong decryption key.".bright_green());
                println!(
                    "{}",
                    "3.  Do Not Forget Your Key After Encryption!!!".bright_green()
                );
                println!("*****************************************************************");
                println!(
                    "{}{}",
                    "If you are encrypting a large file, it will take some time.".bright_green(),
                    "Please be patient!!".bright_red()
                );
                println!("*PRESS ENTER TO CONTINUE*");
                println!("*****************************************************************");
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                restrt();
            }
            "1" => {
                println!("{}", "Nekolker - Encrypt File");
                print!(
                    "{}",
                    "Enter the name of the file you want to encrypt (including its extension): "
                        .bold()
                        .bright_yellow()
                );
                stdout().flush().unwrap();
                let mut file_name = String::new();
                stdin().read_line(&mut file_name).unwrap();
                let file_name = file_name.trim().to_string();
                let passwd = password.clone() + "#neko#";
                let password_arc = Arc::new(passwd.clone());

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
                print!(
                    "{}",
                    "Output File Name (Without Extension i.e -> no period (.) : "
                        .bold()
                        .bright_green()
                );
                stdout().flush().unwrap();
                let mut onf = String::new();
                stdin().read_line(&mut onf).unwrap();
                let mut onfe = String::new();
                print!(
                    "{}",
                    "Output File Extension (Without Period i.e -> no dot (.) : "
                        .bold()
                        .bright_green()
                );
                stdout().flush().unwrap();
                stdin().read_line(&mut onfe).unwrap();
                let mut outp = String::new();

                print!(
                    "{}",
                    "Output Path Folder (Only Folder And No '/' if on linux or \n a backslash if on windows at the end) : "
                        .bold()
                        .bright_green()
                );
                stdout().flush().unwrap();
                stdin().read_line(&mut outp).unwrap();
                let os = OS;
                let mut outfile = String::new();
                match os {
                    "linux" => {
                        outfile =
                            outp.trim().to_owned() + "/" + onf.trim() + "." + onfe.trim() + ".neko"
                    }
                    "windows" => outfile = outp.trim().to_owned() + onf.trim() + "." + onfe.trim(),
                    _ => eprintln!("ERR - UNDEFINED OS - OS : {}", os),
                }
                println!("{}", outfile);
                let mut file = File::create(outfile.trim()).unwrap();
                file.write_all(&encrypted_data).unwrap();
                print!("{}{:?}{}", "File Encrypted At - ", file, " ");
                print!("{}{:?}{}", "With Password - ", password, "\n");
                println!("{}", "*\n\npress enter*".red());
                let mut buf = String::new();
                stdin().read_line(&mut buf).unwrap();
                restrt();
            }
            "2" => {
                print!(
                    "{}",
                    "Enter the file name you want to decrypt (with extension)(file must have .neko at end): "
                    .bold().bright_green()
                );
                stdout().flush().unwrap();
                let mut infile = String::new();
                stdin().read_line(&mut infile).unwrap();
                println!("\n");



                
                let mut outpath = String::new();
                print!("{}","Enter the output folder path (not file and no '/' if on linux and a backward slash if on windows at end): ".bold().bright_green());
                stdout().flush().unwrap();
                stdin().read_line(&mut outpath).unwrap();
                println!("\n");


                let mut outfile = String::new();
                print!("{}","The Output File Name : ".bold().bright_green());
                stdout().flush().unwrap();
                println!("\n");

                
                stdin().read_line(&mut outfile).unwrap();
                let passwd = password.clone() + "#neko#";

                let password_arc: Arc<String> = Arc::new(passwd.clone());

                let input_data = fs::read(&infile.trim()).unwrap();
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
                print!(
                    "{}",
                    "Please input the file extension to save the decrypted file in (e.g., .mp4)(remember to give period i.e -> dot(.) before the extension): "
                );
                stdout().flush().unwrap();
                stdin().read_line(&mut file_extension).unwrap();
                let output_file =
                    outpath.trim().to_string() + "/" + &outfile.trim() + &file_extension.trim();
                let mut file = File::create(output_file).unwrap();
                file.write_all(&decrypted_data).unwrap();
                print!("{}{:?}{}", "File Decrypted At - ", file, " ");
                print!("{}{:?}{}", "With Password - ", password, "\n");
                println!("{}", "*\n\npress enter*".red());
                let mut buf = String::new();
                stdin().read_line(&mut buf).unwrap();
                restrt();
            }
            _ => {
                println!("{}", "Please choose correctly!".red());
                println!("{}", "*press enter*".red());
                let mut buf = String::new();
                stdin().read_line(&mut buf).unwrap();
                restrt();
            }
        }
    }
}

fn restrt() {
    clear().unwrap();
    main();
}
