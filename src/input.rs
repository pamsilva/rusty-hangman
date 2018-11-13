use std::io;

pub fn take_user_guess() -> String {
    println!("Take your guess:");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to get input!");

        match guess.trim().parse() {
            Ok(val) => {
                return val;
            },
            Err(_) => {
                println!("Not a number, please insert a number:");
                continue;
            }
        };
    }
}
