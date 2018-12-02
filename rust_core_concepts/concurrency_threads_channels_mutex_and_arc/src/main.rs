use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

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

    // The `move` keyword forces the closure to reference
    // data by value rather than by reference.
    // Instead of borrowing from vec_for_iteration
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

    // NOTE: `mpsc` =  multiple producer single consumer

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    // NOTE: `recv` is a blocking method, preventing the main thread
    //       from processing any additional computation until the
    //       channel is resolved - `try_recv`
    println!("got {}", rx.recv().unwrap());

    // In the following, we use channels to send data between threads.
    // We have also written an example of the "multiple producer,
    // single consumer" pattern. Specifically, we have 24 cloned
    // transmitters and a single receiver
    const NUM_TIMERS: usize = 24;

    fn timer(d: usize, tx: mpsc::Sender<usize>) {
        thread::spawn(move || {
            println!("{}: setting timer...", d);
            thread::sleep(Duration::from_secs(d as u64));
            println!("{}: sent!", d);
            tx.send(d).unwrap()
        });
    }

    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_TIMERS {
        timer(i, tx.clone());
    }

    for v in rx.iter().take(NUM_TIMERS) {
        println!("{}: received!", v);
    }

    // In addition to channels, Rust also uses the
    // Mutex (mutual exclusion) abstraction to
    // communicate shared memory inside of its
    // concurrency model.
    //
    // A Mutex exists to ensure that a piece of data
    // is only accessible by one thread at a time.
    //
    // A Mutex has two primary rules:
    // (1) the thread that wants access the data must
    //     acquire the lock of the mutex
    // (2) once the data is released, you must unlock the data
    //     so other threads can gain access
    //
    // A mutex functions like a storage locker -
    // one key, multiple persons with access - each person must
    // have access to the key, however, and if a request for access
    // is granted, the key must be released from the last person
    // with access.
    //
    // It acts like a smart pointer (i.e., like Box) - specifically,
    // a mutex lock method returns a smart pointer called a MutexGuard.
    // This MutexGuardis an RAII implementation of a "scoped lock"
    // of a mutex, when this data structure is dropped from memory
    // the lock is removed.

    // NOTE: `Arc` = Atomically Referenced Counted type
    // (i.e., acts like a primitive, but is safe to share across threads)
    let c = Arc::new(Mutex::new(0));
    let mut hs = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&c);
        let h = thread::spawn(move || {
            let mut num = c.lock().unwrap();

            *num += 1;
        });
        hs.push(h);
    }

    for h in hs {
        h.join().unwrap();
    }

    println!("Result: {}", *c.lock().unwrap());

    // In summary:
    // Each thread is spawned, when it is spawned it gains
    // access to the mutex, increments mutex by one, then releases its
    // access, then the next thread gains it access and releases, etc.,
    // After all threads resolve with join, our main thread gets access
    // to the mutex and prints it out.

    // Example of using channels and mutexes together to find primes:

    fn is_prime(n: usize) -> bool {
        (2..n).all(|i| n % i != 0)
    }

    fn producer(tx: mpsc::SyncSender<usize>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            for i in 100_000_000.. {
                tx.send(i).unwrap();
            }
        })
    }

    fn worker(id: u64, shared_rx: Arc<Mutex<mpsc::Receiver<usize>>>) {
        thread::spawn(move || loop {
            {
                let mut n = 0;
                match shared_rx.lock() {
                    Ok(rx) => match rx.try_recv() {
                        Ok(_n) => {
                            n = _n;
                        }
                        Err(_) => (),
                    },
                    Err(_) => (),
                }

                if n != 0 {
                    if is_prime(n) {
                        println!("worker {} found a prime: {}", id, n);
                    }
                }
            }
        });
    }

    let (tx, rx) = mpsc::sync_channel(1024); // memory allocation
    let shared_rx = Arc::new(Mutex::new(rx));

    for i in 1..5 {
        worker(i, shared_rx.clone());
    }

    producer(tx).join().unwrap();
}
