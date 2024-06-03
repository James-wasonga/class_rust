//write a rust funcction that takes a vector of tuple and return the result of the tuple
//the sum of the second element if the first is odd and zero otherwise


const my_num: Vec<(i32, i32)> = Vec::new();
pub fn sum(my_num: Vec<(i32, i32)>){

for (item1 , item2) in my_num.clone(){

 if item2 % 2 == 0{
     println!("{:?}",my_num[0]);
 }else if item1 % 3 == 0{
    println!("{:?}",my_num[1]);

 }
}

}

 