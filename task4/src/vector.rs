pub fn check (par1: Vec<i32>, par2: usize ) -> Option<i32> {

    if par1.len() >  par2 as usize {
        println!("Par1 > par2");
       return Some(par1[par2]);
    }
    return None;

} 

pub fn array_check(arr: &[i32] , index: usize) -> Option<i32>{
    if  index < arr.len() {
        println!("arr > index");
        return Some(arr[index]);

    }
    return None;

}

pub fn compare(num1: Vec<i32>, num2:  usize) -> Option<i32> {

    if num1.len() > num2 as usize {
        println!("num1 > num2");
        return Some(num1[num2]);
    }

    return None;
    
}

pub fn array(array1: &[i32], index: usize) -> Option<i32>{
    if array1.len() > index as usize {
        return Some(array1[index]);

    }
    return None;
}