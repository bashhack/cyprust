use std::f64::consts::PI;

// While we could use the following macro, I feel that we should aim to avoid
// this type of override to ensure that 'dead code' is visually identifiable
// by way of using the leading `_` before the name (e.g., `_unused`)

// #![allow(dead_code)]

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

// Per the docs, enums 'allow you to define a type by enumerating its possible
// values.' Having some resemblence (and utility) comparable to a struct, the
// enum gives us a way to represent our data in the form of variants.
// Each of these variants may - in turn - be defined in terms of associated
// data.
//
// Whereas we might have many individual struct definitions, the enum is a
// single data type; its value matching any of the variants.
//
// In other languages, we might refer to how `enum` behaves as a 'sum type.'
// A sum type (at least in the terms I am familiar with it - that is,
// as Haskell defines it) is an algebraic data type. The sum type (or
// tagged union/variant/choice type/disjoint union) is a data structure
// that is used to store different, fixed types. Only one of these types may
// be used at a time, and a 'tag' field denotes which is in use. Sum types
// are the dual of product types - I remember the difference between them as
// sum types use 'or' and product types use 'and'.
// Sum types are useful in that they save storage by overlapping storage
// for all the types. They are disadventageous, however, because the tag
// does occupy space. To work around this concern alternatives like
// folded tags do exist where a tag value is computed dynamically from the
// union field.
//
// NOTE: Bartosz's paper 'Category Theor for Programmers' is indespensible:
// https://bartoszmilewski.com/2014/10/28/category-theory-for-programmers-the-preface/

#[derive(Debug)]
enum Direction {
    // Instead of repeating the coordinate tuple for each entry,
    // we can abstract the repetition with a struct
    // Up(i32, i32),
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Shape {
    Rectangle { width: u32, height: u32 },
    Square(u32),
    Circle(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            // We'll cast to `f64` a consistent return type across patterns
            // This casting has allowed us to make a polymorphic function -
            // that is, this works for many data types
            Shape::Rectangle { width, height } => (width * height) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => PI * r.powi(2),
        }
    }
}

// ----------------------------------------------------------------------------
// Option Enum
// ----------------------------------------------------------------------------

// Hey, this is like our Maybe monad in Haskell!
//
// This is a data type about encapsulation. Where we don't have a null/nil (
// as in Rust) - or where we don't want to use a null because it's the freaking
// worst! - we can use Option/Maybe to signify the presence or absence of
// value.
//
// In Rust, the Option enum is `enum Option<T> { None, Some(T) }` where
// <T> is our generic type.
//
// We can think of this monadic structure like a box, of sorts. As such,
// we need a way of unpacking or 'unwrapping' a value from the box.
// To do this, we often see patterns of utilizing `match` or `.expect()`.
// NOTE: Don't blindly use `.expect()` or `.unwrap()` - use their safer
//       counterparts: `.unwrap_or_default()` or `.unwrap_or()`
// For example:
//
// let a_monad: Option<int> = Some(1);
// let value_from_match = match a_monad {
//    Some(x) => x,
//    None => 0i, // a reasonable fallback, the complex number 0i
// };
// let value_from_expect = a_monad.expect("No result");
// let value_or_default = a_monad.unwrap_or_default();
// let value_or_fallback = a_monad.unwrap_or(42i);
//
// So, why use it? We should use Options because it enforces (1) a style of
// writing code that accounts for all possible return types (including where
// we have 'null' cases), (2) they wrap values and not pointers (we avoid
// having pointers because 'null' requires a pointer to be instantiated),
// and (3) we can use function composition within transformation pipelines
// to handle errors simply, without having to write error checking at
// every function application step.
//
// Another good example is provided here:
// let number: f64 = 20.;
// let result = Some(number)
//     .map(inverse)
//     .map(double)
//     .map(inverse)
//     .and_then(log)
//     .map(square)
//     .and_then(sqrt);
// match result {
//     Some(x) => println!("Result was: {}", x),
//     None => println!("Functional pipline failed"),
// }
//
// We can benefit from the Option enum here because when we look at the
// return type in the function signatures of `map` and `and_then` we see:
//
// Functor Interface (NOTE: Every monad is a functor)
// `fn map<U>     (self, f: |T| -> U)         -> Option<U>`
//
// Monad Interface
// `fn and_then<U>(self, f: |T| -> Option<U>) -> Option<U>`
//
// Looking at Rust and Haskell again, Rust's `map` is like Haskell's
// `fmap` and `and_then` is like `bind`.
//
// `map` is useful for functions that won't return an `Option` - in other
// words, those that will return a value
//
// `and_then` is useful for function that may return no value,
// in the case above `sqrt()` is a good candidate.
//
// NOTE: I prefer Haskell's type annotations, but - Rust seems to
//       really capture much of what I really like about a well-defined
//       type system. Realizing that not everyone might know or understand
//       the annotations U and (though not used here) V in the context
//       of the signatures for `map`, etc. - these are used for generic type
//       after T has been used, so we say - for Rust - that map "takes a
//       function with input of some generic T and transforms that T into
//       a generic type U, returning the wrapped Option of the computed
//       generic type U." This use of `<T>`, `<U>`, and `<V>` is analogous
//       to how I would write `a`, `b`, and `c` in a Haskell signature
//
// Without further ado - here we have a function that may or may not
// return a value - let's have its return value be `<Option<f64>` because
// we may return `Some` `f64` value or maybe just `None` :)

fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let res = division(5.0, 7.0);
    match res {
        Some(x) => println!("{:.10}", x), // just as in Python, we can format
        None => println!("Hey, no dividing by zero!"),
    }

    let rectangle = Shape::Rectangle {
        width: 10,
        height: 50,
    };
    let square = Shape::Square(10);
    let circle = Shape::Circle(4.5);

    let area_of_rectangle = rectangle.area();
    println!("Area of rectangle: {}", area_of_rectangle);

    let area_of_square = square.area();
    println!("Area of square: {}", area_of_square);

    let area_of_circle = circle.area();
    println!("Area of circle: {}", area_of_circle);

    let up = Direction::Up(Point { x: 0, y: 1 });
    let down = Direction::Down(Point { x: 0, y: -1 });
    let left = Direction::Left(Point { x: -1, y: 0 });
    let right = Direction::Right(Point { x: 1, y: 0 });

    let up_key = up.match_direction();
    let _down_key = down.match_direction();
    let _left_key = left.match_direction();
    let _right_key = right.match_direction();

    let up_key_deconstructed = up_key.destruct();

    // println!("{:?}", up_key);
    println!("{:?}", up_key_deconstructed);

    let ten = 10;
    let reference_to_ten_with_ampersand = &ten;
    let ref reference_to_ten_by_ref_keyword = ten;

    if reference_to_ten_with_ampersand == reference_to_ten_by_ref_keyword {
        println!("`&` and `ref` are equivalent ways of accessing a reference");
    }
}
