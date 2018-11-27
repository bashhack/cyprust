use std::collections::HashMap;

macro_rules! new_macro {
    () => {
        println!("Cool, this is a new, user-defined macro!");
    };
}

macro_rules! macro_that_takes_args {
    (x => $e:expr) => {
        println!("X: {}", $e)
    };
    (y => $e:expr) => {
        println!("Y: {}", $e)
    };
}

macro_rules! macro_that_builds_fn {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

macro_rules! macro_that_prints_an_expression {
    ($e:expr) => {
        println!("{:?} = {:?}", stringify!($e), $e);
    };
}

macro_rules! macro_that_functions_like_match {
    ($l:expr; and $r:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($l),
            stringify!($r),
            $l && $r
        );
    };

    ($l:expr; or $r:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($l),
            stringify!($r),
            $l || $r
        );
    };
}

macro_rules! macro_that_mimics_python_list_comprehension {
    ($id1:ident | $id2: ident <- [$start: expr; $end: expr], $cond: expr) => {{
        let mut vec = Vec::new();

        for num in $start..$end + 1 {
            if $cond(num) {
                vec.push(num);
            }
        }

        vec
    }};
}

fn even(x: i32) -> bool {
    x % 2 == 0
}

fn odd(x: i32) -> bool {
    x % 2 != 0
}

macro_rules! macro_that_creates_new_hashmap {
    // the `*` denotes that there may be infinite expressions
    ($($key: expr => $val: expr)*) => {
        {
            let mut map = HashMap::new();

            $(map.insert($key, $val);)*

            map
        }
    };
}

macro_rules! macro_that_creates_new_hashmap_that_must_take_at_least_one_entry {
    // the `+` denotes that there will be at least one or more expressions
    ($($key: expr => $val: expr,)+) => {
        {
            let mut map = HashMap::new();

            $(map.insert($key, $val);)*

                map
        }
    };
}

macro_rules! macro_that_demonstrates_a_dsl {
    (eval $e:expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    (eval $e:expr, $(eval $es:expr),+) => {
        {
            macro_that_demonstrates_a_dsl! {eval $e}
            macro_that_demonstrates_a_dsl! {$(eval $es),+}
        }
    };
}

fn main() {
    new_macro!();
    macro_that_takes_args!(x => 10);
    macro_that_takes_args!(y => 20 + 30);

    macro_that_builds_fn!(hi_there);
    hi_there();

    macro_that_prints_an_expression!({
        let y = 20;
        let z = 30;
        z + y + 10 * 3 * 100
    });

    macro_that_functions_like_match!(1 == 1; and 2 == 1 + 1);
    macro_that_functions_like_match!(true; or false);

    let evens = macro_that_mimics_python_list_comprehension![x | x <- [1; 10], even];
    println!("{:?}", evens);

    let odds = macro_that_mimics_python_list_comprehension![x | x <- [1; 10], odd];
    println!("{:?}", odds);

    let map = macro_that_creates_new_hashmap!{
        "one" => 1
        "two" => 2
        "three" => 3
    };

    println!("{:?}", map);

    let second_map = macro_that_creates_new_hashmap_that_must_take_at_least_one_entry!{
        "one" => 1,
    };

    println!("{:?}", second_map);

    macro_that_demonstrates_a_dsl! {
        eval 4 * 5,
        eval 4 + 10,
        eval (10 * 3) - 20
    }

    // NOTE: An amazing example of a Lisp-like DSL in Rust:
    //       https://github.com/JunSuzukiJapan/macro-lisp/blob/master/src/lib.rs
}
