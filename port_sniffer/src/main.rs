use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

const CLI_HELP_TEXT: &str = ("Usage:\n \
                              \t-t or --threads to select how many threads you want\n \
                              \t-h or --help to show this help message");
const TOO_FEW_ARGS: &str = "Not enough arguments provided to program";
const TOO_MANY_ARGS: &str = "Too many arguments provided to program";
const MAX_PORT: u16 = 65535;

#[derive(Debug)]
struct Arguments {
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        // Handle basic sources of errors
        if args.len() < 2 {
            return Err(TOO_FEW_ARGS);
        } else if args.len() > 4 {
            return Err(TOO_MANY_ARGS);
        }

        let flag = args[1].clone();

        // Perform basic check on incoming IP (using `from_str` here because
        // both Ipv4Addr and Ipv6Addr implement FromStr trait) - if no other
        // flags and no errors, we pass the IpAddr instance back to `Ok()`,
        // else we continue to process
        if let Ok(ipaddr) = IpAddr::from_str(&flag) {
            return Ok(Arguments { ipaddr, threads: 4 });
        } else {
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!("{}", CLI_HELP_TEXT);
                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                // We're here because the args count was greater than 2,
                // but we still had a request for help - bail out
                return Err(TOO_MANY_ARGS);
            } else if flag.contains("-t") || flag.contains("--threads") {
                // Assign `ipaddr`, returning its value using `from_str` method
                // on Ipv4Addr/Ipv6Addr - we could have an a bad IP, though, so
                // handle that, as well
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IPADDR; must be IPv4 or IPv6"),
                };
                // Just a simple cast to an unsigned 16-bit int, but handle error
                // if we get junk data
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number"),
                };
                return Ok(Arguments { threads, ipaddr });
            } else {
                return Err("Invalid syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, ipaddr: IpAddr, threads: u16) {
    // We could pass in a `start_port` but port 0 is reserved anyway....
    // Instead, we let 1 be the starting port and increment from there.
    let mut port: u16 = 1;
    loop {
        match TcpStream::connect((ipaddr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX_PORT - port) <= threads {
            break;
        }

        port += threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // If we're here - we got an instance of our `Arguments` struct,
    // we'll destructure into `threads` and `ipaddr`
    let Arguments { threads, ipaddr } = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            // Bubble up the error from the impl of `Arguments::new()`
            eprintln!("{} problem parsing arguments: {}", program, err);
            process::exit(0)
        }
    });

    // Create a new async channel, returning its two halves:
    // Sender (tx for transmission) and Receiver (rx for receiving)
    // Data sent on the Sender(tx) is available on the Receiver(rx)
    let (tx, rx) = channel();

    for _ in 1..threads {
        // Sending half (tx) can only be owned by one thread,
        // we clone the Sender in order to send to other threads
        let tx = tx.clone();

        // Spawn off expensive computation to threads...
        thread::spawn(move || {
            scan(tx, ipaddr, threads);
        });
    }

    let mut out = vec![];

    // Explicitly tell the Receiver (rx) the Sender (tx) is out of scope
    drop(tx);

    // Push port to output vector
    for port in rx {
        out.push(port);
    }

    println!("");

    out.sort();

    // Iterate over the port vector and display value
    for port in out {
        println!("{} is open!", port);
    }
}
