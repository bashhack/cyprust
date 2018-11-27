use std::mem;

fn main() {
    // ------------------------------------------------------------------------
    // Tuples
    // ------------------------------------------------------------------------

    // Our simplest tuple is, of course, the empty tuple (this is a unit value)
    // Functions that don't return anything technically returns an empty tuple
    // ...that is, it is like our None in Python and Undefined in JavaScript
    let _empty_tup = ();

    let tup = (3.14, 'x', false);
    let tup_with_nested_tup = (2, (3.14, 'x', false));
    println!("{} {} {}", tup.0, tup.1, tup.2);

    // Not all tuples may be printed with Debug trait, the following is too
    // large to be displayed. Here, the resulting compile-time error is:
    // `std::fmt::Debug` is not implemented
    let _t = (1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15);
    // println!("{:?}", t);

    // ------------------------------------------------------------------------
    // Pretty Printing/Debug Printing
    // ------------------------------------------------------------------------

    // Using a debug flag `:?`, accessible because tuples have the debug trait
    println!("Debug Printing: {:?}", tup_with_nested_tup);
    // To pretty-print, we can use `:#?`
    println!("Pretty Printing: {:#?}", tup_with_nested_tup);

    // ------------------------------------------------------------------------
    // Arrays
    // ------------------------------------------------------------------------

    // Data type and element count can be specified in the signature
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Array access is performed using a usize index `[usize]`,
    // array length can be easily accessed with a call to `len()`
    // and we can retrieve the size of 'pointed-to value in bytes' per the docs
    println!("{} {} {}", xs[0], xs.len(), mem::size_of_val(&xs));

    // Slicing an array is similar to other languages, with a
    // `[start_index..end_index]` where the end_index is exclusive, and
    // the start_index is inclusive
    let ys = &xs[2..4];
    println!("{:?}", ys);

    // Attempting to access an element that is out of bounds will result in a
    // runtime error: in this case, Rust will return a `panic` on the thread
    // due to an 'index out of bounds' error:
    // println!("{:?}", ys[2]);

    // ------------------------------------------------------------------------
    // Strings
    // ------------------------------------------------------------------------

    let _not_a_string_actually_a_slice = "Fermat's Last Theorem";
    let true_string = String::from("Riemann zeta function");
    let _string_by_way_of_to_string_method = "Gaussian distribution".to_string();

    let _slicing_of_string = &true_string[8..12]; // "zeta"
}
