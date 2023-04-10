pub fn run() {
    let mut counter: i16 = 0;
    for i in 1..11 {
        counter += i;
    }
    println!("The sum from 1 to 10 is {}", counter);
}
