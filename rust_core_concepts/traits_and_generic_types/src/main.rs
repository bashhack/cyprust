use std::f64::consts::PI;
use std::fmt::Debug;
use std::ops::{Add, Mul};

// ----------------------------------------------------------------------------
// Traits
// ----------------------------------------------------------------------------

// Traits define shared behavior over multiple sets of data.
// Similar to interfaces in other languages, in that they allow us to define
// what a function should look like and allow us to implement it for a given
// data type.
//
// Specifically, the Rust docs state that 'a trait is a language feature that
// tells the Rust compiler about functionality a type must provide.'
//
// We define traits with a method signature (no body), then using an
// `impl` for a type (body present).
//
// While we might often implement a trait for a struct, it should be said that
// we could add a trait for any type - so we could add traits on primitives,
// for example. It should also be said that this would be a terrible idea!

trait Shape {
    fn area(&self) -> u32;
}

#[derive(Debug)]
struct Rectangle {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (PI * self.radius.powi(2)) as u32
    }
}

#[derive(Debug, Clone)]
struct A(i32);

#[derive(Debug, Clone, Copy)]
struct B(f32);

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct C(u32);

struct D;
struct E;

#[derive(Debug)]
struct DE;

#[derive(Debug)]
struct ED;

impl Add<E> for D {
    type Output = DE;

    fn add(self, _rhs: E) -> DE {
        DE
    }
}

impl Add<D> for E {
    type Output = ED;

    fn add(self, _rhs: D) -> ED {
        ED
    }
}

struct WillBeDroppedFromMemory {
    some_property: String,
}

impl Drop for WillBeDroppedFromMemory {
    fn drop(&mut self) {
        println!("dropped {} from memory", self.some_property)
    }
}

struct FibSeq {
    current_num: u32,
    next_num: u32,
}

impl Iterator for FibSeq {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let next_num = self.current_num + self.next_num;
        self.current_num = self.next_num;
        self.next_num = next_num;

        Some(self.current_num)
    }
}

fn fib() -> FibSeq {
    FibSeq {
        current_num: 1,
        next_num: 1,
    }
}

// ----------------------------------------------------------------------------
// Generics
// ----------------------------------------------------------------------------

// Generics are expressions of 'parametric polymorphism' - when the 'type
// of a value contains one or more (unconstrained) type variables, so that
// the value may adopt any type that results from substituting those variables
// with concrete types'.
//
// Parametrically polymorphic values have no idea about the unconstrained
// type variables, so they must behave the same regardless of their type.
//
// In layman's terms, we would say that 'parametrically polymorphic' types
// in type theory are those types or functions that 'have multiple ("poly"
// is multiple, "morph" is form) over a given parameter ("parametric")'.
//
// When looking at a function using generic types, we might see the following:
//   fn takes_anything<T>(x: T) {
//       // do something with x
//   }
//
// Here, we say that 'this function is generic over one type, T' and that this
// function has one parameter `x` and 'x has the type T.'
//
// And, of course, as we've been doing here, we can use generic types in
// functions, as well as structs, enums, and implementations.

struct GenericSquare<T> {
    _x: T,
}

// <T: Debug>, shorthand for 'deriving Debug trait for generic type T'
fn p<T: Debug>(x: T) {
    println!("{:?}", x);
}

struct WillUseImplWithGeneric<T> {
    x: T,
}

impl<U> WillUseImplWithGeneric<U> {
    fn item(&self) -> &U {
        &self.x
    }
}

// We can also use generic types as a sort of pattern creation for structs
struct _X<T, U> {
    x: T,
    y: U,
}

struct _Y<V> {
    x: V,
    y: V,
}

trait GenericShape<T> {
    fn area(&self) -> T;
}

struct GenericRectangle<T: Mul> {
    x: T,
    y: T,
}

// struct GenericCircle<T: Mul> {
//     radius: T,
// }

// impl<T: Mul<Output = T> + Copy> GenericShape<T> for GenericCircle<T> {
//     fn area(&self) -> T {
//         (PI * self.radius.powi(2))
//     }
// }

// impl<T: Mul<Output = T> + Copy> GenericShape<T> for GenericRectangle<T> {
//     fn area(&self) -> T {
//         self.x * self.y
//     }
// }

// As the number of trait bounds increases, the syntax can be cumbersome.
// To work around this, the `where` keyword can help clean things up.
impl<T: Copy> GenericShape<T> for GenericRectangle<T>
where
    T: Mul<Output = T> + Debug + Copy + Clone,
{
    fn area(&self) -> T {
        self.x * self.y
    }
}

fn main() {
    let circle = Circle { radius: 65.4321 };
    let rectangle = Rectangle { x: 24, y: 42 };
    println!(
        "Area of the Circle ({:#?}) was:\n{}\nArea of the Rectangle ({:#?}) was:\n{}",
        circle,
        circle.area(),
        rectangle,
        rectangle.area()
    );

    let a = A(42);
    let b = B(42.);
    let _c = a.clone();
    let d = b;
    println!("`a` was: {:?}", a);
    println!("`b` was: {:?}", b);
    println!("`d` was: {:?}", d);

    println!("Overloading basic Rust operators");
    println!("D + E: {:?}", D + E);
    println!("E + D: {:?}", E + D);

    let will_be_dropped_before_program_end = WillBeDroppedFromMemory {
        some_property: String::from("dropped `will_be_dropped_before_program_end`"),
    };
    {
        let _some_other_variable = WillBeDroppedFromMemory {
            some_property: String::from("dropped `_some_other_variable`"),
        };
        {
            let _yet_another_variable = WillBeDroppedFromMemory {
                some_property: String::from("dropped `_yet_another_variable`"),
            };
            println!("leaving inner scope 2");
            // `_yet_another_variable` will be dropped
            // at the end of the block in which it was defined
        }
        // `_some_other_variable` will be dropped
        // at the end of the block in which it was defined
        println!("leaving inner scope 1");
    }
    // now, we'll drop `will_be_dropped_before_program_end`
    // before its scope has ended (i.e., before Rust compiler would drop it naturally)
    drop(will_be_dropped_before_program_end);
    println!("program for drops has ended");

    println!("Printing first ten Fibonacci nums:");
    for n in fib().take(10) {
        println!("{}", n);
    }

    println!("Printing second ten Fibonacci nums:");
    for n in fib().skip(10).take(10) {
        println!("{}", n);
    }

    let mut slow_fib = fib();

    println!("{:?}", slow_fib.next());
    println!("{:?}", slow_fib.next());
    println!("{:?}", slow_fib.next());
    println!("{:?}", slow_fib.next());
    println!("{:?}", slow_fib.next());
    println!("{:?}", slow_fib.next());
    println!("{:?}", slow_fib.next());
    println!("{:?}", slow_fib.next());
    println!("{:?}", slow_fib.next());
    println!("{:?}", slow_fib.next());

    let _s = GenericSquare { _x: 1 };
    let _s = GenericSquare { _x: 1. };
    let _s = GenericSquare { _x: "One" };
    let _s = GenericSquare { _x: '1' };

    p(1);
    p("One");

    let instance_of_generic_struct = WillUseImplWithGeneric { x: "hello" };
    println!(
        "Property `x` of generic instance: {:?}",
        instance_of_generic_struct.item()
    );

    let generic_rectangle_1 = GenericRectangle { x: 10, y: 20 };
    let generic_rectangle_2 = GenericRectangle {
        x: 30.424,
        y: 40.27,
    };
    println!(
        "Area of `generic_rectangle_1`: {}",
        generic_rectangle_1.area()
    );
    println!(
        "Area of `generic_rectangle_2`: {:.5}",
        generic_rectangle_2.area()
    );
}
