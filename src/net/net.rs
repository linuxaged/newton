// todo: rudp library

use std::net::UdpSocket;

fn main() {
    let mut socket = (UdpSocket::bind("127.0.0.1:34254")).unwrap();

    let mut buf = [0; 10];
    let (amt, src) = (socket.recv_from(&mut buf)).unwrap();

    // Send a reply to the socket we received data from
    let buf = &mut buf[..amt];
    buf.reverse();
    (socket.send_to(buf, &src)).unwrap();

    drop(socket); // close the socket
}

