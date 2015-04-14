use std::net::{SocketAddrV4, Ipv4Addr};
use std::net::UdpSocket;
fn main() {
    let mut socket = (UdpSocket::bind("127.0.0.1:4445")).unwrap();

    let serverIp = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 4444);
    (socket.send_to(b"tracytracy", &serverIp)).unwrap();

    drop(socket); // close the socket
}