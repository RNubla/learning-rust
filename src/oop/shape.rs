use std::f64::consts::PI;

enum Shapes {
    Triangle(Triangle),
    Rectangle(Rectangle),
    Circle(Circle),
}

pub struct Triangle {
    base: f64,
    height: f64,
}

pub struct Rectangle {
    base: f64,
    height: f64,
}

pub struct Circle{
    radius: f64,
}

// Use trait to abstract common behavior across different types
pub trait Shape{
    fn area(&self) -> f64;
}
impl Triangle{
    pub fn new(base: f64, height:f64) -> Self {
        Triangle{base,height}
    }

}

impl Rectangle{
    pub fn new(base: f64, height: f64) -> Self{
        Rectangle{base, height}
    }
}

impl Circle{
    pub fn new(radius: f64) -> Self{
        Circle{radius}
    }
}
impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl Shape for Rectangle{
    fn area(&self) -> f64 {
        self.base * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius.powf(2.00)
    }
}


