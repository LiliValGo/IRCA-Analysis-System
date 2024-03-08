use std::io;

fn main() {
    println!("IRCA CLASIFICATION");

    loop {
        println!("Please input the value feature");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input: f32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a feature value!");
                continue;
            },
        };

        println!("You guessed: {user_input}");

        match user_input {
            _ if user_input <= 5.0 => println!("NO RISK"),
            _ if user_input <= 14.0 => println!("LOW RISK"),
            _ if user_input <= 35.0 => println!("MEDIUM RISK"),
            _ if user_input <= 80.0 => println!("HIGH RISK"),
            _ => {
                println!("SANITARY UNFEASIBLE");
                break; 
            },
        }
    }
}



