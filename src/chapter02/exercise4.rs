use std::io;
use std::io::stdout;
use std::io::Write;

fn reverse_string(input_string: &String) -> String {
    input_string.chars().rev().collect::<String>()
}

pub fn run() {
    print!("Write a message to reverse: ");
    stdout().flush().expect("Could not flush output");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read line");

    println!("The reversed string is: {}", reverse_string(&mut buffer));
}
