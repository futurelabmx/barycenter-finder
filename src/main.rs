#[derive(Debug, Clone, Copy)]
pub struct Body {
    x: f64,
    y: f64,
    z: f64,
    mass: f64
}

fn average(a: f64, b:f64) -> f64 {
    (a + b) / 2.0
}

fn average_with_mass(a: f64, b:f64, amass: f64, bmass: f64) -> f64 {
    average(a * amass, b * bmass) / (amass * bmass)
}

fn main() {

}
