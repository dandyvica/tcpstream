use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::io::{ErrorKind, Read};

fn handle_connection(mut client_stream: TcpStream) {
    // set read timeout for read() operation
    let timeout = Duration::new(5, 0);
    client_stream
        .set_read_timeout(Some(timeout))
        .expect("set_read_timeout call failed");

    loop {
        let mut buffer = [0u8; 1024];

        match client_stream.read(&mut buffer) {
            Ok(0) => {
                println!(
                    "thread_id={:?}, the other end closes the cnx",
                    thread::current().name().unwrap(),
                );
                return;
            }
            Ok(n) => {
                let request =
                    String::from_utf8(buffer[..n].to_vec()).expect("Unable to convert to utf8");
                println!("received: <{}>", request);
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP

                // read timed out: shutdown
                println!("timeout, err={:?}", e);
                client_stream.shutdown(Shutdown::Both).unwrap();
            }
            Err(e) => {
                println!("read error, err={:?}", e);
                client_stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    // count number of threads spawned
    let mut nb_threads = 0;

    // manage connection to our listener
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // get remote port number of the socket
                let remote_port = stream.peer_addr().unwrap().port();
                println!(
                    "======================================> Incoming stream received, port={}",
                    remote_port
                );

                // set read timeout for read() operation
                let timeout = Duration::new(5, 0);
                stream
                    .set_read_timeout(Some(timeout))
                    .expect("set_read_timeout call failed");

                // start new thread for managing connexion
                match thread::Builder::new()
                    .name(remote_port.to_string())
                    .spawn(move || {
                        handle_connection(stream);
                    }) {
                    Ok(_) => {
                        nb_threads += 1;
                        println!("nb threads so far = {}", nb_threads);
                    },
                    Err(err) => panic!(
                        "Could not create a new thread for handling request from remote port {}, err={:?}",
                        remote_port,
                        err
                    ),
                }
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
