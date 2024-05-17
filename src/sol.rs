
fn main(){
    let values = [1.2, 3.4, 8.6];
    let output = array_input(&values);
    println!("{}", output);
 
 }
 pub fn array_input(par1:&[f32]) -> f32{
    let mut index = 0;
    let length = par1.len();
    let mut totals = 0.0;
    while index < length{
       index = index + 1;
       totals = totals + par1[index];
    }
 return totals
 }