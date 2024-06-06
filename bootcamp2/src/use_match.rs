pub fn number(x: i32){
match x {
    1 | 2 => println!("The number is 1 or 2"),
    3..=10 => println!("The number is between 3 and 10"),
    x if x % 2 == 0 => println!("The number is even"),
    _=> println!("The number is uknown"),

}


}


//you have been tasked with creating a traffic light 
//it can be red, green , orange
//using knowledge of match red do not go,orange ready and  green go 


pub fn traffic(color: &str){
    match color{
        "Red" => println!("Red light Do not Go"),
        "Orange" => println!("Orange light Be ready to Go"),
        "Green" => println!("Green GO"),
        _=> println!("No such Traffic Light"),

    }
 
}

