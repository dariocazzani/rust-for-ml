### 2. Getting Started with Rust <a name="getting-started-with-rust"></a>

In this chapter, you'll learn how to install Rust, write your first Rust program, and manage your projects using Cargo, Rust's package manager.

#### 2.1 Installing Rust

Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install Rust for your operating system. This will also install `cargo`, Rust's package manager.

#### 2.2 Hello, World! in Rust

Create a new file called `main.rs` and write the following Rust code:

```rust
fn main() {
    println!("Hello, World!");
}
```

To compile and run the program, open a terminal, navigate to the directory containing `main.rs`, and run the following commands:

```bash
$ rustc main.rs
$ ./main
```


You should see the output `Hello, World!`.

#### 2.3 Package management with Cargo

Cargo is Rust's package manager and build tool. It helps you manage dependencies, compile your projects, and run tests.

To create a new Rust project using Cargo, open a terminal, navigate to the directory where you want to create the project, and run the following command:

```bash
$ cargo new project_name
```


Replace `project_name` with the desired name for your project. This will create a new directory with the same name, containing a `Cargo.toml` file (the project's manifest) and a `src` directory with a `main.rs` file.

To build and run a Cargo project, navigate to the project's root directory (where `Cargo.toml` is located) and run the following command:

```bash
$ cargo run
```


This will compile your project and run the generated binary.

#### 2.4 Exercises

1. **Exercise 1:** Install Rust and Cargo on your computer, following the instructions from the [official Rust website](https://www.rust-lang.org/tools/install).

2. **Exercise 2:** Create a Rust program that prints "Hello, Rust!" to the console. Compile and run the program using `rustc`. (_Hint: Modify the "Hello, World!" example provided in this chapter._)

3. **Exercise 3:** Create a new Cargo project called "hello_cargo". Modify the `main.rs` file to print "Hello, Cargo!" to the console. Build and run the project using `cargo run`. (_Hint: Modify the "Hello, World!" example provided in this chapter._)

4. **Exercise 4 (challenging):** Write a Rust function that takes a string as input and returns the string reversed. Create a Cargo project to test your function by reversing a user-provided string. (_Hint: Use the `std::io` module to read user input and the `chars()` method to iterate over characters in the string._)
