use std::io;

fn main() {
    title();

    user_info();
}

fn title() {
    println!("#######################");
    println!("RUST LEARNER SURVEY");
    println!("#######################");
}

fn user_info() {
    let mut name: String = String::new();
    let mut current_rust_lesson: String = String::new();
    let mut favorite_language: String = String::new();

    println!("Enter your name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Enter a valid input");

    println!();

    println!("Enter your current Rust lesson:");
    io::stdin()
        .read_line(&mut current_rust_lesson)
        .expect("Enter a valid input");

    println!();

    println!("Enter your favorite programming language:");
    io::stdin()
        .read_line(&mut favorite_language)
        .expect("Enter a valid input");

    println!();

    println!("Your name is: {}", name);
    println!("Your current Rust lesson is: {}", current_rust_lesson);
    println!("Your favorite programming language is: {}", favorite_language);
}