pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn should_panic(number: i32){
    if number < 100{
        panic!("Too large")
    }else {
        if number < 50 {
            println!("You are almost there")
        }else{
            println!("just reduce it a little")
        }
    }
}

pub fn find_word(word: &str, text: &str) -> Option<usize> {

    text.find(word)
    
    }

  fn par(par: String) -> bool{
    let v = vec!["one", "two", "three"];
    let contained = v.into_iter().find(|x| *x == par);
     match contained{
        Some(par)=> true,
        None => false
     }
  }  

#[cfg(test)]
mod tests {
    use super::*;
   

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    
    #[test]
    fn it_works_2() {
        let result = add(3, 3);
        assert_eq!(result, 6);
    }

    pub trait Area{
        fn area(&self) -> f64;
    }
     pub struct Rectangle{
        length: f64,
        width: f64,
     }

     impl Area for Rectangle{
        fn area(&self) -> f64{
            self.length * self.width
        }
     }
//create an implementaion of function signature fn divide(a: f64, b: f64) -> Result<f64, String>
//write a implementaion , can't divide by zero  and make some checks 

fn divide(a: f64, b: f64 ) -> Result<f64, String>{
    if b == 0.0 {
        Err(String::from("A number Zero cannot be used for division"))
        // panic!("cannot divide by zero");
    }else{
        Ok(a/b)
    }
}


//coming up with the generic 

pub trait Check{
    
    fn num_zero() -> Self;
    fn num_is_zero(&self) -> bool;

}

impl Check for f64{
    fn num_zero() -> Self{
        0.0
    }
    fn num_is_zero(&self) -> bool{
        *self == 0.0
    }
}

pub fn divisions<T:  std::ops::Div<Output = T>+ Check>(a: T, b: T ) -> Result<T, String>{
    if b.num_is_zero(){
        Err(String::from("The number is Zero"))
        
    }else{
        Ok(a/b)
    }
}


fn find_less <T: std::cmp::PartialOrd> (par_1: T, par_2: T) -> T{
    if par_1 < par_2 {
        par_1
    }else {
        par_2
    }
    
}

// Assignment

//using iterator adaptors
//Write a Rust function that takes a vector of booleans and returns
//"All true!" if all elements are true, "All false!" if all elements are false, and "Mixed!" otherwise. 



fn my_vector(val: Vec<bool>) -> String{
    if my_vector.iter().all(|x|x){
        "They are All true!".to_string();
    }else  if my_vector.iter().all(|x| !x){
        "They are All false".to_string();
    }else{
        "They are mixed".to_string();
    }
}




        #[test]
    fn should_panic_test() {
        should_panic(500);
    }

    #[test]
    fn test_find_word(){
        let word_to_be_found = find_word("the","the quick brown fox jumped over the bridge");
        assert_eq!(word_to_be_found, Some(0));

        let invalid_word = find_word("rust","the quick brown fox jumped over the bridge");
        assert!(invalid_word != Some(0));
    }
    #[test]
    fn test_rectangle_area(){
        let rec = Rectangle{length: 4.0, width:3.0 };
        assert_eq!(rec.area(), 12.0);
    }

     //testing for the correct

    #[test]
     fn test_divide(){
        let result = divide(10.0, 2.0);
        assert_eq!(result,Ok(5.0) );
    }

    //testing for the wrong , the err
    #[test]
    fn test_wrong(){
        let result_2 = divide(10.0, 0.0);
        assert_ne!(result_2,Err(String::from("cannot divide by Zero")) );
    }

    //the gneric part implementation
    #[test]
    fn test_division(){
       let result = divisions(10.0, 2.0);
       assert_eq!(result,Ok(5.0) );
   }

   //testing for the wrong , the err
   #[test]
   fn test_wrong_division(){
       let result_2 = divisions(10.0, 0.0);
       assert_ne!(result_2,Err(String::from("The number is zero")) );
   }

   #[test]
   fn  test_numbers(){
      let num_1 = find_less(5,6);
      assert!(num_1 == 5);
   }
   #[test]
    fn test_char(){
    let char_1 = find_less('a', 'b');
    assert!(char_1 == 'a');
    assert_ne!(char_1, 'b');
}
    #[test]
    fn test_string(){
    let str_1 = find_less("Kevin", "Charles");
    assert!(str_1 == "Kevin");
    
 }
   #[test]
   fn test_my_vector(){
   let first_val = vec![true,true];
   assert!(first_val ==  "They are All true!".to_string());
   let second_val = vec![false,false];
   assert!(second_val ==  "They are All false".to_string());
   let third_val = vec![true, false];
   assert!(third_val =="They are mixed".to_string());


   }
}


