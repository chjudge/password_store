pub(crate) const PATH: &str = "Project2PW.txt";

// function that opens the file, reads it, and returns a vector of strings
// each string is a line in the file
fn read_file() -> Vec<String> {
    let mut file = match std::fs::File::open(PATH) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error opening file");
            return Vec::new();
        }
    };

    let mut contents = String::new();
    match std::io::Read::read_to_string(&mut file, &mut contents) {
        Ok(_) => {},
        Err(_) => {
            eprintln!("Error reading file");
            return Vec::new();
        }
    }

    contents.lines().map(|s| s.to_string()).collect()
}

//adds a user to the password store
pub(crate) fn add(user: String, password: String){
    println!("add")
}

pub(crate) fn check(user: String, password: String){
    println!("check")
}

pub(crate) fn remove(user: String){
    println!("remove")
}

pub(crate) fn print(){
    let lines = read_file();

    for line in lines{
        // println!("{}", line)
        animate_text(&line, 50)
    }

}



// function that animates the text
pub(crate) fn animate_text(text: &str, speed: u64){
    for c in text.chars(){
        print!("{}", c);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(speed));
    }
}

