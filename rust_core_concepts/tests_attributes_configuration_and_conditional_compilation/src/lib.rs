// (1) The form #[] wraps an attribute
// (2) `cfg` annotation signifies "configuration"
// (3) `cfg` links to "conditional compilation", that is
//     we conditionally compile the block following
//     the attribute if running the test suite
//
// (4) `mod tests` creates a `tests` sub-module
// (5) The form `#[test]` is an attribute that tells the compiler
//     that the expression following from it should be
//     included in the test suite
// (6) Attributes are simply metadata about our code,
//     Rust builds the binary with these attributes
// (7) There are various attributes for testing, among them
//     `should_panic`

// `assert_eq!` is a macro that assumes the evaluation of its
// arguments will return `true` - if the value is `true`
// the test passes, otherwise the test case fails and a `panic!`
// is thrown. A nice behavior of the macro is that it
// displays the value of the error on failure.

// The attribute `should_panic` is convenient as a way of
// saying "I expect this test to fail with the following..."

// `assert_ne!` is a macro that assert 'not equal'

// We can add custom error messages to our assertions by passing
// a String argument to the third parameter of an assertion.

// We can specify running single tests by name: `cargo test TEST_NAME`

fn internal_add(m: i32, n: i32) -> i32 {
    m + n
}

pub fn add_one(m: i32) -> i32 {
    internal_add(m, 1)
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

// Using our conditional compilation flag again...
// these are not only essential for tests, but
// extremely useful for CLI utilities where
// we might want to specify different behavior per OS.

#[cfg(target_os = "linux")]
fn os_detection() {
    println!("On Linux!");
}

#[cfg(not(target_os = "linux"))]
fn os_detection() {
    println!("Not on Linux!");

    if cfg!(target_os = "Linux") {
        println!("Still on Linux");
    } else {
        println!("Still not on Linux!");
    }
}

fn main() {
    os_detection()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_add_one_to_ten() {
        assert!(10 + 1 == 11);
    }

    #[test]
    fn test_should_fail() {
        assert!(1 == 2);
    }

    #[test]
    #[should_panic]
    fn test_will_fail_if_no_panic() {
        assert!(1 == 2);
    }

    #[test]
    fn test_output_of_assert_eq() {
        assert_eq!(2, internal_add(1, 1));
    }

    #[test]
    fn test_output_of_assert_ne() {
        assert_ne!(3, internal_add(1, 1));
    }

    #[test]
    fn test_custom_msg_on_failure() {
        let result = greeting("Rust");
        assert!(
            result.contains("Python"),
            "Greeting did not contain name, value was: {}",
            result
        )
    }
}
