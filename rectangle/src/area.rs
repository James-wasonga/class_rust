pub struct rectangle{
   height: u32,
   width: u32,

}
impl rectangle{
    pub fn new(height:u32,width:u32) -> Self{
        return rectangle{
            height,width
        }

    }
pub fn area(&self) -> u32{
    self.width * self.height
}
}

//research on the document attributes and the previous assignment should contain documentation 