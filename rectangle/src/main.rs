//program that calculate the area of a rectangle(height and width );
//using the knowledge of struct

mod area;
use area::rectangle;

fn main(){
    // let rect1 = rectangle{
    //     height: 20,
    //     width: 30,
    // };

    let rect1 = rectangle::new(20,30);
    let result = rect1.area();
    println!("The area is {}",result);
}


