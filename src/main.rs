use std::io::{Read,Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut data = [0 as u8; 1024];
        match stream.read(&mut data) {
            Ok(size) => {
                stream.write(&data[0..size]).unwrap();
                stream.flush().unwrap();
            },
            Err(_) => {
                println!("An error occurred, connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333!");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection with {}", stream.peer_addr().unwrap());
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Error {e}");
                /* connection failed */
            }
        }
    }

    Ok(())
}
