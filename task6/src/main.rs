
mod person;
use crate::person::Describe;
use person::Person;
mod gene_trats;
use gene_trats::Summable;
use crate::gene_trats::print_sum;
mod multiply;
use multiply::Multiply;
 use crate::multiply::print_multiply;
mod news;
use news::News;
use crate::news::Summary;
mod area;
use area::Area;
use crate::area::Circle;
use crate::area::Rectangle;

fn main(){
    let p = Person{
         name: String::from("James"),
        age: 21,
    };

    println!("{}",p.describe_person());
    println!("{:?}",p.describe());

    let m = vec![2,3,5];
    print_sum(m);

    let m = vec![2,3,5];
    print_multiply(m);

    let news = News{
        name: String::from("Daily Nation"),
        headlines: String::from("The Education"),
        number: 234,
    };

    println!("{}",news.summarize());
    println!("The default summarize is {}", news.default_summarize());

    let circle = Circle{radius: 7.0};
    let rectangle = Rectangle{length: 4.0, width: 3.0};


    println!("The area of a circle is {}",circle.area());
    println!("The area of a rectangle is {}", rectangle.area());
}



