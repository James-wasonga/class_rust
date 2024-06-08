fn main(){
    let numbers: i32 = vec![1,3,5,6,4,8];
    let even_numbers = numbers.into_iter().filter(|&x| x % 2 == 0).collect();

    println!("The even number is {:?}",even_numbers);

    

}