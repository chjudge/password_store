use base64;
use rand::{thread_rng, Rng};
use regex::Regex;
use sha2::{Digest, Sha512};
use std::{fmt::format, fs::File, io::Write, fs::OpenOptions};

pub(crate) const PATH: &str = "Project2PW.txt";

fn read_file() -> Vec<String> {
    return std::fs::read_to_string(PATH)
        .expect("File should exist prior to this being called")
        .split('\n')
        .map(|x| x.to_string())
        .collect();
}

fn write_line(line: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(PATH)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn hash_pass(password: String, salt: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password + &salt);

    return base64::encode(hasher.finalize());
}

//adds a user to the password store
pub(crate) fn add(user: String, password: String) {
    println!("add");

    let rand: String = thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let salt = base64::encode(rand);

    let hash = hash_pass(password, salt);

    let out = format!("{user}:$6${salt}${hash}");
    write_line(out);
}

pub(crate) fn check(user: String, password: String) {
    let re = Regex::new(r"(?P<user>[a-zA-Z_\-0-9]+):$6$(?P<salt>[A-Za-z\d+/=]+)$(?P<pass>[A-Za-z\d+/=]+)").unwrap();
    let correct = read_file()
    .iter()
    .any(|text| {
        let caps = re.captures(text).unwrap();
        return user == caps["user"].to_string() && hash_pass(password.to_owned(), caps["salt"].to_string()) == caps["pass"].to_string();
    });
    if correct {
        println!("Password is good!");
        return;
    }
    println!("Username pass combination is no good.")
}

pub(crate) fn remove(user: String) {
    println!("remove")
}

pub(crate) fn print() {
    // let lines = read_file();

    // for line in lines{
    //     // println!("{}", line)
    //     animate_text(&line, 50)
    // }
}

// function that animates the text
pub(crate) fn animate_text(text: &str, speed: u64) {
    for c in text.chars() {
        print!("{}", c);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(speed));
    }
}
