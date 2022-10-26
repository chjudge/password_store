use regex::Regex;
mod password_store;

/*
    Runs the logic loop allowing users to
        add-user [user] [password] (adds the given user and password (becomes hashed) to the file)
        check-password [user] [password] (compares given user and password with the hashed password in the file)
        remove-user [user] (removes the given user from the file along with their hashed password)
        print (prints the entire file of users and hashed passwords to console)
        end (exits the program)
*/
fn main() {
    if !std::path::Path::new(password_store::PATH).exists() {
        match std::fs::File::create(password_store::PATH) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error creating file");
                return;
            }
        }
    }
    let mut line: String;

    // Main program loop waiting for user input
    loop {
        println!("please enter a command");
        line = String::new();
        let result = std::io::stdin().read_line(&mut line);

        match result {
            Err(error) => {
                println!("Error reading line {}", error);
                continue;
            }
            Ok(_) => {
                let mut args = line.trim().split(" ");

                let cmd = args.next().unwrap();
                let user = args.next();
                let password = args.next();
                // Regex that validates a username
                let user_re = Regex::new(r"^[a-zA-Z_\-0-9]+$").unwrap();

                match cmd {
                    "add-user" => {
                        if !user.is_none() && !password.is_none(){
                            if user_re.is_match(user.unwrap()) {
                                password_store::add(user.unwrap().to_owned(), password.unwrap().to_owned())
                            }
                            else {
                                eprintln!("Error: Invalid username for command 'add-user'")
                            }
                        }
                        else if user.is_none(){ 
                            eprintln!("Error: No user specified for command 'add-user'")
                        } else {
                            eprintln!("Error: No password specified for command 'add-user'")
                        }
                    }

                    "check-password" => {
                        if !user.is_none() && !password.is_none(){
                            // validates the given password hashed matches the one in the file
                            password_store::check(user.unwrap().to_owned(), password.unwrap().to_owned())
                        } else if user.is_none(){
                            eprintln!("Error: No user specified for command 'check-password'")
                        } else {
                            eprintln!("Error: No password specified for command 'check-password'")
                        }
                    }

                    "remove-user" => {
                        if !user.is_none(){
                            // removes the given user from file
                            password_store::remove(user.unwrap().to_owned())
                        } else{
                            eprintln!("Error: no user specified for command 'remove-user'")
                        }
                    },
                    "print" => password_store::print(),
                    "end" => break,
                    _ => eprintln!("Invalid command"),
                }
            }
        }
    }
}
