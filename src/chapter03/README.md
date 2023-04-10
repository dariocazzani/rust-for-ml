### 3. Rust Language Basics <a name="rust-language-basics"></a>

In this chapter, you'll learn the fundamental concepts and constructs of the Rust programming language, such as variables, data types, control structures, functions, enums, and modules.

#### 3.1 Variables and data types

In Rust, variables are immutable by default, which means their values cannot be changed after they are assigned. To make a variable mutable, use the `mut` keyword:

```rust
let x = 5;        // Immutable variable
let mut y = 10;   // Mutable variable
y = 15;           // Allowed because y is mutable
```

Rust has several basic data types, including integers, floating-point numbers, booleans, and characters:

```rust
let a: i32 = 42; // 32-bit signed integer
let b: f64 = 3.14159; // 64-bit floating-point number
let c: bool = true; // Boolean
let d: char = 'R'; // Character
```

#### 3.2 Control structures

Rust has several control structures, such as `if`, `else`, `while`, `loop`, and `for`:

```rust
// if-else
let x = 5;
if x < 10 {
    println!("x is less than 10");
} else {
    println!("x is greater than or equal to 10");
}

// while loop
let mut x = 0;
while x < 5 {
    println!("x is {}", x);
    x += 1;
}

// infinite loop
loop {
    println!("This will print indefinitely");
}

// for loop
for x in 0..5 {
    println!("x is {}", x);
}
```

#### 3.3 Functions

Functions in Rust are defined using the fn keyword:
```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let x = 5;
    let y = 10;
    let result = add(x, y);
    println!("The sum is {}", result);
}
```

#### 3.4 Enums and pattern matching
Enums are a way to define custom data types with a finite set of possible values:

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let direction = Direction::North;

    match direction {
        Direction::North => println!("We are heading north!"),
        Direction::South => println!("We are heading south!"),
        Direction::East => println!("We are heading east!"),
        Direction::West => println!("We are heading west!"),
    }
}
```

#### 3.5 Modules and packages
Modules help organize your code by grouping related functionality. Packages are a collection of one or more crates (libraries or executables).

```rust
// src/lib.rs
pub mod math {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn subtract(x: i32, y: i32) -> i32 {
        x - y
    }
}

// src/main.rs
use my_package::math;

fn main() {
    let x = 5;
    let y = 10;
    let result = math::add(x, y);
    println!("The sum is {}", result);
}
```

#### 3.6 Exercises

1. **Exercise 1:** Create a mutable variable called counter initialized to 0. Use a for loop to iterate over the numbers 1..11 and increment `counter` by each number. Print the final value of `counter`. (_Hint: Use the `+=` operator to update `counter`._)

2. **Exercise 2:** Write a function called `is_even` that takes an integer as input and returns a boolean indicating whether the number is even. Test your function with several input values. (_Hint: Use the `%` operator to check for divisibility._)

3. **Exercise 3:** Define an enum called `Color` with variants `Red`, `Green`, and `Blue`. Write a function called `describe_color` that takes a `Color` value as input and returns a string describing the color. Use pattern matching to handle each variant. Test your function with each variant of the `Color` enum. (_Hint: Use the `match` keyword for pattern matching._)

4. **Exercise 4 (challenging):** Create a module called `geometry` that contains two submodules: `circle` and `rectangle`. The `circle` submodule should define a function called `area` that takes a radius as input and returns the area of a circle with that radius. The `rectangle` submodule should define a function called `area` that takes a width and a height as input and returns the area of a rectangle with those dimensions. In the `main` function, use the `geometry` module to calculate and print the area of a circle with radius 5 and a rectangle with width 4 and height 6. (_Hint: The area of a circle is π * r^2, and the area of a rectangle is width * height._)

