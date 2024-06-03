//create a struct called news
//the struct should have three properties , name, Headlines and the number 
//create a trait that has two functions ,,, one function has a default implemenation and the second fn does not have a default implementation
//create a summary of news 
//have a impl of the trait for the news struct  
//call the two trait functions from the main



//create struct for the News

pub struct News{
   pub  name: String,
   pub  headlines: String,
    pub number: u32,
}

pub trait Summary{
    fn summarize(&self) -> String; //function without  default implementaion

    //function with a default implementaion

    fn default_summarize(&self) -> String{
        format!("The news is from {}",self.name())
    }

    //to help get the name

    fn name(&self) -> &str;
       
}

impl Summary for News{
    fn summarize(&self) -> String{
        format!("Read from {},{}", self.name,self.headlines)
    }

    fn name(&self) -> &str{
        &self.name
    }
}



