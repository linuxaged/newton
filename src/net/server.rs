use std::net::UdpSocket;
use std::str::from_utf8;

fn main() {
    let mut socket = (UdpSocket::bind("127.0.0.1:4444")).unwrap();

    let mut buf = [0; 10];
    let (amt, src) = (socket.recv_from(&mut buf)).unwrap();
    if (amt > 0) {
    	println!("{}", from_utf8(&buf).unwrap());
    }
    let buf = &mut buf[..amt];
    buf.reverse();
    (socket.send_to(buf, &src)).unwrap();

    drop(socket); // close the socket
}