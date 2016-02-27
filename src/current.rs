pub mod structs;
pub mod linalg;
use linalg::Vec2;

fn main() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let c = a + b;
    println!("Hello Vec2 = {}", c);
}
