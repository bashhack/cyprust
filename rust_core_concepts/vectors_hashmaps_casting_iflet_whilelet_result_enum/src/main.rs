use natural_constants::math::apery;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fs::File;

#[derive(Debug)]
enum NumTypes {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // ------------------------------------------------------------------------
    // Vectors
    // ------------------------------------------------------------------------

    let vector = vec!["some", "dynamic", "data", "from", "api"];

    println!("Iterating over the vector");
    for x in &vector {
        println!("`x` was: ({:?})", x);
    }

    let mut mutable_vector: Vec<&str> = Vec::new();

    mutable_vector.push("streamable data");
    mutable_vector.push("that could be");
    mutable_vector.push("of unknown size");

    println!(
        "Vector: {:?} - its len: {} - its capacity: {}",
        &mutable_vector,
        mutable_vector.len(),
        mutable_vector.capacity()
    );

    println!(
        "Pop! We removed the last element of the vector: {:?}",
        mutable_vector.pop()
    );

    mutable_vector.push("some");
    mutable_vector.push("other");
    mutable_vector.push("elements");

    println!("We just popped an element off the vector, and added three more elements");

    println!(
        "Vector: {:?} - its len: {} - its capacity: {}",
        &mutable_vector,
        mutable_vector.len(),
        mutable_vector.capacity()
    );

    let _typed_vector: Vec<i32> = Vec::new();

    let vector_with_enum = vec![
        NumTypes::Int(42),
        NumTypes::Float(42.),
        NumTypes::Text(String::from("forty-two")),
    ];
    println!("{:?}", vector_with_enum);

    // ------------------------------------------------------------------------
    // HashMap
    // ------------------------------------------------------------------------

    let mut hash_map = HashMap::new();

    hash_map.insert(String::from("first entry"), 42);
    hash_map.insert(String::from("second entry"), 24);
    hash_map.insert(String::from("will be removed entry"), 0);

    for (k, v) in &hash_map {
        println!("Entry was: {}: {}", k, v);
    }

    match hash_map.get(&String::from("second entry")) {
        Some(&n) => println!("Value of entry: {}", n),
        _ => println!("no match for entry key"),
    }

    hash_map.remove(&String::from("will be removed entry"));
    println!("{:#?}", hash_map);

    let physicists = vec![
        String::from("Marie Sklodowska Curie"),
        String::from("Maria Goeppert-Mayer"),
        String::from("Donna Strickland"),
    ];
    let contributions = vec![
        String::from("radiation phenomena"),
        String::from("nuclear shell structure"),
        String::from("generating high-intensity, ultra-short optical pulses"),
    ];
    let female_physicists_hash_map_by_collect: HashMap<_, _> =
        physicists.iter().zip(contributions.iter()).collect();
    println!(
        "Female Recipients of the Nobel Prize in Physics: {:#?}",
        female_physicists_hash_map_by_collect
    );

    // Because it is such a common use case that we may want to check
    // whether a key has a value and, if not, to insert a corresponding value
    // for it, Rust provides the `entry` API.
    // The return value of `entry()` is an enum `Entry` that 'represents
    // a value that might or might not exist.'

    let mut numeric_constants = HashMap::new();

    numeric_constants.insert(String::from("Pi"), PI);

    numeric_constants.entry(String::from("Pi")).or_insert(3.14);
    // Fun fact - how cool is this constant? Apery's constant is the approximate
    // value of the convergent series where `s=3` for the Riemann Zeta Function
    numeric_constants
        .entry(String::from("Apery's Constant"))
        .or_insert(apery);

    println!("Numeric Constants HashMap:\n{:#?}", numeric_constants);

    // Another use case for hash maps is looking up a value by key, and updating
    // it based on its old value - as the docs show in the following example
    // where we traverse a string, counting the occurrences of words:

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // ------------------------------------------------------------------------
    // if-let
    // ------------------------------------------------------------------------
    let s = Some("c");

    match s {
        Some(i) => println!("{}", i),
        _ => {}
    }

    // ...another option compared to the previous `match`
    // is the `if let` binding. It's not exhaustive like `match`
    // - so if had to match on an enum with many fields,
    // we're only looking at a single pattern which
    // may not be comprehensive enough for our needs
    if let Some(i) = s {
        println!("{}", i);
    } else {
        {}
    }

    // ------------------------------------------------------------------------
    // while-let
    // ------------------------------------------------------------------------

    let mut another_s = Some(0);
    // loop {
    //     match another_s {
    //         Some(i) => if i > 19 {
    //             println!("Quit");
    //             another_s = None;
    //         } else {
    //             println!("{}", i);
    //             another_s = Some(i + 2);
    //         },
    //         _ => {
    //             break;
    //         }
    //     }
    // }
    while let Some(i) = another_s {
        if i > 19 {
            println!("Quit");
            another_s = None;
        } else {
            println!("{}", i);
            another_s = Some(i + 2);
        }
    }

    // ------------------------------------------------------------------------
    // Casting
    // ------------------------------------------------------------------------

    // Rust has no implicit type conversion or "coercion".
    // Instead, it provides explicit conversion or "casting".

    let f = 88.2841_f32;
    let i = f as u8;
    let c = i as char;

    println!("Float: {}, Float as u8: {}, u8 Int as Char: {}", f, i, c);
    println!("La cédille: {}", 231 as char);

    // ------------------------------------------------------------------------
    // Result Enum
    // ------------------------------------------------------------------------

    // Like our Option type, the Result enum type is defined in the `std` lib.
    // It has the form: enum Result<T, E> { Ok(T), Err(E) }
    // Instead of expressing the absence or presence of a value, Result
    // expresses the possibility of error. Within the standard library,
    // Result is most used in I/O operations.
    // One of the best parts about the Result type in Rust is that it must
    // be used - we can't simply ignore the return value.
    // If we received the Error variant from the Result type, the Rust compiler
    // will expect us to handle it - perhaps, using a robust handler, or
    // maybe just calling `.expect()` with a simple error message.
    // As the docs point out, writing functions that return a Result type,
    // it can be cumbersome to construct all our error handling. By using
    // the `?` operator, we can use some syntactic sugar to propagate erros
    // up the call stack

    let _f = match File::open("does_not_exist.txt") {
        Ok(file) => file,
        Err(error) => {
            panic!(
                "There was a problem attempting to open the file:\n{:?}",
                error
            );
        }
    };
}
