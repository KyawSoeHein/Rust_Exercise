use std::io;

pub fn ask_user_input() -> String {
    let mut input = String::new();
    println!("Please enter your input");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    return input;
}
