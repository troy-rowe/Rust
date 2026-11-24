fn main() {
    //This is the program title
    title();

    println!(); //This is a blank line

    println!("This is information about me:");
    //This contains my personal information
    personal_info();

    println!();

    println!("This is my current learning progress:");
    //This contains my current learning progress
    learning_progress();
}

fn title() {
    println!("====================");
    println!("RUST LEARNING JOURNAL");
    println!("====================");
}

fn personal_info() {
    //This collects all the relevant information about myself
    let name = "Troy";
    let age = 24;
    let major = "Computer Science";
    let favorite_language = "Python";

    //This is the output portion of the requirements
    println!("Name:");
    println!("{}", name);

    println!();

    println!("Age:");
    println!("{}", age);

    println!();

    println!("Major:");
    println!("{}", major);

    println!();

    println!("Favorite Programming Language:");
    println!("{}", favorite_language);
}

fn learning_progress() {
    let current_lesson = 2;
    let current_editor = "RustRover";
    let operating_system = "Windows";

    println!("My current lesson I have completed:");
    println!("{}", current_lesson);

    println!();

    println!("This is the current IDE I am using:");
    println!("{}", current_editor);

    println!();

    println!("This is the operating system I am using:");
    println!("{}", operating_system);
}