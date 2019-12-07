use std::net;
use std::string;
use std::os;
use std::thread;
use std::future;
use std::collections::HashMap;

pub struct Server {
    addr: net::Ipv4Addr,
    sock: net::SocketAddrV4,
    listner: net::TcpListener,
    dict: HashMap<String, String>,
    clients: HashMap<net::SocketAddrV4, net::TcpStream>,
}

pub struct InternalError {

}

impl Server {
    pub fn new(addr: net::Ipv4Addr) -> Server {
        let sock: net::SocketAddrV4 = net::SocketAddrV4::new(self.sock, port: 7777);
        let listener = net::TcpListener::bind(sock).unwrap();
        Server{addr, sock, listner, dict: HashMap::new(), clients: HashMap::new()}
    }

    pub fn run(&mut self) -> ! {
        loop {
            let (stream, addr) = self.listner.accept().unwrap();
            self.clients.insert(addr, stream);
            thread::spawn(|| {

            });
        }
    }

    pub fn set(&self, key: String, value: String) -> InternalError {

    }

    pub fn get(&self, key: String) -> Result<String, InternalError> {

    }
}
