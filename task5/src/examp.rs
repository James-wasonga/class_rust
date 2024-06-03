pub struct Point<D,X,R>{
    x:D,
    y:X,
    z:R,
}
impl<D,X,R> Point <D>{
    fn new(x:D, y:X, z:R)-> Self{
        Point{x,y,z}
    }
    fn get_x(&self) -> &D{
        &self.x
    }
}
