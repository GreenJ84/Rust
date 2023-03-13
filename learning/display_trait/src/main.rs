use std::fmt;

struct Satellite {
    name: String,
    velocity: f64
}


impl fmt::Display for Satellite {
    fn fmt(&self, s: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(s, "{} is flying at {} miles per hour.", self.name, self.velocity)
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("{} update: {}", hubble.name, hubble)
}
