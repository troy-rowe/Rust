use std::io;

fn main() {
    title();

    println!();

    user_input();

    println!();

    learn_info();

    println!();

    data_type();

    println!();

    mut_var();
}

fn title() {
    println!("***************************");
    println!("STUDENT INFORMATION SYSTEM");
    println!("***************************");
}

fn user_input() {
    println!("*********************");
    println!("PERSONAL INFORMATION");
    println!("*********************");

    let mut name: String = String::new();
    let mut age: String = String::new();
    let mut major: String = String::new();
    let mut favorite_language: String = String::new();

    println!("Enter your name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Enter a valid input");

    println!("Enter your age:");
    io::stdin()
        .read_line(&mut age)
        .expect("Enter a valid input");

    println!("Enter your major:");
    io::stdin()
        .read_line(&mut major)
        .expect("Enter a valid input");

    println!("Enter your favorite programming language:");
    io::stdin()
        .read_line(&mut favorite_language)
        .expect("Enter a valid input");

    println!("Your name is: {}", name);
    println!("Your age is: {}", age);
    println!("Your major is: {}", major);
    println!("Your favorite programming language is: {}", favorite_language);
}

fn learn_info() {
    println!("*********************");
    println!("LEARNING INFORMATION");
    println!("*********************");

    let mut current_lesson: String = String::new();
    let mut hours_studies: String = String::new();
    let mut enjoy_rust: String = String::new();

    println!("Enter your current Rust lesson:");
    io::stdin()
        .read_line(&mut current_lesson)
        .expect("Enter a valid input");

    println!("Enter your hours studied:");
    io::stdin()
        .read_line(&mut hours_studies)
        .expect("Enter a valid input");

    println!("Are you enjoying Rust?");
    io::stdin()
        .read_line(&mut enjoy_rust)
        .expect("Enter a valid input");

    println!("Your current Rust lesson is: {}", current_lesson);
    println!("Your hours studied is: {}", hours_studies);
    println!("Are you enjoying Rust: {}", enjoy_rust);
}

fn data_type() {
    println!("*******************");
    println!("DATA TYPE PRACTICE");
    println!("*******************");

    let integer: i32 = 20;
    let float: f64 = 4.9;
    let boolean: bool = false;
    let character: char = 'N';
    let string: &str = "Hello";

    println!("{}", integer);
    println!("{}", float);
    println!("{}", boolean);
    println!("{}", character);
    println!("{}", string);
}

fn mut_var() {
    println!("***************************");
    println!("MUTABLE VARIABLES PRACTICE");
    println!("***************************");

    let mut var1: i32 = 9;
    let mut var2: f64 = 7.9;

    println!("Here are the variables before the change:");
    println!("{}", var1);
    println!("{}", var2);

    var1 = 3;
    var2 = 8.9;

    println!("Here are the variables after the change:");
    println!("{}", var1);
    println!("{}", var2);
}