//in your program you have been given a string "The lazy fox jumps over the bridge"
//count the number occurrences of each character over the string 

use crate::HashMap;


pub fn char_check(){
let my_string: &str = "The lazy Fox Jumps Over the Bridge lazy "; 

//creating a hashmap to store the character count

let mut char_count: HashMap<char, i32> = HashMap::new();


for c in my_string.chars(){
//check if the character is a letter
if c.is_alphabetic(){
    let c: char = c.to_ascii_lowercase();

    //Increment the count of the character in the hash map
    let counter: &mut i32 = char_count.entry(c).or_insert(0);
    *counter +=1;

    }
}

//print the count of each character
for (key, value) in &char_count {
    println!("{}:{}",key,value);
}

let my_word: &str = "The lazy Fox Jumps Over the Bridge lazy "; 

let mut total: HashMap<&str, i32> = HashMap::new();

for i in my_word.split_whitespace(){
    let count = total.entry(i).or_insert(0);
    *count +=1;

}
println!("{:?}",total);
}


