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
        line = String::new();
        let result = std::io::stdin().read_line(&mut line);

        match result {
            Err(error) => {
                println!("Error reading line {}", error);
                continue;
            }
            Ok(_) => {
                let mut args = line.split(" ");

                let cmd = args.next().unwrap();
                let user = args.next().unwrap_or("*").to_owned();
                let password = args.next().unwrap_or("*").to_owned();

                match cmd {
                    "add-user" => password_store::add(user, password),
                    "check-password" => password_store::check(user, password),
                    "remove-user" => password_store::remove(user),
                    "print" => password_store::print(),
                    "end" => break,
                    _ => println!("Invalid command"),
                }
            }
        }
    }
}
