//shop management system that stores items and their qntities eg pens quanity 10
//using the Knowledge of struct and traits
//create solution and ensure to use the trait to display a summary of the stock
struct Product{
    item: String,
    quanity: i32,
}
trait Summary{
    fn summary_data(&self);
}

impl Summary for Product{
    fn summary_data(&self){
         println!("stcok {}, data{}",self.name,self.quantity)
    }

}