pub fn check(par1: Vec<i32>, par2:usize) -> Option<i32>{
    //checking if par1 < par2
    if par1.len() > par2 as usize{
      println!("length of par1 > par2");
      Some{par1[par2]}
  
    }else{
      return None;
    }
  
  }