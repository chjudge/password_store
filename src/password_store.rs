use base64;
use rand::{thread_rng, Rng};
use sha2::{Sha512, Digest};
use std::{fs::File, fmt::format};

pub(crate) const PATH: &str = "Project2PW.txt";

// fn read_file() -> Vec<String> {
//     let s = std::fs::read_to_string(PATH)?.split('\n')

// }

// fn write_line(line: String) {
//     std::fs::write(PATH, line);
// }

//adds a user to the password store
pub(crate) fn add(user: String, password: String) {
    println!("add");

    let rand: String = thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    

    let salt = base64::encode(rand);


    let mut hasher = Sha512::new();
    hasher.update(password + &salt);

    let hash = base64::encode(hasher.finalize());

    println!("{user}:$6${salt}${hash}");
}

pub(crate) fn check(user: String, password: String) {
    println!("check")
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
