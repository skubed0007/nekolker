use clearscreen::clear;
use colored::*;
use std::{
    fs::{self, File},
    io::{stdin, stdout, BufReader, Write},
    sync::Arc,
};

use zip::{write::FileOptions, CompressionMethod, ZipArchive, ZipWriter};

fn main() -> ! {
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

    println!("{}", "Enter your password. If you're decrypting, enter the decryption password key. If encrypting, enter the encryption password key. If unsure, just press Enter:".green());
    let mut psswd = String::new();
    stdin().read_line(&mut psswd).unwrap();
    let pswd: String = psswd.trim().to_string();

    loop {
        match chc.trim() {
            "3" => {
                clear().unwrap();
                println!("{}", "IMPORTANT INFORMATION:".bold().bright_green());
                println!(
                    "{}",
                    "If you encounter issues with the decrypted file, it could be due to:"
                        .bright_green()
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
                println!("*****************************************************************");
                println!(
                    "{}",
                    "If you're encrypting a large file, it may take some time. Please be patient."
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
                let pswd = pswd.clone() + "#neko#" + "   ";
                let pswd_arc = Arc::new(pswd.clone());

                let input_data = fs::read(&flnm).unwrap();
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

                print!(
                    "{}",
                    "Output Folder Name (only folder name, without path): "
                        .bold()
                        .bright_green()
                );
                stdout().flush().unwrap();
                let mut folder_name = String::new();
                stdin().read_line(&mut folder_name).unwrap();

                let output_folder = folder_name.trim().to_string();
                fs::create_dir_all(&output_folder).unwrap();

                let otfs = format!("{}/{}.neko", output_folder, flnm);

                let filee = File::create(&otfs).unwrap();

                let mut zpfs: ZipWriter<File> = ZipWriter::new(filee);
                let zpoptns: FileOptions<()> = FileOptions::default()
                    .compression_method(CompressionMethod::DEFLATE)
                    .compression_level(Some(9));
                ZipWriter::start_file(&mut zpfs, flnm + ".neko", zpoptns).unwrap();
                zpfs.write_all(&encrypted_data).unwrap();
                zpfs.finish().unwrap();
                println!("File Encrypted At - {}", otfs);
                println!("Using password - {}", psswd.trim());
                println!("{}", "*\n\npress enter*".red());
                let mut buf = String::new();
                stdin().read_line(&mut buf).unwrap();
                restrt();
            }
            "2" => {
                print!("{}", "Enter the name of the file you want to decrypt (including its extension) (file must have .neko at end): ".bold().bright_green());
                stdout().flush().unwrap();
                let mut infile = String::new();
                stdin().read_line(&mut infile).unwrap();
                let infile = infile.trim();

                let dzfs = File::open(infile).unwrap();
                let mut zreadr = ZipArchive::new(BufReader::new(dzfs)).unwrap();

                let mut output_folder = String::new();
                print!("{}", "Enter the output folder name: ".bold().bright_green());
                stdout().flush().unwrap();
                stdin().read_line(&mut output_folder).unwrap();
                let output_folder = output_folder.trim();
                for idx in 0..zreadr.len() {
                    let mut file = zreadr.by_index(idx).unwrap(); // Borrow the file from the ZipArchive
                    println!("{:?}", file.name());
                    let output_file = format!("{}/{}{}", output_folder, file.name(), ".ntmp");
                    let outff = output_file.as_str();
                    let mut outputt_file = File::create(&output_file).unwrap();

                    std::io::copy(&mut file, &mut outputt_file).unwrap();
                    let pswd = pswd.clone() + "#neko#" + "   ";
                    let pswd_arc = Arc::new(pswd.clone());

                    let input_data = fs::read(&outff).unwrap();
                    let decrypted_data: Vec<u8> = input_data
                        .iter()
                        .map(|&byte| {
                            let seed = pswd_arc
                                .as_bytes()
                                .iter()
                                .fold(0, |acc: u8, &byte| acc.wrapping_add(byte));
                            byte ^ seed + 4 ^ 4
                        })
                        .collect();

                    print!(
                        "{}",
                        "PLEASE Input THE OUTPUT FILE NAME WITHOUT EXTENSION : ".bold()
                    );
                    stdout().flush().unwrap();
                    let mut outfn = String::new();
                    stdin().read_line(&mut outfn).unwrap();
                    let mut outfe = String::new();
                    print!(
                        "{}",
                        "PLEASE Input THE OUTPUT FILE Extension (With dot at (.) ): ".bold()
                    );
                    stdout().flush().unwrap();
                    stdin().read_line(&mut outfe).unwrap();
                    let ff = format!("{}/{}{}", output_folder.trim(), outfn.trim(), outfe.trim());
                    let mut outfff = File::create(ff).unwrap();
                    outfff.write(&decrypted_data).unwrap();
                    fs::remove_file(&output_file).unwrap();
                    println!("{}{:?}", "File Decrypted At : ", outfff);
                }

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