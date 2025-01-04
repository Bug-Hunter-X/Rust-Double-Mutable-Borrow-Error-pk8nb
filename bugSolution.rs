fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y += 1;
    } // y goes out of scope here
    {
        let z = &mut x; // z is a mutable reference to x
        *z += 1; // This is now safe
    }
    println!("x = {}", x);
} 