// 1.Server -> Start (update waiting for data)
// 2.Client -> Connect to Server
// 3.Server -> Send snapshot to all clients
// snapshot 的计算和渲染：http://gamedev.stackexchange.com/questions/87553/how-would-a-game-state-snapshot-system-be-implemented-for-networked-real-time-ga
// Quake3 相关 snapshot 参考：https://github.com/id-Software/Quake-III-Arena/blob/dbe4ddb10315479fc00086f08e25d968b4b43c49/code/server/sv_snapshot.c
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

