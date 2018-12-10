use std::io::{ErrorKind, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::net::{TcpListener};
use std::sync::mpsc;
use std::thread;

const MSG_SIZE: usize = 32;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100))
}

fn main() {
    let local_socket: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6000);
    let mut clients = vec![];
    let (tx, rx) = mpsc::channel();

    // Just wanting to test out a bit of net::TcpStream...
    // if let Ok(stream) = TcpStream::connect(local_socket) {
    //     println!("Connected to the server!");
    // } else {
    //     println!("Couldn't connect to server...");
    // }

    let server = TcpListener::bind(local_socket).expect("Listener failed to bind!");

    // Move TCP stream into non-blocking mode,
    // equivalent to calling `fcntl FIONBIO`.
    // A socket is in blocking mode when an I/O
    // call waits for an event to complete,
    // whereas a socket in non-blocking mode
    // the program continues even though the
    // I/O call may not have completed.
    // Any error that arises in non-blocking mode
    // will result in an io::ErrorKind::WouldBlock
    server
        .set_nonblocking(true)
        .expect("Cannot set non-blocking!");

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client connected to: {}", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client!"));

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message!");

                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("Failed to send msg to rx!");
                    }
                    // Return unit value....
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing connection with: {}", addr);
                        break;
                    }
                }

                sleep();
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MSG_SIZE, 0);

                    client.write_all(&buff).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }

        sleep();
    }
}
