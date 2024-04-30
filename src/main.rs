use std::{
    fs::{self, File},
    io::{stdin, stdout, Write},
    process::exit,
    sync::Arc,
    thread,
};
use colored::*;
use clearscreen::clear;



/*
Dear Neko,

I just wanted to take a moment to express my gratitude for having you in my life. Since the day we met on Discord and dove into the amazing world of Nekolker, it's been an incredible journey filled with laughter, support, and endless joy.

One of the things that truly warms my heart is how you wholeheartedly embrace everything I do. Whether it's sharing my latest programming project or discussing our favorite topics late into the night, your unwavering support and enthusiasm never fail to lift my spirits.

Your genuine interest in my passions and your willingness to share yours with me have created a bond that I cherish deeply. It's rare to find someone who not only appreciates your interests but also celebrates them alongside you, and for that, I am truly grateful.

Thank you for being the amazing friend that you are. Your kindness, positivity, and infectious energy brighten my days in ways I never imagined. Here's to many more adventures together, both in the virtual world of Discord and beyond.

With all my appreciation and affection,

Skub
*/






fn main() {
    println!("{}","Welcome To Neckolker\nFile Encryption At Its Peak!".yellow());
    println!("{}","Please Choose Any One Of The Following :".yellow());
    print!("(1) Encrypt\n(2) Decrypt\n(3) READ THIS! VERY IMPORTANT!!\n: ");
    stdout().flush().unwrap();
    let mut choice = String::new();
    stdin().read_line(&mut choice).unwrap();

    println!("{}","Enter your password (If you are decrypting, enter the decryption password key. If encrypting, enter the encryption password key. If unsure, just press Enter):".green());
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();

    loop {
        match choice.trim() {
            "3" => {
                clear().unwrap();
                println!("{}","THIS IS EXTREMELY IMPORTANT INFORMATION!".bold().bright_green());
                println!("{}","IF YOU ENCOUNTER ISSUES WITH THE DECRYPTED FILE, IT COULD BE DUE TO:".bright_green());
                println!("{}","1. Manual editing resulting in missing data blocks.".bright_green());
                println!("{}","2. Using the wrong decryption key.".bright_green());
                println!("***************************************************************");
                println!("{}{}","IF YOU ARE ENCRYPTING A LARGE FILE, IT WILL TAKE SOME TIME.".bright_green(),"PLEASE BE PATIENT!!".bright_red());
                println!("*PRESS ENTER TO CONTINUE*");
                println!("***************************************************************");
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                main();
            }
            "1" => {
                println!("Nekolker - Encrypt");
                print!("Enter the file path: ");
                stdout().flush().unwrap();
                let mut file_path = String::new();
                stdin().read_line(&mut file_path).unwrap();
                let file_path = file_path.trim().to_string();
                let password_arc = Arc::new(password.clone());

                let handle = thread::spawn(move || {
                    let input_data = fs::read(&file_path).unwrap();
                    let encrypted_data: Vec<u8> = input_data
                        .iter()
                        .map(|&byte| {
                            let seed = password_arc
                                .as_bytes()
                                .iter()
                                .fold(0, |acc: u8, &byte| acc.wrapping_add(byte));
                            byte ^ seed
                        })
                        .collect();
                    let output_file = file_path.to_string() + ".neko";
                    let mut file = File::create(output_file).unwrap();
                    file.write_all(&encrypted_data).unwrap();
                    println!("{}{}{:?}",
                        "File encrypted with password key: and saved at: ".bright_green(),
                        password_arc, file
                    );
                });

                handle.join().unwrap();
                main();
            }
            "2" => {
                print!("{}","Enter the file path: ".bright_green());
                stdout().flush().unwrap();
                let mut file_path = String::new();
                stdin().read_line(&mut file_path).unwrap();
                let file_path = file_path.trim().to_string();
                let password_arc = Arc::new(password.clone());

                let handle = thread::spawn(move || {
                    let input_data = fs::read(&file_path).unwrap();
                    let decrypted_data: Vec<u8> = input_data
                        .iter()
                        .map(|&byte| {
                            let seed = password_arc
                                .as_bytes()
                                .iter()
                                .fold(0, |acc: u8, &byte| acc.wrapping_add(byte));
                            byte ^ seed
                        })
                        .collect();
                    let mut fileextension = String::new();
                    print!("{}","Please input the file extension to save the decrypted file in\n it should be just before .neko in the file name\nlike tst.mp4.neko so .mp4 is the extension!\n:");
                    stdout().flush().unwrap();
                    stdin().read_line(&mut fileextension).unwrap();
                    let output_file = file_path.to_string() + fileextension.as_str();
                    let mut file = File::create(output_file).unwrap();
                    file.write_all(&decrypted_data).unwrap();
                    println!("{}{}{:?}",
                        "File decrypted with password key:  and saved at: ".bright_green(),
                        password_arc, file
                    );
                });

                handle.join().unwrap();
                main();
            }
            _ => {
                println!("{}","Please choose correctly!".red());
                exit(1);
            }
        }
    }
}
