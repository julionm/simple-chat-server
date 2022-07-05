use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use server::{ConnectionsPool, handle_connection};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
    let mut connections_pool = ConnectionsPool::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let connections = connections_pool.connections.clone();

        spawn(move || {
            handle_connection(stream, connections);
        });
    }
}
