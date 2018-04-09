fn to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + 32.0
}

fn to_celsius(f: f64) -> f64 {
    f - 32.0 * (5.0 / 9.0)
}

fn fib(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    }
    match n {
        0 => panic!("Zero is not a valid argument to fib()!"),
        1 | 2 => 1,
        3 => 2,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn fib_non_recursive(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    } else if n == 0 {
        panic!("Zero is not a valid argument to fib()!");
    } else if n == 1 {
        return 1;
    }

    let mut i = 0;
    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;

    while i < n - 1 {
        sum = last + curr;
        last = curr;
        curr = sum;
        i += 1;
    }

    sum
}

fn cant_stand_christmas_tunes() {
    let mut i = 12;

    while 0 < i {
        println!("On the {} day of Christmas,", i);
        i -= 1;
    }

    "On the {} day of Christmas";
    "my true love sent to me:";
    "A Partridge in a Pear Tree";

    "Two turtle doves";
    "A Partridge in a Pear Tree";
}

fn main() {
    println!("{} fahrenheit to celsius: {}", 70.0, to_celsius(70.0));
    println!("{} celsius to fahrenheit: {}", 20.0, to_fahrenheit(20.0));
    println!("The {}th Fibonacci is {}", 40, fib(40));
    println!("The {}th Fibonacci is {}", 92, fib_non_recursive(92));
    cant_stand_christmas_tunes();
}
