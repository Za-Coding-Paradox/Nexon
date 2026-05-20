trait Shape {
    fn area(&self) -> f64;
    fn describe(&self) {
        println!("Area: {}", self.area());
    }
}
struct Circle {
    radius: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

const constCircle: Circle = Circle { radius: 3 as f64 };
fn main () {
    constCircle.describe();
}
