use std::io;

fn main() {
    let mut word = String::new();
    println!("Enter the word you would like to reverse: ");

    io::stdin()
        .read_line(&mut word)
        .expect("Could not readline.");

    let word: &str = word.trim();

    let mut string = String::new();

    for i in word.chars().rev() {
        string.push(i);
    }
    println!("The word reversed is: {}", string);
}
