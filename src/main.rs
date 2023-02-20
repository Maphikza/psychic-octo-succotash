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

    let string_two = reverse_string(&string);
    println!("The reverse_string function returned {}", string_two);

    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(reverse_string("rust"), "tsur");
    assert_eq!(reverse_string("abcdefg"), "gfedcba");
}

fn reverse_string(input: &str) -> String {
    let mut string = String::new();
    for i in input.chars().rev() {
        string.push(i);
    }
    string
}
