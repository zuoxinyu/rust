use std::net;
use std::io::Read;
use std::io::Write;

fn main() {
    let host = net::SocketAddrV4::new(net::Ipv4Addr::LOCALHOST, 8080);
    println!("Listening on host: {}", host);
    let listener = net::TcpListener::bind(host)
        .expect("can't bind");
    
    loop {
        let mut buf : [u8;1024] = [0;1024];
        let (mut stream, remote_addr) = listener.accept().unwrap();
        println!("The remote address: {}", remote_addr);
        let size = stream.read(&mut buf).unwrap();
        println!("Recieved len: {}\n\n{}", size, std::str::from_utf8(&buf).unwrap());
        let resp = "HTTP/1.1 200 OK\r\n\r\nresp";
        let send_size = stream.write(resp.as_bytes()).unwrap();
        println!("Sent size: {}", send_size);
        stream.shutdown(net::Shutdown::Both).unwrap();
    }
}
