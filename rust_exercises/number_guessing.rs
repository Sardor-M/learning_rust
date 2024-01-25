
 use std::io;    // using stdio for user input
 use rand::Rng;  // using rand for random number generation

 fn man() {

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");

    loop {
        // Prompting the user to enter a number
        println!("Please input your guess: ");
        
        let mut guess = String::new();

        // Reading usr input
        io::stdin().read_line(&mut guess).expect("failed to read");

        // converting th input to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not Correct Number. Please enter a number.");
            }
        };

        // check for user's guess
        match guess.cmp(&secret_num) {
            std::cmp::Ordering::Less => println!("Too small."),
            std::cmp::Ordering::Greater => println!("Too big."),
            std::cmp::Ordering::Equal => {
                println!("You guesses the correct number, You win! {}", secret_num);
                break;
            }
        }
     }
 }