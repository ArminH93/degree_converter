use std::io;

fn main() {
    println!("Type in a temperature in °F : ");

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        let user_input: f32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only numbers allowed, please try again");
                continue;
            }
        };

        let deg_celsius = (user_input - 32_f32) * 5_f32 / 9_f32;
        println!("{}°F is {}°C", user_input, deg_celsius.round());
        break;
    }
}
