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
    let mut chc = String::new();
    stdin().read_line(&mut chc).unwrap();

    println!("{}", "Enter your pswd. If you are decrypting, enter the decryption pswd key. If encrypting, enter the encryption pswd key. If unsure, just press Enter:".green());
    let mut psswd = String::new();
    stdin().read_line(&mut psswd).unwrap();
    let pswd: String = psswd.trim().to_string();
    //print!("{}{:?}{}", "With pswd - ", psswd.trim(), "\n");


    loop {
        match chc.trim() {
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
                let mut flnm = String::new();
                stdin().read_line(&mut flnm).unwrap();
                let flnm = flnm.trim().to_string();
                let pswd = pswd.clone() + "#neko#"+"   ";
                let pswd_arc = Arc::new(pswd.clone());

                let input_data = fs::read(&flnm).unwrap();
                let encrypted_data: Vec<u8> = input_data
                    .iter()
                    .map(|&byte| {
                        let seed = pswd_arc
                            .as_bytes()
                            .iter()
                            .fold(0, |acc: u8, &byte| acc.wrapping_add(byte));
                        byte ^ seed + 4^57
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
                let mut otfs = String::new();
                match os {
                    "linux" => {
                        otfs =
                            outp.trim().to_owned() + "/" + onf.trim() + "." + onfe.trim() + ".neko"
                    }
                    "windows" => otfs = outp.trim().to_owned() + onf.trim() + "." + onfe.trim(),
                    _ => eprintln!("ERR - UNDEFINED OS - OS : {}", os),
                }
                println!("{}", otfs);
                let mut file = File::create(otfs.trim()).unwrap();
                file.write_all(&encrypted_data).unwrap();
                print!("{}{:?}{}", "File Encrypted At - ", file, " ");
                print!("{}{:?}{}", "With pswd - ", psswd.trim(), "\n");
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


                let mut otfs = String::new();
                print!("{}","The Output File Name : ".bold().bright_green());
                stdout().flush().unwrap();
                println!("\n");

                
                stdin().read_line(&mut otfs).unwrap();
                let pswd = pswd.clone() + "#neko#"+"   ";

                let pswd_arc: Arc<String> = Arc::new(pswd.clone());

                let input_data = fs::read(&infile.trim()).unwrap();
                let decrypted_data: Vec<u8> = input_data
                    .iter()
                    .map(|&byte| {
                        let seed = pswd_arc
                            .as_bytes()
                            .iter()
                            .fold(0, |acc: u8, &byte| acc.wrapping_add(byte));
                        byte ^ seed + 4^57
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
                    outpath.trim().to_string() + "/" + &otfs.trim() + &file_extension.trim();
                let mut file = File::create(output_file).unwrap();
                file.write_all(&decrypted_data).unwrap();
                print!("{}{:?}{}", "File Decrypted At - ", file, " ");
                print!("{}{:?}{}", "With pswd - ", psswd.trim(), "\n");
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
