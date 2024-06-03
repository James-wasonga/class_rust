pub enum Culture<Y>{
        ONE(Y),
        TWO
    }
    
    fn main(){
        let one = Culture::ONE(56);
        let two = Culture::ONE("Hllo world".to_string());
    
        match one{
            Culture::ONE(value) => println!("The value is {}",value);
            Culture::TWO(value) => println!("culture is two {}",value);
    
             
        }
        match two{
            Culture::ONE(value) => println!("The value is {}",value);
            Culture::TWO(value) => println!("-> Culture is two");
    
             
        }
    
    
    }