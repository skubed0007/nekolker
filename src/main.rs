use clearscreen::clear;
use colored::*;
use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
    sync::Arc,
};

use zip::{write::FileOptions, CompressionMethod, ZipWriter};

fn main() {
    clear().unwrap();
    println!(
        "{}",
        "Welcome to Nekolker - Advanced File Encryption!".yellow()
    );
    println!("{}", "Please choose one of the following options:".yellow());
    println!("(1) Encrypt\n(2) Decrypt\n(3) Read Important Information\n: ");
    let mut chc = String::new();
    stdin().read_line(&mut chc).unwrap();

    println!("{}", "Enter your password. If you're decrypting, enter the decryption password key. If encrypting, enter the encryption password key. If unsure, just press Enter:".green());
    let mut psswd = String::new();
    stdin().read_line(&mut psswd).unwrap();
    let pswd: String = psswd.trim().to_string();

    match chc.trim() {
        "1" => encrypt_file(&pswd),
        "2" => decrypt_file(&pswd),
        "3" => read_important_information(),
        _ => println!("{}", "Invalid choice. Please choose correctly!".red()),
    }
}

fn encrypt_file(password: &str) {
    println!("{}", "Nekolker - Encrypt File");
    print!(
        "{}",
        "Enter the name of the file you want to encrypt (including its extension): "
            .bold()
            .bright_yellow()
    );
    stdout().flush().unwrap();
    let mut file_path = String::new();
    stdin().read_line(&mut file_path).unwrap();
    let file_path = file_path.trim();

    if let Ok(mut file) = File::open(file_path) {
        let mut input_data = Vec::new();
        file.read_to_end(&mut input_data).unwrap();

        print!(
            "{}",
            "Enter the output folder path for the encrypted file: "
                .bold()
                .bright_green()
        );
        stdout().flush().unwrap();
        let mut output_folder = String::new();
        stdin().read_line(&mut output_folder).unwrap();
        let output_folder = output_folder.trim();

        print!(
            "{}",
            "Enter the output file name for the encrypted content (without extension): "
                .bold()
                .bright_green()
        );
        stdout().flush().unwrap();
        let mut file_name = String::new();
        stdin().read_line(&mut file_name).unwrap();
        let file_name = file_name.trim();

        let output_path = PathBuf::from(output_folder).join(format!("{}.neko", file_name));

        if let Ok(mut zip_file) = File::create(&output_path) {
            let pswd_arc = Arc::new(password.to_owned());
            let encrypted_data: Vec<u8> = input_data
                .iter()
                .map(|&byte| {
                    let seed = pswd_arc
                        .as_bytes()
                        .iter()
                        .fold(0, |acc: u8, &byte| acc.wrapping_add(byte));
                    byte ^ seed + 4 ^ 4
                })
                .collect();

            let mut zip = ZipWriter::new(&mut zip_file);
            let options: FileOptions<()> = FileOptions::default()
                .compression_method(CompressionMethod::DEFLATE)
                .compression_level(Some(9));
            zip.start_file(file_path.to_owned() + ".neko", options)
                .unwrap();
            zip.write_all(&encrypted_data).unwrap();
            println!("File encrypted successfully at: {}", output_path.display());
        } else {
            println!("{}", "Failed to create output file. Exiting...".red());
        }
    } else {
        println!("{}", "Error: No such file or directory".red());
        println!("Failed to read input file: {}", file_path);
        println!("{}", "Exiting...".red());
    }
}

fn decrypt_file(password: &str) {
    println!("{}", "Nekolker - Decrypt File");
    print!(
        "{}",
        "Enter the name of the encrypted file you want to decrypt (file must end with .neko): "
            .bold()
            .bright_yellow()
    );
    stdout().flush().unwrap();
    let mut file_path = String::new();
    stdin().read_line(&mut file_path).unwrap();
    let file_path = file_path.trim();

    if let Ok(mut zip_file) = File::open(file_path) {
        print!(
            "{}",
            "Enter the output folder path for the decrypted file: "
                .bold()
                .bright_green()
        );
        stdout().flush().unwrap();
        let mut output_folder = String::new();
        stdin().read_line(&mut output_folder).unwrap();
        let output_folder = output_folder.trim();

        print!(
            "{}",
            "Enter the output file name for the decrypted content (without extension): "
                .bold()
                .bright_green()
        );
        stdout().flush().unwrap();
        let mut file_name = String::new();
        stdin().read_line(&mut file_name).unwrap();
        let file_name = file_name.trim();

        let output_path = PathBuf::from(output_folder).join(format!("{}.decrypted", file_name));

        let mut zip = zip::ZipArchive::new(&mut zip_file).unwrap();
        let mut encrypted_data = Vec::new();
        zip.by_index(0)
            .unwrap()
            .read_to_end(&mut encrypted_data)
            .unwrap();

        let pswd_arc = Arc::new(password.to_owned());
        let decrypted_data: Vec<u8> = encrypted_data
            .iter()
            .map(|&byte| {
                let seed = pswd_arc
                    .as_bytes()
                    .iter()
                    .fold(0, |acc: u8, &byte| acc.wrapping_add(byte));
                byte ^ (seed + 4) ^ 4
            })
            .collect();

        let mut output_file = File::create(&output_path).unwrap();
        output_file.write_all(&decrypted_data).unwrap();

        println!("File decrypted successfully at: {}", output_path.display());
    } else {
        println!("{}", "Error: No such file or directory".red());
        println!("Failed to read input file: {}", file_path);
        println!("{}", "Exiting...".red());
    }
}

fn read_important_information() {
    println!("{}", "IMPORTANT INFORMATION:".bold().bright_green());
    println!(
        "{}",
        "If you encounter issues with the decrypted file, it could be due to:".bright_green()
    );
    println!(
        "{}",
        "1. Manual editing leading to missing data blocks.".bright_green()
    );
    println!(
        "{}",
        "2. Using the incorrect decryption key.".bright_green()
    );
    println!(
        "{}",
        "3. Remember to keep your encryption key safe!".bright_green()
    );
    println!(
        "{}",
        "If you're encrypting a large file, it may take some time. Please be patient."
            .bright_green()
    );
    println!("*PRESS ENTER TO CONTINUE*");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
}
