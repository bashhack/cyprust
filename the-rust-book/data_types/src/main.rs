fn main() {
    //
    // There are four (4) scalar types (i.e., representing a single value):

    // - Integer Types
    //
    //   |-- Length -|---------- Signed ---------|---------------- Unsigned  ---------------|
    //   | (in bits) |  (positive and negative)  |   (larger positive value, no negative)   |
    //   |-----------|---------------------------|------------------------------------------|
    //   |---8-bit---|------------i8-------------|-------------------u8---------------------|
    //   |--16-bit---|------------i16------------|-------------------u16--------------------|
    //   |--32-bit---|------------i32------------|-------------------u32--------------------|
    //   |--64-bit---|------------i64------------|-------------------u64--------------------|
    //   |---arch----|-----------isize-----------|------------------usize-------------------|
    //
    //   |- Number literals -|-- Example --|
    //   |-----Decimal-------|---98_222----|
    //   |-------Hex---------|----0xff-----|
    //   |------Octal--------|----0o77-----|
    //   |-----Binary--------|-0b1111_0000-|
    //   |- Byte (u8 only)---|----b'A'-----|
    //
    //
    //   Q: Help - when do I use which type?
    //   A: Generally, use the Rust default of i32, as it's generally the fastest!
    //   Q: When do I use these isize and usize integer types?
    //   A: Use them when indexing collections!

    let a: i8 = -4;
    let b = 250; // i32
    let c: u8 = 255;
    let d: i64 = -9223372036854775808;

    // - Floating-Point Types
    //
    //   There's only two primitives here: `f32` and `f64`
    //   Default is `f64`, as modern CPU architecture can process them at the same speed
    //   as `f32` while retaining greater precision.
    //   Rust implements floating-point numbers according to the IEEE-754 standard: where `f32`
    //   is a single-precision float, and `f64` has double precision.

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // -- Numeric Operations
    //    addition
    let sum = 5 + 10;

    //    subtraction
    let sum = 5 + 10;

    //    multiplication
    let sum = 5 + 10;

    //    division
    let sum = 5 + 10;

    //    remainder
    let remainder = 43 % 5;
    println!('remainder: {}', remainder)




    // - Booleans
    // - Characters

}
