/**
 * Functional definition (a la Haskell) of a linked list,
 * a recursive definition expressed as a sum type (a type
 * that can have different values which may be different
 * types, Rust refers to these types as `enum`s)
 *
 * Ex. List a = Empty | Elem a (List a)
 *
 * We can write Rust's version of this functional definition:
 */

pub enum List {
    Empty,
    Elem(i32, Box<List>)
}


