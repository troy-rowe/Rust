use std::io;

fn main() {
    title();

    println!();

    survey();
}

fn survey() {
    println!("#####################");
    println!("SURVEY RESULTS");
    println!("#####################");

    println!();

    user_info();

    additional_questions();

    data_type_demo();
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

fn additional_questions() {
    let mut game: String = String::new();
    let mut code_editor: String = String::new();
    let mut sport: String = String::new();

    println!("Enter your favorite computer game:");
    io::stdin()
        .read_line(&mut game)
        .expect("Enter a valid input");

    println!("Enter your preferred code editor:");
    io::stdin()
        .read_line(&mut code_editor)
        .expect("Enter a valid input");

    println!("Enter your favorite sport:");
    io::stdin()
        .read_line(&mut sport)
        .expect("Enter a valid input");

    println!();

    println!("Your favorite computer game is: {}", game);
    println!("Your preferred code editor is: {}", code_editor);
    println!("Your favorite sport is: {}", sport);
}

fn data_type_demo() {
    let current_year_college: i32 = 1;
    let hours_studied: f64 = 0.5;
    let enjoying_rust: bool = true;
    let favorite_letter: char = 'B';

    println!("Current college year: {}", current_year_college);
    println!("Hours studied today: {}", hours_studied);
    println!("Enjoying Rust: {}", enjoying_rust);
    println!("Favorite letter: {}", favorite_letter)
}