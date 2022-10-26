use base64;
use rand::{thread_rng, Rng};
use regex::Regex;
use sha2::{Digest, Sha512};
use std::{io::Write, fs::OpenOptions};

//Constant variable that stores the path to the password file
pub(crate) const PATH: &str = "Project2PW.txt";

/*
    Helper method that reads in the whole file, splits it into lines,
     and collects them into a vector of Strings
 */
fn read_file() -> Vec<String> {
    return std::fs::read_to_string(PATH)
        .unwrap_or("".to_string())
        .split('\n')
        .map(|x| x.to_string())
        .collect();
}

/*
    Helper method that opens the file
    and appends the vairable line to the file
*/
fn write_line(line: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(PATH)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

/*
    Clears the entire file by 
    overwriting it with an empty string
*/
fn clear_file() {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(PATH)
        .unwrap();

    if let Err(e) = write!(file, "") {
        eprintln!("Couldn't write to file: {}", e);
    }
}

/*
    Hashes the given password with addition of 
    the given salt using SHA512 and returns the base64 encoding
*/
fn hash_pass(password: String, salt: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password + &salt);

    return base64::encode(hasher.finalize());
}

/*
    Adds the given user and hashed password to the file
*/
pub(crate) fn add(user: String, password: String) {
    println!("add");

    let rand: String = thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let salt = base64::encode(rand);

    let hash = hash_pass(password, salt.to_owned());

    let out = format!("{user}:$6${salt}${hash}");
    write_line(out);
}

/*
    Checks to make sure the given password matches
    the given user's actual password
*/
pub(crate) fn check(user: String, password: String) {
    // Regular expression used to extract the username, salt, and password
    let re = Regex::new(r"(?P<user>[a-zA-Z_\-0-9]+):\$6\$(?P<salt>[A-Za-z\d+/=]+)\$(?P<pass>[A-Za-z\d+/=]+)").unwrap();
    let correct = read_file()
    .iter()
    .any(|text| {
        if text == "" {return false};
        let caps = re.captures(text).unwrap();
        // checks the password hashes are the same
        return user == caps["user"].to_string() && hash_pass(password.to_owned(), caps["salt"].to_string()) == caps["pass"].to_string();
    });
    if correct {
        println!("Password is good!");
        return;
    }
    println!("Username pass combination is no good.")
}

/*
    Removes the user matching the given string from
    the file of users and hashed passwords
*/
pub(crate) fn remove(user: String) {
    let lines = read_file();
    clear_file();
    // Regular expression used to extract the username, salt, and hashed password
    let re = Regex::new(r"(?P<user>[a-zA-Z_\-0-9]+):\$6\$(?P<salt>[A-Za-z\d+/=]+)\$(?P<pass>[A-Za-z\d+/=]+)").unwrap();
    for line in lines {
        if &line == "" {continue};
        let caps = re.captures(&line).unwrap();
            if user != caps["user"].to_string() {
                let u  = caps["user"].to_string();
                let s = caps["salt"].to_string();
                let h = caps["pass"].to_string();
                let out = format!("{u}:$6${s}${h}");
                write_line(out);
            }
    };
}

/*
    Prints the file of users and hashed passwords
    out to the console screen (animates the text)
*/
pub(crate) fn print() {
    let lines = read_file();

    println!();
    for line in lines{
        println!();
        animate_text(&line, 5);
    }
    println!();
}

// function that animates the text
pub(crate) fn animate_text(text: &str, speed: u64) {
    for c in text.chars() {
        print!("{}", c);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(speed));
    }
}
