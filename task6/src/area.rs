pub trait Area{
    fn area(&self) -> f64;
}

//struct for a circle
pub struct Circle{
    pub radius: f64,
}

//implimenting the trait for the Circle

impl Area for Circle {
    fn area(&self) -> f64{
        3.142 * self.radius * self.radius
    }

}

// creating struct Rectangle
pub struct Rectangle {
   pub length: f64,
   pub width: f64,
}

//implementing Area trait for Rectangle

impl Area for Rectangle{
    fn area(&self) -> f64 {
        self.length * self.width
    }
}