mod E;
mod F;

mod A {
    fn a() {}
    pub fn b() {}
    mod B {
        fn a() {}
        fn b() {}
    }
}

pub mod C {
    fn a() {}
    pub fn b() {}
    pub mod D {
        pub fn a() {}
        fn b() {}
    }
}

fn test() {
    // A::a(); // function a will be private
    A::b(); // is public via `pub` on function `b`
    // A::B::a() // module B will be private
    C::D::a(); // is public via `pub` on module `D`
    E::a(); // imported from external `E.rs`
    F::a(); // imported from directory `F` module `mod.rs`
    F::G::b(); // imported from directory `F` and sub-module `G.rs`
}
