use std::io;
use std::fs::read_to_string;
fn main() {
    println!("Welcome to the word count program"); 

    println!("enter word you want to count");

    let mut word_to_count = String::new();
    let binding = read_to_string("location of your txt file/").unwrap().to_lowercase();
    let content = binding.split_whitespace();   

    io::stdin().read_line(&mut word_to_count).expect("Failed to read line");

    let word_to_count = word_to_count.trim().to_string().to_lowercase();

    let mut count = 0;

    for word in content {
        if word == word_to_count{
            count += 1;}
        }
    

    println!("word input: {}", word_to_count);
    println!("word shows up: {}", count);

}
