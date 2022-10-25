mod password_store;

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

                match cmd {
                    "add-user" => {
                        if !user.is_none() && !password.is_none(){
                            password_store::add(user.unwrap().to_owned(), password.unwrap().to_owned())
                        } else if user.is_none(){
                            eprintln!("Error: No user specified for command 'add-user'")
                        } else {
                            eprintln!("Error: No password specified for command 'add-user'")
                        }
                    }

                    "check-password" => {
                        if !user.is_none() && !password.is_none(){
                            password_store::check(user.unwrap().to_owned(), password.unwrap().to_owned())
                        } else if user.is_none(){
                            eprintln!("Error: No user specified for command 'check-password'")
                        } else {
                            eprintln!("Error: No password specified for command 'check-password'")
                        }
                    }
                    "remove-user" => {
                        if !user.is_none(){
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
