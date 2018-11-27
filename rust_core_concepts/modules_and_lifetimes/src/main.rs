extern crate modules_and_lifetimes;

pub mod k {
    pub mod l {
        pub mod m {
            pub mod n {
                pub fn o() {}
            }

            fn _another_function() {}
        }
    }
}

pub mod p {
    pub mod q {
        pub mod r {
            pub fn s() {}
        }
    }
}

enum EnumsMakeTheirOwnNamespace {
    _H,
    _I,
    J,
}

use self::k::l::m::n;
use self::p::q::{r};
use self::EnumsMakeTheirOwnNamespace::*;
use modules_and_lifetimes::C;

fn main() {
    // ------------------------------------------------------------------------
    // Modules/Imports
    // ------------------------------------------------------------------------

    J;

    n::o();

    r::s();

    C::b();

    // ------------------------------------------------------------------------
    // Lifetimes
    // ------------------------------------------------------------------------

    // The borrow-checker determines whether all borrows are valid - a
    // 'lifetime' is the term used to describe the scope that a reference
    // is valid for.

    // The following would not work, because the lifetime of y - and
    // consequently the assignment to x of the borrowed value - does not
    // live lone enough.

    // (A) - will fail
    // let x;

    // {
    //     let y = 10;

    //     x = &y;
    // }

    // println!("{}", x);

    // (B) - will work
    let x;
    let y = 10;

    x = &y;

    println!("{}", x);

    // In the following, the return type `&str` is created inside the
    // `if` scope and we cannot guarantee that the borrow will exist
    // long enough to be returned to our main function
    // We use the lifetime specifier syntax - "a prime" or "b prime", etc.
    // - to help Rust compile our code as intended.
    // This tells Rust that our parameters `a` and `b` have the same
    // lifetime as our output string (`x`, `y`)

    // (A) - will fail
    // fn pr(x: &str, y: &str) -> &str {
    //     if x.len() == y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // let c = pr(a, b);

    // (B) - will work
    fn pr<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() == y.len() {
            x
        } else {
            y
        }
    }

    let a = "'a' string";
    let b = "'b' string";
    let c = pr(a, b);
    println!("{}", c);

    // Under the hood, any time we have a function that takes an argument
    // by reference, we can be implicit or explicit about the lifetime:

    // Implicit
    fn _foo(_x: &i32) {
        // do something with x
    }

    // Explicit
    fn _bar<'a>(_x: &'a i32) {
        // do something with x
    }

    // References aren't just for functions though!
    // We can also use references inside of structs:

    #[derive(Debug)]
    struct A<'a, 'b> {
        x: &'a str,
        y: &'b str,
    }

    let instance_of_a = A{x: "Hello", y: "Rust"};
    println!("{:#?}", instance_of_a);

    fn _ab<'a, 'b>(_x: &'a i32, _y: &'b i32) {}
    fn _a<'a>(_x: &'a i32) {}

    impl <'a, 'b> A<'a, 'b> {
        fn _slf(&self) -> &str {
            self.x
        }
    }

    // NOTE: There is a special lifetime specifier available to us, but we
    //       should almost always avoid its usage due to the serious potential
    //       for degredation of performance in your program!

    let _static_a: &'static str = "The Long Running String";
}
