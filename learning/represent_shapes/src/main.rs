struct Rectangle {
    width: f64,
    height: f64
}
impl Rectangle{
    fn new(width: f64, height: f64) -> Rectangle{
        Rectangle {
            width: width,
            height: height
        }
    }
    fn scale(&mut self, scale: f64) {
        self.width *= scale;
        self.height *= scale;
    }
    fn get_area(&self) -> f64{
        self.width * self.height
    }
}
struct Circle {
    radius: f64
}
impl Circle {
    fn new(radius: f64) -> Circle{
        Circle {
            radius: radius,
        }
    }
    fn scale(&mut self, scale: f64) {
        self.radius *= scale;
    }
    fn get_area(&self) -> f64{
        self.radius * self.radius * 3.14
    }
}

fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    
    rect.scale(0.5);
    
    assert_eq!(rect.get_area(), 1.02);


    let mut circ = Circle::new(1.2);
    assert_eq!(circ.get_area(), 4.5216);
    
    circ.scale(2.0);
    
    assert_eq!(circ.get_area(), 18.0864);

    println!("Tests passed!");
}

