use std::io;
use std::io::Write;
use std::io::stdout;

fn reverse_string(input_string: & String) -> String {
    let mut new_string:String = String::new();
    new_string.extend(input_string.chars().rev());
    return new_string;
} 

pub fn run() {
    print!("Write a message to reverse: ");
    stdout().flush().expect("Could not flush output");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Could not read line");
    println!("The reversed string is: {}", reverse_string(&buffer));
}