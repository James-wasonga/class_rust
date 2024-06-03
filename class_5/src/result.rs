use std::fs::File;


// fn main(){

//     let greeting_file_result: Result<File> = File::open("hello.tx");


// let greeting_file: File = match greeting_file_result{

//     ok(file:File) => file,
//     Err(error: Error) => match error.kind(){
// ErrorKind::NOtFound => match File::create("hello.text"){
//     ok(file:File) => file,
//     Err(error: Error) => panic!("Problem creating the file {:?}",e),

// };
// other_errror: ErrorKind => {
//     panic!("Problem opening the file {:?}",other_errror),

// }   
// };
// }
// }
// use std::io;
// fn main(){

//     let result: Result<String,i32> = get_content_of_file();
//     println!("result are {}", result.expect("An error occured"));
// }

// fn get_content_of_file() -> Result<String, i32>{
//     // ok("hey".to_string(),0)
//     Err(30)
// }

fn main(){
    let result: Result<String, Error> = get_content_of_file();
    println!("result are {}", result.expect("An error occured"));

}

fn get_content_of_file() -> Result<String, io::Error>{
    
    let username_file_result:Result<File> = File::open("hello.text");


    let mut username_file: File = match username_file_result{
        ok(file: file) => file,
        Err(e :Error) => return Err(e),
    };

    let mut username: String =String::new();

    match username_file.read_to_string(&mut username){
        ok(_) => ok(username),
        Err(e :Error) => Err(e),
    }
    
}

//perform a get request using any libraries 
//have a look
