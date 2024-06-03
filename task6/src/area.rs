pub trait Area{
    fn area(&self) -> f64;
}
pub struct Circle{
    pub radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64{
        3.142 * self.radius * self.radius
    }

}

pub struct Rectangle {
   pub length: f64,
   pub width: f64,
}

impl Area for Rectangle{
    fn area(&self) -> f64 {
        self.length * self.width
    }
}