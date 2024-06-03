use std::fs::file;

let greeting_file_result = File::open("Hello.txt");

let greeting_file = match greeting_file_result{
    ok(file) => file,
    Err(error) => panic!("problem opening the file")
}