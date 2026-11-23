fn main() {
    title();

    println!();

    personal_info();
}

fn title() {
    println!("====================");
    println!("RUST LEARNING JOURNAL");
    println!("====================");
}

fn personal_info() {
    let name = "Troy";
    let age = 24;
    let major = "Computer Science";
    let favorite_language = "Python";

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