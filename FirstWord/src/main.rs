use std::io;

fn main() {
    println!("Please enter some words and this program will extract only the first word.");
    let mut words = String::new();
    io::stdin()
            .read_line(&mut words)
            .expect("Cant read inputted words :(");
    
    let firstword = firstword(&words);

    println!("The firstword in the bunch of words you typed above is {}", firstword);
    println!("{}", words)
}

fn firstword(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//This program is an example of String Slices and references. 