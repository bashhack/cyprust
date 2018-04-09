fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn with_return_value() -> i32 {
    5
}

fn main() {
    // Functions are defined using the `fn` keyword

    // Function names are conventionally written in snake case

    // `main` is our primary entry point, other functions may be declared anywhere else

    // Parts of a Rust function:
    //   - `fn` keyword
    //   - name
    //   - parameters
    //   - body (statements/expressions)
    //   - return

    let _y = 6; // a statement, does not return a value

    // NOTE: As an example, the following does not work because statements don't return values
    // let _wont_work = (let this_is_a_statement = 10);

    let z = { // an expression, returns a value
        let inside_of_expression = 3;
        inside_of_expression + 1
    };

    another_function(5, 6);

    println!("Value of z: {}", z);

    let get_five = with_return_value();
    println!("Value of call to get_five: {}", get_five);
}
