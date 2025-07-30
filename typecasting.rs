fn main() {
    let x: i32 = 10;
    let y: f64 = x as f64; // casting i32 to f64
    let z: u8 = x as u8;   // casting i32 to u8

    println!("x as f64: {}", y);
    println!("x as u8: {}", z);

    let f: f64 = 65.4321;
    let i: u8 = f as u8;
    println!("f64 {} cast to u8 is {}", f, i);

    // Casting from character to integer
    let c: char = 'A';
    let c_code: u32 = c as u32;
    println!("Character '{}' as u32 is {}", c, c_code);
}
