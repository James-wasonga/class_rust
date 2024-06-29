//palindrome is a number that can be read both ways from right to left and also from left to right
//909 = 909 -> 909

//find a max palindrome that can be formed form a three digits figures mult together
//9009 -> L-R(9009) ->R-L(9009) 

fn my_palindrome(n: u32) ->bool{
    let my_string = n.to_string();
    let my_rev: String = my_string.chars().rev().collect(); 
    my_string == my_rev

}

fn largest_palindrome() -> u32{
    let mut max_palindrome = 0;

    for i in 100..1000{
        for k in 100..1000{
            let product = i*k;

            if my_palindrome(product) {
                if product > max_palindrome{
                    max_palindrome  = product;
                }
            }
        }
    }

    return max_palindrome;
}

fn main(){
    let result = largest_palindrome();
    println!("{}",result );
}