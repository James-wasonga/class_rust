//fibonacci
fn main(){
fn check(n: u32) -> u32{

    if n == 0{
        return 1
    }else if n==1{
        return 1
    }else{
        let mut a = 0;
        let mut b = 1;

        for _ in 2 ..n{
            let c = a + b;
            a = b;
            b = c;

        }
        return b
    }
}


    let result = check(10);
    println!("{}",result);

}