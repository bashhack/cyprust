use std::sync::mpsc;
use std::thread;

// ----------------------------------------------------------------------------
//
// ----------------------------------------------------------------------------

fn main() {
    // ------------------------------------------------------------------------
    //
    // ------------------------------------------------------------------------

    // Concurrent programming relies on a foundation: the thread.
    //
    // It is the smallest sequence of programmed instructions
    // that can be managed independently by a scheduler.
    //
    // Threads differentiate themselves from processes in that:
    // threads are subsets of a process, threads carry less
    // information about state, and threads share their space in memory.
    //
    // For the sake of this overview, we really care about
    // two types of threads: (1) OS threads, and (2) green threads.
    // In many programming languages (Haskell, Erlang, CPython, etc.),
    // we deal with green threads - an abstraction over native threads.
    // These green threads will emulate multithreaded environments
    // without any native OS dependency, managed in userland instead
    // of priviledged kernel space.
    //
    // Rust eschews green threads - though we can pull in crates that
    // give us access to these M:N (M green threads per N operating
    // system threads) - instead giving us OS (or 1:1) threading.
    // This allows Rust to have a lower runtime - a lower amount of code
    // included in each binary after compilation.
    //
    // Ultimately, concurrency allows us to split computation across
    // threads to the end of improving performance because the program
    // can now execute multiple tasks at the same time. The trade-off
    // is that we introduce complexity in writing our code (and in the
    // interpretation of it by our peers).

    let mut just_a_vector_for_iteration = vec![];

    for i in 0..10 {
        // spawn will return a 'std::thread::JoinHandle',
        // an owned permission to join on a thread (block
        // on its termination) - it detaches the associated
        // thread when it is dropped
        just_a_vector_for_iteration.push(thread::spawn(move || {
            println!("thread number {}", i);
        }));
    }

    // for j in 0..10 {
    //     println!("main thread");
    // }

    for j in just_a_vector_for_iteration {
        // iterate through our vector of threads,
        // forcing the main thread to wait for the thread
        // that's attached to the JoinHandle to finish
        j.join();
    }

    let vec_for_iteration = vec![1, 2, 3];

    // the `move` keyword forces the closure to reference
    // data by value rather than by reference
    // instead of borrowing from vec_for_iteration
    // we force the closure to take from vec_for_iteration
    // and put it inside of the thread, giving it
    // complete ownership. We also guarantee that the main
    // thread won't interact with that moved value - attempting
    // to access it in the main scope would throw an error.
    let handle = thread::spawn(move || {
        println!("vector: {:?}", vec_for_iteration);
    });

    handle.join();

    // Another main abstraction for concurrent programming in Rust
    // are channels.
    // They allow us to pass messages, made up of a transmitter
    // and a receiver. The transmitter sits upstream, where we push
    // the message in, and the receiver is where the message comes out.

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    // NOTE: `recv` is a blocking method, preventing the main thread
    //       from processing any additional computation until the
    //       channel is resolved
    println!("got {}", rx.recv().unwrap());
}
