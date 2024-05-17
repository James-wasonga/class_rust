
pub struct book {
    name: String,
    status: bool,
    category: String,
}
impl book{

pub fn new(name:String, status:bool, category:String) ->Self{
    return book{
        name,status,category
    }
}
pub fn check_availability(&self) -> bool{
 return self.status
}
}