  pub struct Point<D>{

    x:D,
    y:D, 
    // z: i32,

}

impl<D> Point <D>{
   pub  fn new(x: D, y: D) -> Self{
        Point{x,y}
    }
    pub fn get_x(&self) ->&D{
        &self.x
    }

    pub fn get_y(&self) -> &D{
        &self.y
    }
}

pub fn swap<T>(x: &mut T, y: &mut T){
    std::mem::swap(x,y);
}