use sha2::{Digest, Sha256};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};
fn main() {
    let args: Vec<String> = env::args().collect();

    let wanted_hash = &args[1];
    let password_file = "src/passlist.txt";
    let mut attempts = 1;

    println!("Cracking {}\n", wanted_hash);

    let password_list = File::open(password_file).unwrap();
    let reader = BufReader::new(password_list);

    for line in reader.lines() {
        let line = line.unwrap();
        let password = line.trim().to_owned().into_bytes();
        let password_hash = format!("{:x}", Sha256::digest(&password));

        println!("[{}] {} == {}", attempts, std::str::from_utf8(&password).unwrap(), password_hash);

        if &password_hash == wanted_hash {
            println!("Found after {} Password = {} hash = {}\n", attempts,std::str::from_utf8(&password).unwrap(), password_hash);
            exit(0);
        }
        attempts += 1;
    }
    println!("No hash found");

}
