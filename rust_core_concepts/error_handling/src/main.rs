use std::fs::File;
use std::io::{Error, ErrorKind, Read};

// (A)
// fn exit(x: i32) {
//     if x == 0 {
//         panic!(" we got a 0");
//     }
//     println!("things are fine!");
// }

// (B)
fn exit(x: Option<i32>) {
    match x {
        Some(0) => panic!("we got a 0"),
        Some(x) => println!("we got a {} - things are fine!", x),
        None => println!("we got nothing"),
    }
}

// (C) Error propagation
fn _read_file_with_error_propagation() -> Result<String, Error> {
    let f = File::open("text.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// (D) Error propagation with the `?` operator - it's type is: `M<T, E> -> T`
fn _read_file_with_question_mark_operator() -> Result<String, Error> {
    let mut f = File::open("text.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// (E) More concise version of (D)
fn _read_file_with_question_mark_operator_but_more_concise() -> Result<String, Error> {
    let mut s = String::new();
    File::open("text.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// NOTE; The "Big Takeaway":
//       If your code could end up in a bad state, usually it's advisable
//       to use a `panic!`. For things like invalid values, contradictory
//       values, or missing values, we should probably handle them with
//       a `panic!` (or with things that produce a `panic!`) because
//       "bad state" isn't something we generally expect to happen -
//       though it does occur. Another case where we might want to use
//       a `panic!` is if someone intentionally calls your code with
//       something that doesn't make sense - then a `panic!` seems
//       the most appropriate response.
//       Likewise, when we are calling external code that is out of our
//       control (i.e., reading JSON, reading from a web server, etc.)
//       we are dealing with actions that are good candidates for `panic!.`
//       Lots of error checking is annoying, but we do have the benefit
//       of a robust type system does help us catch many errors and avoid
//       a great deal of manual error handling.

fn main() {
    let v = vec![1, 2];

    // v[2];

    // For (A)
    // exit(1);
    // exit(0);

    // For (B)
    exit(Some(1));
    exit(Some(10));
    exit(None);
    // exit(Some(0));

    // Using verbose error handling...
    let f = File::open("text.txt");
    let _f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("text.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!("could not create file {:?}", e);
            }
        },
        Err(error) => {
            panic!("could not open the file: {:?}", error);
        }
    };

    // ...the same as the above but using some quick rudamentary error handling -
    // not suitable for production code - but useful for quick dev or prototyping, though
    let _f = File::open("text.txt").unwrap();
    // // `.expect()` will expect an Ok() but automatically panic! otherwise
    let _f = File::open("text.txt").expect("could not open file");

    // NOTE: For idiomatic, production-ready Rust error handling,
    //       use something like (D) or (E) above
}
