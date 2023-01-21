use std::io;
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let mut name = String::new();
    let mut age = String::new();

    println!("Give your name");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Your age");

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    let me = Person {
        name: name.trim().to_string(),
        age: age.trim().parse::<i32>().expect("Incorrect number given"),
    };

    println!("Hi! my name is {} and I am {} years old", me.name, me.age)
}
