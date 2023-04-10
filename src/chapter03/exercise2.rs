use rand::Rng;

fn is_even(number: &i32) -> bool {
    number % 2 == 0
}

pub fn run() {
    let mut rng = rand::thread_rng();
    for _ in 1..10 {
        let number: i32 = rng.gen_range(-1000..1000);
        match is_even(&number) {
            true => println!("{} is even!", number),
            false => println!("{} is odd!", number),
        };
    }
}
