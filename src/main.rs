use std::io;
// use pdf;
fn main() {
    println!("Welcome to the word count program"); 

    println!("enter word you want to count");

    let mut word_to_count = String::new();

    io::stdin().read_line(&mut word_to_count).expect("Failed to read line");

    //Remove this and implement a pdf file reader
    let words = vec!["apples", "banana", "apples", "king", "rabbit"]; //test

    let word_to_count = word_to_count.trim();

    let mut count = 0;

    for word in words.iter() {
        if *word == word_to_count{
            count += 1;}
        }
    

    println!("word input: {}", word_to_count);
    println!("word shows up: {}", count)
}
