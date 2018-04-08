fn main() {
    // Immutability
    let mut x = 5;
    println!("The value of `x` is: {}", x);
    x = 6;
    println!("Because `x` was mutable, our value is now: {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000;

    // Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of shadowed `y` is: {}", y);
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value os shadowed `spaces` is: {}", spaces);
}
