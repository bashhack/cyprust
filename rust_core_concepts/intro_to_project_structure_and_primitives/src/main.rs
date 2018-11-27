fn main() {
    // NOTE: Throughout, I've used an '_' in front of variable names to
    //       signify to the compiler that the value will be unused - moreover,
    //       that we don't care if it is unused - silencing the compiler
    //       warning stemming from:
    //       #[warn(unused_variables)]

    println!("Bonjour, Marc! Happy coding! - from Rust");

    // ------------------------------------------------------------------------
    // Variables and Mutability
    // ------------------------------------------------------------------------

    // All variables are immutable, by default
    // The following operation is not permitted (we need the `mut` keyword):
    let immutable_var = 5;
    // immutable_var = 6;
    println!("Value of my immutable_var is: {}", immutable_var);

    // In contrast, `let mut` denotes a mutable variable declaration
    let mut mutable_var = 5;
    println!("Value of my mutable_var after declaration: {}", mutable_var);
    mutable_var = 6;
    println!("Value of my mutable_var after updating: {}", mutable_var);

    // We can 'shadow' variables by redeclaring a name with its value
    // NOTE: The following example from 'The Rust Programming Language'
    //       provides a good working example of why we might want to shadow
    let spaces = "     ";
    let spaces = spaces.len();
    println!(
        "Value of 'shadowed' `spaces` helps to avoid writing
something like `spaces_str` and `spaces_num`: {}",
        spaces
    );

    // NOTE: If we had used `mut` in the shadowed example, we would have
    //       encountered a compile-time error because the types were
    //       incongruent (&str and usize)

    // Alonside variables, we have constants - these also bind name to value

    // We are not allowed to use mut with constants (declared with
    // `const` vs `let`), they are immutable - forever and always

    // Constants must have the type of their value annotated

    // Constants must not be the value of an expression or function,
    // that is, they cannot be a value that could only be computed at runtime

    // By convention, `const` is paired with a name in all caps

    const _IMMUTABLE_CONSTANT: i8 = -4;

    // ------------------------------------------------------------------------
    // Data Types
    // ------------------------------------------------------------------------

    // Every value has a 'data type', either scalar or compound

    // Rust is staticly-typed, and has excellent type inference

    // (A) Scalar Types (Single Value)
    //     (1) Integer Types: i8, u8, i16, u16, i32, u32, i64, u64, isize, usize
    //         ---
    //         General Notes:
    //         - Number without a fractional component
    //         - Can be of signed or unsigned value
    //         - Numeric identifier (8/16/32/64) denotes bits of space
    //         - Integer literals are allowed in the following forms:
    let _decimal = 98_222;
    let _hex = 0xff;
    let _octal = 0o77;
    let _binary = 0b1111_0000;
    let _byte = b'A';
    //         - If integer overflow occurs, Rust will `panic`
    //         TLDR:
    //         - i32 is a safe bet for most general applications of integers
    //           (it's fast on 64-bit systems)
    //         - Indexing a collection? Use `isize` or `usize`
    let _integer: i32 = 42;

    //     (2) Floating Point Types: f32, f64
    //         ---
    //         General Notes:
    //         - Floating-point numbers are represented according to the
    //           IEEE-754 standard
    //         - Number with decimal points (fractional)
    //         - 32 or 64 bits in size
    //         TLDR:
    //         - Default type is `f64` on modern CPUs, more precision and
    //           close speed to `f32`
    let _floating_point = 42.0; // f64

    //     (3) Numeric Operations: +, -, *, /, %, etc.
    //         ---
    //         General Notes:
    //         - Some of the common numeric operations are shown below:
    //           - addition
    let _sum = 3 + 1 + 4;
    //           - subtraction
    let _difference = 14.5 - 7.6;
    //           - multiplication
    let _product = 9 * 14;
    //           - division
    let _quotient = 15 / 5;
    //           - remainder
    let _remainder = 28 % 3;
    //         TLDR:
    //         - A full list of operators can be found in the
    //           official documentation here:
    //           file://$HOME/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/html/book/2018-edition/appendix-02-operators.html

    //     (4) The Boolean Type: true, false
    //         ---
    //         General Notes:
    //         - Booleans are of type `bool`
    //         - They are one byte in size
    //         TLDR:
    //         - We typically consume Booleans through conditionals
    //           (i.e.,  `if` expressions)
    let _f: bool = false;
    let _is_true = true;

    //     (5) The Character Type: ''
    //         ---
    //         General Notes:
    //         - The Rust languages "most primitive alphabetic type"
    //         - Represents a Unicode Scalar Value
    //           (i.e., can be more than ASCII - Accented letters, Chinese,
    //           Japanese, Korean, and zero-width spaces, etc.)
    //         TLDR:
    //         - A `char` literal is declared with single-quotes, a string
    //           literal with double-quotes
    let _char = 'a';
    let _string_literal = "Rustacean";

    // (B) Compound Types (Group multiple values into one type)
    //     (1) The Tuple Type
    //         ---
    //         General Notes:
    //         - Groups n number of values (may be of a variety of types)
    //         - Are of a fixed length,
    //           cannot grow or shrink in size after declaration
    //         TLDR:
    //         - Elements enclosed in parentheses, comma-separated, may be of
    //           various types, entire form is of a static length
    let _tup_with_types: (char, i32, f64, u8) = ('x', 500, 6.4, 1);

    //         - Tuples may be unpacked
    let tup = (3.14, 1.4142, 2.7182);
    let (_pi, _sqrt_of_2, _eulers) = tup;

    //         - A `.` followed by an index is used to access elements of tuple
    let pi_by_direct_access = tup.0;
    println!("pi by direct access: {}", pi_by_direct_access);

    //     (2) The Array Type
    //         ---
    //         General Notes:
    //         - Like a tuple - groups elements, comma-separated,
    //           are of a static length
    //         - Unlike a tuple - enclosed in square brackets,
    //           must all be of same type
    //         TLDR:
    //         - Use them when data should be on allocated on the
    //           stack versus on the heap
    //         - Use when you have a fixed number of elements
    //         - It is not as flexible as the vector type because,
    //           which IS allowed to grow/shrink
    //         - When in doubt, you should probably use a vector
    let beatles = ["John", "Paul", "Ringo", "George"];
    let _array_with_type_and_count_annotation: [i32; 5] = [2, 3, 5, 7, 11];
    //         - Brackets with a usize index `[usize]` retrieve elements
    //           from the array
    let _ringo = beatles[2];
    let _george = beatles[beatles.len() - 1];

    // Accessing an index that is out of bounds will not cause a compile-time
    // error, but at runtime Rust will check that the index specified is less
    // than the arr.len(). If this check fails, Rust will `panic`, exiting
    // immediately instead of allowing invalid memory access and
    // continuing with a potential attack vector.
}
