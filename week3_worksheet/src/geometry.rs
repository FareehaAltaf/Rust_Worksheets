// TASK 9 
/*
  Design a geometry library in Rust. 
  Create traits like Area and Perimeter that have methods for calculating area and perimeter for various 
  geometric shapes (e.g., Circle, Rectangle). Implement these traits for the respective struct types.
*/
pub const PI: f32 = 3.142;
// structs of shapes
pub struct Circle {
    pub radius: f32,
}
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}
// traits of area and perimeter
pub trait Area {
    fn area(&self) -> f32;
}

pub trait Perimeter {
    fn perimeter(&self) -> f32;
}

// implementing traits for Circle struct
impl Area for Circle {
    fn area(&self) -> f32 {
        PI * self.radius * self.radius // pie * r^2
    }
}

impl Perimeter for Circle {
    fn perimeter(&self) -> f32 {
        2.0 * PI * self.radius // 2 * pie * r
    }
}
// implementing traits for Rectangle struct
impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Perimeter for Rectangle {
    fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }
}



