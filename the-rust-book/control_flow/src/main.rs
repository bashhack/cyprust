fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number == 3 {
        println!("number was: {}", number);
    }

    if number % 2 == 0 {
        println!("number is even");
    } else {
        println!("number is odd");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // We can use an `if` expression in a `let` statement, but all branch values must be of same type
    let condition = true;

    let other_number = if condition { 10 } else { 20 };
    println!("Value of `other_number` is: {}", other_number);

    // Rust has three kinds of loops: `loop`, `while` and `for`

    // - loop
    let mut counter = 0;

    loop {
        if counter == 5 {
            println!("breaking out of loop on condition met");
            break;
        }

        println!("counter: {}", counter);

        counter += 1;
    }

    // - while
    let mut decrement = 5;

    while decrement != 0 {
        println!("decrement is: {}", decrement);

        decrement -= 1;
    }

    println!("broken out of while loop! {}", decrement);

    // - for
    let books = [
        "Hobbit",
        "Fellowship of the Ring",
        "Two Towers",
        "Return of the King",
        "Silmarillion",
    ];
    let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", books[index]);
    //     index += 1;
    // }

    for book in books.iter() {
        println!("the value is: {}", book);
    }

    //   -- iterating over a range
    for num in (1..11).rev() {
        println!("{}", num);
    }
}
