fn main() {
    // ------------------------------------------------------------------------
    // How Memory Works
    // ------------------------------------------------------------------------

    // Computer memory is used to store information for immediate use,
    // the content of which may be transfered to other storage on the machine.
    // Memory is a physical entity - formally, memory is known as 'addressable
    // semiconductor memory' (i.e., silicon-based transistors).
    // There are volatile and non-volatile types of memory; the former,
    // requiring power to maintain stored information, and the latter
    // being able to retrieve information without having to be power cycled.
    //
    // Memory is typically composed of memory cells, each storing one bit
    // (0 or 1). Cells are grouped into 'words' of fixed 'word length'
    // (1, 2, 4, 8, 16, 32, 64, 128 bits). Each word may be accessed by a
    // binary address of N bit, thus we can store 2^N words in memory.
    //
    // Memory management is a blanket term for the complicated and vital
    // processes, protocol, and policies that allow the computer to
    // dynamically allocate memory to programs and to free memory when
    // it is no longer needed.
    //
    // Manual memory management provides a means of detecting, allocating,
    // and deallocating objects in the store. Manual memory management
    // is the root of several critical bugs when handled incorrectly.
    //
    // Garbage collection handles some of the classes of vulnerabilites,
    // but not all. Rust does not use garbage collection but enforces
    // memory safety via a system of ownership.

    // ------------------------------------------------------------------------
    // Stack versus Heap
    // ------------------------------------------------------------------------

    // The stack and heap are each parts of memory that are used to store
    // information, but their structure differs. These differences, in turn,
    // relate to their utilization.
    //
    // The stack 'stores values in the order it gets them and removes the
    // values in the opposite order.' In other words, the stack is LIFO (last
    // in, first out). We "push onto" and "pop off" data from the stack.
    // Because new data is always placed at the 'top' of the stack,
    // there need not be any searching for a place to put new data or
    // to get data from. Additionally, data placed on the stack must be of a
    // 'known, fixed size.' These properties combined make the stack 'fast.'
    //
    // The heap, in contrast, is not as organized and lets us store data
    // that might change or is of an unknown size at compile time.
    // When data is to be added to the heap, a search is undertaken for a
    // place in memory of the size requested. The OS finds an available
    // spot in the heap, marks it as used, and returns a pointer to an
    // address in memory. We call this 'allocating on the heap.'
    // Because the system must follow this pointer via jumps in memory,
    // we sometimes refer to the heap as 'slower' than the stack.
    // Per the Rust docs, 'managing heap data is why ownership exists.'

    // ------------------------------------------------------------------------
    // Ownership and Borrowing
    // ------------------------------------------------------------------------

    // At the most basic level, ownership and borrowing provides us with both
    // a conceptual framework and language features to work with and manage
    // memory - safely, and securely.
    //
    // Per the docs, 'ownership is Rust's most unique feature, and
    // enables Rust to make memory safety guarantees without needing
    // a garbage collector.'
    //
    // Unlike an automated garbage collector or execution of manual calls
    // by an engineer to `malloc`, `calloc`, or `free` (as in C, for example),
    // Rust uses ownership to define a set of rules the compiler checks
    // against at compile time.
    //
    // There are three fundamental rules for ownership:
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    //
    // In languages with a garbage collector (a GC), the GC tracks memory
    // and cleans up (to 'free') memory once it's not being used. Without
    // it, we must identify when memory isn't being used and when it's needed.
    // This is hard work, without a doubt - and when we make mistakes here,
    // we open ourselves up to an entire class of vulnerabilities that
    // pose serious potential for risk.
    //
    // To avoid these pitfalls, in Rust memory is 'automatically returned
    // once the variable that owns it goes out of scope.' It is at that
    // moment where a variable goes out of scope that Rust internally calls
    // a function `drop` - this handles a process akin to
    // 'Resource Acquisition is Initialization (RAII)' in C++.
    //
    // The process of taking ownership and returning it with every function
    // is made easier with Rust's 'references.'
    // We can both reference (denoted by an `&` operator) and
    // dereference (denoted by an `*` operator) a value.
    // Referencing a value allows us to 'refer to some value without
    // taking ownership.' A reference to a value will be dropped when
    // the reference is out of scope.
    //
    // From the docs, a good basic example might look like:
    fn calculate_length(s: &String) -> usize {
        // s is a reference to a String
        s.len()
    } // Here, s goes out of scope. But because it does not have ownership of what
      // it refers to, nothing happens.

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // By default, as with variables, borrows are immutable!

    // If we do need to borrow and then mutate, we can use the form: `&mut`
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    let mut s = String::from("hello");

    change(&mut s);

    println!("The value of 's', after mutation, is {}", s);

    // ------------------------------------------------------------------------
    // Data Race Mechanics
    // ------------------------------------------------------------------------

    // As mentioned, the system of ownership applied in Rust prevents many
    // categories of errors. Among them, and discussed in detail in the docs,
    // is data race conditions.
    //
    // Data races occur when:
    // 1. Two or more pointers access the same data at the same time.
    // 2. At least one of the pointers is being used to write to the data.
    // 3. There’s no mechanism being used to synchronize access to the data.
    //
    // Ownership prevents data races - with its restrictions enforcing:
    // 1. That we may have multiple references, but no simultaneous ones.
    // 2. That no mutable references exists while we have an immutable one.
    //

    // ------------------------------------------------------------------------
    // Dangling References
    // ------------------------------------------------------------------------

    // In C, we can be left in a position where we have a dangling pointer,
    // one that references a memory address that could be invalid.
    // In Rust, this is avoided becaused the compiler will prevent us from
    // having data out of scope before its reference.
    //
    // An example is provided from the docs:
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //
    //     &s
    // }
    //
    // let reference_to_nothing = dangle();
    //
    // This will not work, because we return a reference to the String, s
    // and s is out of scope past the `{` or `}`.
    //
    // If we had simply returned the `String` directly it would be no problem:
    fn no_dangle() -> String {
        let s = String::from("hello");

        s
    }

    let _reference_to_something = no_dangle();

    // ------------------------------------------------------------------------
    // The Slice Type
    // ------------------------------------------------------------------------

    // It seem odd to talk about the `slice` data type in the context of
    // references, but it is (like the `String` type) a data type without
    // ownership. The `String` type is allocated on the heap (unlike previous
    // data types we've encountered that are all on the stack).
    //
    // Slices 'let us reference a contiguous sequence of elements in a
    // collection rather in the whole collection.'
    //
    // We denote a 'string slice' type as `&str`.
    //
    // To use create slices, we use syntax that is quite Pythonic:
    let a_string = String::from("Pythagoras");
    let slice_1 = &a_string[0..4];
    let slice_2 = &a_string[..4];
    let slice_3 = &a_string[4..7];
    let slice_4 = &a_string[4..=7];
    let slice_5 = &a_string[5..];
    let slice_6 = &a_string[5..a_string.len()];
    let slice_7 = &a_string[..];
    println!("'slice_1' is: {}", slice_1);
    println!("'slice_2' is: {}", slice_2);
    println!("'slice_3' is: {}", slice_3);
    println!("'slice_4' is: {}", slice_4);
    println!("'slice_5' is: {}", slice_5);
    println!("'slice_6' is: {}", slice_6);
    println!("'slice_7' is: {}", slice_7);

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let s = String::from("hello world");

    // works on slices of `String`s
    let _word = first_word(&s[..]);
    let string_literal = "hello world";
    // works on slices of string literals
    let _also_word = first_word(&string_literal[..]);
    // string literals are already slices, so we could have written
    let _also_word = first_word(string_literal);

    // ------------------------------------------------------------------------
    // More Examples
    // ------------------------------------------------------------------------

    // Example 1
    // Below, we create a heap-allocated String value, when we then
    // to copy that value to `string_moved` we are unable to use
    // our `string_created` again. We call this 'moving', we could have
    // solved this by using our borrowing syntax.
    let string_created = String::from("Ramanujan");
    let _string_moved = string_created;
    let other_string_created = String::from("Hardy");
    let _string_borrowed = &other_string_created;
    println!("Would not work to reference 'string_created' - it's been moved!");
    println!(
        "We do have access to 'other_string_created' ({}) because we borrowed, not moved",
        other_string_created
    );

    // Example 2
    fn take_ownership_but_do_not_return_it(v: Vec<i32>) {
        println!("We took (or 'moved') 'v': {}", v[10] + v[100]);
    }

    let mut v = Vec::new();

    for i in 1..1000 {
        v.push(i);
    }

    take_ownership_but_do_not_return_it(v);

    // the following will fail because the vector `v` was moved by `take_ownership`
    // println!("{}", v[0]);

    // Example 3
    // Unlike a vector or string value being passed to a function -
    // both data types that are stored on the heap - `a` and `b` below
    // are scalar values that exist on the stack.
    //
    // Though `copy_value` takes `a` and `b`, there is no moving occuring.
    //
    // That is, in the examples above (for instance) we said that ownership
    // was moved - and once ownership of our heap allocated `v` or
    // `string_created` variables were moved, we were unable to access them
    // in the 'main' scope(s) (the scope in which they was declared).
    // Here, `a` and `b` in the following do not become unallocated within
    // the scope they were defined because the scalar values are internally
    // copied by Rust - that is, they are accessible in their original scope
    // and within the `copy_value` scope.
    fn copy_value(a: i32, b: i32) {
        println!("{}", a + b)
    }

    let a = 32;
    let b = 45;

    copy_value(a, b);

    println!("We have still have access to a: {} and b: {}", a, b);

    // Example 4
    fn move_and_return_ownership(vect: Vec<i32>) -> Vec<i32> {
        println!("{:?}", vect[120] + vect[111]);
        vect
    }

    fn borrow_with_dereferencing(vect: &Vec<i32>) {
        println!("{}", (*vect)[10] + (*vect)[12]);
    }

    fn idiomatic_rust_borrow(vect: &Vec<i32>) {
        println!("{}", vect[10] + vect[11]);
    }

    let mut vect = Vec::new();

    for i in 1..1000 {
        vect.push(i);
    }

    vect = move_and_return_ownership(vect);
    println!("Still own vect: {} {}", vect[0], vect[1]);

    borrow_with_dereferencing(&vect);
    println!("Still own vect: {} {}", vect[0], vect[1]);

    idiomatic_rust_borrow(&vect);
    println!("Still own vect: {} {}", vect[0], vect[1]);

    // Example 5
    // We are not transfering ownership of the vector, using a reference
    // to the vector and borrowing in the loop and in the function `count`
    let v = vec![2, 4, 2, 5, 7, 2, 4, 5, 5, 2, 1, 7, 5, 8, 8, 2, 8];
    for &val in &v {
        println!("v {:?}", &v);
        println!("val {}", val);
        let r = count(&v, val);
        println!("{} is repeated {} times", val, r);
    }

    fn count(v: &Vec<i32>, val: i32) -> usize {
        // NOTE: In the body of the function, we want to iterate over the
        //       borrowed vector (passed from the previous function as `&v`
        //       and taken in here as `v`).
        //       When we create the iterator over the borrowed vector,
        //       the iterator is made by taking a reference of the borrowed
        //       vector. The type of the incoming `v` is, of course, `&Vec`,
        //       effectively then `v` is actually of a type `& (&v)`.
        //       The iterator takes that reference of the borrowed vector
        //       and for each element creates a reference to that element.
        //       So it is that within the filter function's closure, our
        //       type of `x` is in fact `&&i32`. It is being compared to
        //       `val` which is of type `i32`.
        //       These types are not directly comparable, so we cannot
        //       make use of the `PartialEq` trait which would have allowed
        //       us to perform equality checks on like types.
        //       As a result, we add `&&` to the parameter `x` in our
        //       lambda function passed to the filter function to ensure that
        //       we are dealing with a value that may be compared to `val`
        //       - that is, we need to be seeing an `i32` for both.
        //       Had we instead made the type of `val` into a `&i32`, we'd
        //       only have to write one reference on `x` as `&x`.

        // NOTE: Instead of using references `&&`, we could have used
        //       raw pointers - again taking into account the layers of
        //       referencing that occured - by writing `**x` in the body
        //       of the lambda function provided to filter.

        // NOTE: Okay - references and raw pointers - when do I use one
        //       versus the other? What gives?
        //       Answering this, I'm sure, is nuanced - but initially
        //       I find benefit in the docs, where it is stated:
        //
        //           "Much of Rust’s safety comes from compile-time checks,
        //            but raw pointers don’t have such guarantees,
        //            and are unsafe to use."
        //
        //       Moreover:
        //
        //           "Here are some things to remember about raw pointers
        //            that are different than other pointer types. They:
        //            - are not guaranteed to point to valid memory and
        //              are not even guaranteed to be non-NULL
        //              (unlike both Box and &);
        //            - do not have any automatic clean-up, unlike Box,
        //              and so require manual resource management;
        //            - are plain-old-data, that is, they don't move ownership,
        //              again unlike Box, hence the Rust compiler cannot
        //              protect against bugs like use-after-free;
        //            - lack any form of lifetimes, unlike &, and so the
        //              compiler cannot reason about dangling pointers; and
        //            - have no guarantees about aliasing or mutability other
        //              than mutation not being allowed directly through a
        //              *const T."
        //
        //       Also:
        //
        //           "At runtime, a raw pointer `*` and a reference pointing to
        //            the same piece of data have an identical representation
        //            ....When writing certain types of libraries, you'll need
        //            to get around Rust's safety guarantees for some reason.
        //            In this case, you can use raw pointers to implement your
        //            library, while exposing a safe interface for your users
        //            ....Raw pointers are useful for FFI: Rust's `*const T`
        //            and `*mut T` are similar to C's `const T*` and `T*`,
        //            respectively."
        //
        //       Speaking of C code, raw pointers are typically most commonly
        //       used when interfacing with C code - and in a few other
        //       specific patterns.
        //
        //       If we aim to ever dereference a raw pointer, we must do so
        //       using the `unsafe` keyword to declare an unsafe block or
        //       unsafe function. As stated in the docs:
        //
        //           "In both unsafe functions and unsafe blocks, Rust will let you do
        //            three things that you normally can not do. Just three.
        //            Here they are:
        //            - Access or update a static mutable variable.
        //            - Dereference a raw pointer.
        //            - Call unsafe functions. This is the most powerful ability."

        // Option 1 (via raw pointer):
        // v.into_iter().filter(|x| **x == val).count()

        // Option 2 (via reference):
        v.into_iter().filter(|&&x| x == val).count()
    }
}
