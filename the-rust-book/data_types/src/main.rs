fn main() {
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

    let _a: i8 = -4;
    let _b = 250; // i32
    let _c: u8 = 255;
    let _d: i64 = -9223372036854775808;

    // - Floating-Point Types
    //
    //   There's only two primitives here: `f32` and `f64`
    //   Default is `f64`, as modern CPU architecture can process them at the same speed
    //   as `f32` while retaining greater precision.
    //   Rust implements floating-point numbers according to the IEEE-754 standard: where `f32`
    //   is a single-precision float, and `f64` has double precision.

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // -- Numeric Operations
    //    addition
    let _sum = 5 + 10; // 15

    //    subtraction
    let _difference = 95.5 - 4.3; // 91.2

    //    multiplication
    let _product = 4 * 30; // 120

    //    division
    let _quotient = 56.7 / 32.2; // 1.7608...

    //    remainder
    let _remainder = 43 % 5; // 3

    // - Boolean Type
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // - Character Type
    let _c = 'z';
    let _z = 'Z';
    let _heart_eyed_cat = '😻';

    // There are two (2) compound types (i.e, grouping of multiple values):

    // - Tuple Type
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_e, _f, g) = _tup;
    println!("The value of `_g` is: {}", g);
    let five_hundred = _tup.0;
    let _six_point_four = _tup.1;
    let _one = _tup.2;
    println!("The value of `_five_hundred` from `_tup`: {}", five_hundred);

    // - Array Type

    //   -- Elements of an array must be of the same type
    //   -- Arrays in Rust are of a fixed length: once declared, they cannot grow or shrink in size
    //   -- Arrays are useful when you want data allocated on the stack rather than the heap, or
    //      when you want to ensure you have a fixed number of elements
    //   -- An array is not as flexible as a vector type - while similar, vectors are allowed to
    //      grow or shrink in size
    //   -- If in doubt, use a vector!
    let _arr = [1, 2, 3, 4, 5];

    //   NOTE: Using an array here because the elements are unlikely to change in the course of program
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Length of _months: {}", months.len());

    //   Accessing array elements is done via indexing
    let june = months[5];
    println!("Six month should be June: {}", june);

    //   Invalid array element access
    let new_arr = [1, 2, 3, 4, 5];
    let index = 10;
    let element = new_arr[index];
    println!("Element at index 10: {}", element);
}
