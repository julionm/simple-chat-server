use std::io::{BufReader, BufWriter, BufRead, Read, Write};
use std::net::{TcpStream, SocketAddr};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

type ConnMap = HashMap<SocketAddr, TcpStream>;
type ConnPool = Arc<Mutex<ConnMap>>;

pub struct ConnectionsPool {
    pub connections: ConnPool
}

impl ConnectionsPool {
    pub fn new() -> ConnectionsPool {
        ConnectionsPool { connections: Arc::new(Mutex::new(HashMap::new())) }
    }
}

pub fn handle_connection(stream: TcpStream, connections: ConnPool) {

    let ip = stream.peer_addr().unwrap();

    {
        (*connections.lock().unwrap())
            .insert(ip, stream.try_clone().unwrap());
    }

    let mut reader = BufReader::new(&stream);

    macro_rules! receive {
        () => ({
            let mut buf = String::new();
            
            reader.read_line(&mut buf).unwrap();

            buf
        });
    }

    loop {
        let received = receive!();

        if received == String::from("quit") {
            break;
        }

        {
            dispatch_messages(received, &ip, &connections.lock().unwrap());
        }
    }

    // TODO disconnect

}

fn dispatch_messages(msg: String, except: &SocketAddr, lock: &MutexGuard<'_, ConnMap>) {

    for (k, v) in (*lock).iter() {
        // ? send msg to any k through v
        if k != except {
            let mut writer = BufWriter::new(v);
    
            writeln!(writer, "{}", &msg);
            writer.flush();
        }
    }

}

// * handle_connection()
// * - create connection
// * - receive messages
// *   - dispatch_messages()
// * dispatch_messages()
