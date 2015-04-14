use std::net::{SocketAddrV4, Ipv4Addr};
use std::net::UdpSocket;
fn main() {
    let mut socket = (UdpSocket::bind("127.0.0.1:4445")).unwrap();

    let mut buf = [0; 10];
    let (amt, src) = (socket.recv_from(&mut buf)).unwrap();

    // Send a reply to the socket we received data from
    let buf = &mut buf[..amt];
    buf.reverse();
    let serverIp = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 4444);
    (socket.send_to(buf, &serverIp)).unwrap();

    drop(socket); // close the socket
}