fn main() {
    // to create a string object we use String::new() method or String::from() method
    //it is a growable, mutable, owned, UTF-8 encoded string type 
    let mut my_string=String::new();// empty string
    my_string.push_str("Hello,world again!"); // push_str() method to append a literal to a String
    println!("greeting: {}",my_string); 
    let my_string=String::from("Hello,world again!"); // String::from() method to create a string object
    println!("greeting: {}",my_string);
     
     
     
    let name = "Rust Programming".to_string(); // to_string() method to create a string object
    println!("this is first name: {}",name);
    let name2 = name.replace("Rust","Dart"); // replace() method to replace a string
    println!("this is second name: {}",name2);
    let mut reaction = "I love Rust Programming".to_string();
    reaction.push_str(" and Dart Programming"); // push_str() method to append a literal to a String
    println!("reaction: {}",reaction);
    println!("length of reaction: {}",reaction.len()); // len() method to get the length of a string
    println!("is reaction empty: {}",reaction.is_empty()); // is_empty() method to check if a string is empty
   
   
   
   
    let  reaction = "     I love Rust Programming         ".to_string();// trim() method to remove whitespace from a string
    let trimed = reaction.trim(); // trim() method to remove whitespace from a string
    println!("reaction length : {}, trimed length : {}",reaction,trimed.len());// difference in length
    let rust_says = " I am a statically-typed programming language designed for performance and safety ".to_string();
    for word in rust_says.split_whitespace() { // split_whitespace() method to split a string into an iterator over its words
        println!("splitted by split_whitespace(): {}",word);
    }
   
   
    // split() method to split a string into an iterator over its words
    let key_words = "Rust,Programming,Language".to_string();
    for word in key_words.split(",") { // split() method to split a string into an iterator over its words
        println!("splitted by split(): {}",word);
    }

    let word = "Rust".to_string();
    for c in word.chars() { // chars() method to iterate over the characters in a string
        println!("done by chars(): {}",c);
    }
}
