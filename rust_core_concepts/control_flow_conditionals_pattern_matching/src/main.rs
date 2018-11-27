extern crate natural_constants;

use natural_constants::physics::{
    bohr_radius, classical_electron_radius, gravitational_acceleration, neutron_mass,
    plancks_constant, proton_mass, quantum_of_circulation, von_klitzing_const,
};

fn main() {
    // ------------------------------------------------------------------------
    // Relational Operators
    // ------------------------------------------------------------------------

    // == != > >= < <=

    // ------------------------------------------------------------------------
    // Control Flow
    // ------------------------------------------------------------------------

    // (A) `if` statement
    if gravitational_acceleration >= bohr_radius {
        println!(
            "The value of gravatational acceleration is: {:?}",
            gravitational_acceleration
        );
    } else {
        println!("The value of Bohr's radius is: {:?}", bohr_radius);
    }

    // (B) `if else` statement
    let n: u64 = 2047854210394;
    if n % 4 == 0 {
        println!("n is divisible by 4");
    } else if n % 3 == 0 {
        println!("n is divisible by 3");
    } else if n % 2 == 0 {
        println!("n is divisible by 2");
    } else {
        println!("n is not divisible by 2, 3, 4")
    }

    // `if` statements are expressions - we can use them in variable binding
    let _p = if proton_mass > neutron_mass {
        println!(
            "Proton mass ({}) is larger than neutron mass ({})",
            proton_mass, neutron_mass
        );
    } else {
        println!(
            "Neutron mass ({}) is larger than proton mass ({})",
            neutron_mass, proton_mass
        );
    };

    // ------------------------------------------------------------------------
    // Loop Constructs
    // ------------------------------------------------------------------------

    let mut lift_off = 10;

    // (A) plain `loop`, continues infinitely unless an exit is explicitly added
    loop {
        // NOTE: Without a way to `break` out, the following would print forever
        // println!("...on and on and...");

        lift_off -= 1;

        println!("T-minus {}", lift_off);

        if lift_off == 0 {
            break;
        }
    }

    println!("Value after countdown ({}) - lift off!", lift_off);

    // (B) named, or labelled, `loop`
    'first_loop: loop {
        println!("in first_loop");
        'second_loop: loop {
            println!("in second loop");
            'third_loop: loop {
                println!("in third loop");
                // ...cycle second_loop and third_loop
                // break;

                // break all the way to first_loop,
                // cycling from first_loop through second_loop to third_loop
                break 'second_loop;
            }
        }

        // NOTE: The following would start at begining of first_loop,
        //       replicating the default behavior of continuing the loop
        //       (sans the counter exit below, of course)
        // continue 'first_loop;

        lift_off += 1;
        if lift_off >= 10 {
            break;
        }
    }

    // (C) conditional `while` loop
    while lift_off != 0 {
        println!("Next ship to space in T-minus: {}", lift_off);
        lift_off -= 1;
    }

    // `loop` statements like `if` statements are expressions,
    // so they may be used for variable binding
    let _x = loop {
        break 10;
    };

    // (D) iterative `for` loop
    let physic_constants = vec![
        plancks_constant,
        quantum_of_circulation,
        von_klitzing_const,
        classical_electron_radius,
    ];
    for c in physic_constants {
        println!("constant was: {}", c);
    }

    // exclusive range
    for i in 1..10 {
        println!("i: {}", i);
    }

    // inclusive range
    for i in (1..=10).rev() {
        println!("i: {}", i);
        lift_off += 1;
    }

    // ------------------------------------------------------------------------
    // Pattern Matching
    // ------------------------------------------------------------------------

    // similar to a `case` statement in other languages, but more akin
    // to the type of pattern matching I'm used to in Haskell
    match lift_off {
        0 => println!("zero"),
        2 => println!("two"),
        4 => println!("four"),
        6 => println!("six"),
        8 => println!("eight"),
        _ => println!("fell through, so not 0, 2, 4, 6, or 8 - probably 10!"),
    }

    // We can use rich expressions for the match pattern
    let x = 30;
    match x {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
        // NOTE: Strangely - there is no exclusive range within match statements
        20...29 => println!("between 20 and 29"),
        20...30 => println!("between 20 and 30"),
        // 20..=30 => println!("this'll work too, even Rust has syntax oddities"),
        _ => println!("nope, must be something else"),
    }

    // One creative way to use a match is to destructure incoming data,
    // matching on its constituent parts
    let pair = (true, false);
    match pair {
        (true, y) => println!("y: {}", y),
        (x, false) => println!("x: {}", x),
        _ => println!("no match for pair"),
    }

    // We can replicate Haskell-like 'guards', as well
    let another_pair = (2, -4);
    match another_pair {
        (x, y) if x == y => println!("equal"),
        (x, _) if x % 2 != 0 => println!("x is not even"),
        (_, y) if y % 3 == 0 => println!("y is divisible by 3"),
        _ => println!("no match for pair"),
    }

    // Using the `@` operator, we can dynamically create and bind a variable -
    // in this case, `z` will be declared and defined in the pattern
    let p = 7;
    match p {
        z @ 1...5 => println!("n in first set: {}", z),
        z @ 6...10 => println!("n in second set: {}", z),
        _ => println!("no match on p"),
    }

    // Just as with `if` and `loop` statements, we can evaluate the result
    // of a `match` statement (because it is an expression) and bind
    // it to a variable with `let`
    let p2 = 2;
    let n2 = match p2 {
        n2 @ 1...5 => n2,
        n2 @ 6...10 => n2,
        _ => 0,
    };
    println!("n: {}", n2);
}
