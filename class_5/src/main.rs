
// mod result;
// use::result::get_content_of_file;

fn main() {


    let result: Result<String, Error> = get_content_of_file();
    println!("result are {}", result.expect("An error occured"));

//     let my_vec: Vec<i32> = vec![1,2,3,4,5];

//     // let result_1 = my_vec[1];
//     // println!("{}",result_1);

//     let result_2:Option<&i32> = my_vec.get(99);

//     match result_2{
//         some(number: &i32) => {
//             println!("Number found {}", number);
        
//         None =>{
//             println!("An error....");
//         }
//     }


// }

// let my_string: String = "Hello World".to_string();
// let res: String = match my_string

// let res: Option<&str> = print_occupation(name:"Scientist");
// println!("{}",res.expect("Not in the option available"));
}


fn print_occupation(name: &str) -> Option<&str>{
    match name {
        "programmer" => Some("Hey Fellow Coder"),
        "Mathematician" => Some("Maths is great"),
        _=>None
    }
}

fn print_number(name: &str) -> Option<i32>{
    match name {
        "Kenn" => Some(200),
        "Phil" => Some(201),
        _=>None
    }
}