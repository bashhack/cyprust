// ----------------------------------------------------------------------------
// Box
// ----------------------------------------------------------------------------

// A pointer is a variable that contains an address in memory, rather
// than an actual value - it points to some other data, hence the name "pointer"
//
// In Rust, a smart pointer are data structures that act like a pointer but
// has additional metadata. Of these smart pointers, the most useful is the `Box`.
//
// primitive in Rust is allocated to the stack - but, with the `Box`,
// we can allocate data directly on the heap.

// NOTE: The intuition should be that `Box` works like a Rust `reference`

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    End,
}

use self::List::Cons;
use self::List::End;

// ----------------------------------------------------------------------------
// Closures (Lambdas/Anonymous Functions)
// ----------------------------------------------------------------------------

// Closures in Rust are simply lambdas - anonymous functions. They can be
// saved as variables, be passed as arguments to other functions, and can
// themselves generate and return other closures.

// We would noramlly write:
fn _traditional_function(i: i32) -> i32 {
    i + 1
}

// Transforming that into a closure (or anonymous/lambda function), we'd write:
// let lambda_function = |i: i32| -> i32 i + 1;

fn run<F: Fn()>(f: F) {
    f();
}

fn add3<F: Fn(i32) -> i32>(f: F) -> i32 {
    f(3)
}

fn a_normal_function() {
    println!("this came from a normal function!");
}

fn create() -> Box<Fn()> {
    Box::new(move || println!("this is a closure in a box"))
}

// ----------------------------------------------------------------------------
// Iterators
// ----------------------------------------------------------------------------

// When we speak of iterators, we're talking about a pattern that allows us to
// execute a task on a sequence of items, one after another.
//
// An iterator - itself - is 'responsible for the logic of iterating over each
// item and determining when the sequence has finished.'
// In Rust, as in Haskell, iterators are lazy - that is, the evaluation of
// an expression is delayed until its value is needed.
//
// Rust iterators share a trait named `Iterator` that is defined in the
// standard library as:

trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn is_even(n: u32) -> bool {
    n % 2 == 0
}

fn main() {
    // ------------------------------------------------------------------------
    // Box
    // ------------------------------------------------------------------------

    let b = Box::new(10);
    println!("b = {}", b);

    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));
    println!("{:?}", l);

    let y = 4;
    let x = &y;
    let z = Box::new(y);

    if *x == *z {
        println!("true");
    }

    // ------------------------------------------------------------------------
    // Closures
    // ------------------------------------------------------------------------

    let add_one_without_types = |i| i + 1;
    let add_one_with_types = |i: i32| -> i32 { i + 1 };
    let ten = 10;
    let twenty = 20;
    println!("{}", add_one_without_types(ten));
    println!("{}", add_one_with_types(twenty));

    let a_closure_with_no_params = || println!("this is a closure with no parameters");
    a_closure_with_no_params();

    let mut zero = 0;
    let mut inc = || {
        zero += 1;
        println!("incremented by 1: {}", zero);
    };

    inc();
    inc();
    inc();
    println!("zero is now three: {}", zero == 3);

    let simple_closure_declaration = || println!("hello from run function!");
    run(simple_closure_declaration);

    let mult_by_ten = |i| i * 10;

    println!("3 * 10 = {:?}", add3(mult_by_ten));

    run(a_normal_function);

    let returns_a_closure = create();
    returns_a_closure();

    // ------------------------------------------------------------------------
    // Iterators
    // ------------------------------------------------------------------------

    let vect = vec![1, 2, 3];
    println!("v {}", vect.iter().any(|&x| x != 2));

    println!("{:?}", vect.iter().next());

    // Imperative version...
    let top = 10000;
    let mut counter = 0;

    for n in 0.. {
        let x = n * n;

        if x >= top {
            break;
        } else if is_even(x) {
            counter += x;
        }
    }
    println!("Imperative: {}", counter);

    // Functional version...
    let s: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < 10000)
        .filter(|&n| is_even(n))
        .fold(0, |acc, x| acc + x);
    println!("Functional: {}", s);
}
