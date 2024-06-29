fn main() {
 let result = fibo(8);
println!("{}",result);

}
fn fibo(n: u32) -> u32{

    if n == 0{
        println!("1") 
    }else if n==1{
        println!("1") 
    }else{
        let mut a = 0;
        let mut b = 1;

        for _ in 2 ..n{
            let c = a + b;
            a = b;
            b = c;

        }
        return b;
    }
}

fn fib(n: i32) -> i32 {

    if n <= 0 {

        return 0;

    } else if n == 1 {

        return 1;

    }

    return fib(n - 1) + fib(n - 2);

}

fn main() {

    let mut sum: i32 = 0;

    for i in 2..34 {

        let term: i32 = fib(i);

        println!("Fibacci of {} is {}", i, fib(i));

        if term < 4000000 && term % 2 == 0 {

            sum += term;

        }

    }

    println!("sum is {}", sum);

}