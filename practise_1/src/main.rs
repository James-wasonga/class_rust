// use std::collections::HashMap;

// fn main(){
//     let vec_1: Vec<i32> = (1..=50).collect();
//     let mut vec_2: Vec<i32> = (51..=100).collect();

//     vec_2.reverse();

//     let mut my_hash = HashMap::new();

//     for(key, value) in vec_1.iter().zip(vec_2.iter()){
//         my_hash.insert(*key, *value);
//     }
//     for(key, value) in my_hash{
//         println!("{},{}",key, value);
//     }
// }

//shop management system that stores items and their qntities eg pens quanity 10
//using the Knowledge of struct and traits
//create solution and ensure to use the trait to display a summary of the stock 

struct Product{
    item: String,
    quantity: i32,
}
trait Summary{
    fn summary_data(&self);
}

impl Summary for Product{
    fn summary_data(&self){
         println!("stcok {}, data{}",self.item,self.quantity)
    }

}

fn main(){
   let mut vec:Vec<Product> = Vec!::new();
   let my_product = Product{item: "Pen",quanity: 4};

   vec.push(&my_product);

   my_product.summary_data();

}