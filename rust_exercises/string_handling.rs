use std::io;

fn main() {
    
    println!("Your name? ");

    // Taking user input into string
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read");

    println!("How old are you?");

    // Reading user input to string
    let mut age::String = new();
    io::stdin().read_line(&mut age).expect("failed to read");

    // convertion to a number(u32)
    let age: u32 = match.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("not correct age, setted the age to 0");
            0
        }
    };

    println!("Hey, {}! You are {} years old.", name.trim(), age);
}