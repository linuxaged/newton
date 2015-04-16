#![feature(collections)]
mod socket {

use std::net::UdpSocket;
use std::collections::LinkedList;
use std::vec;
use std::ptr;

struct PacketData {
    sequence: u32,          // packet sequence number
    time:     f32,          // time offset since packet was sent or received (depending on context)
    size:     u32,          // packet size in bytes
}

trait PacketQueue {
    fn exists(&self, mut sequence: u32) -> bool;
    fn insert_sorted(&mut self, p: PacketData,  max_sequence: u32);
}

#[inline]
fn sequence_more_recent(s1: u32, s2: u32, max_sequence: u32) -> bool {
    (( s1 > s2 ) && ( s1 - s2 <= max_sequence / 2 )) || (( s2 > s1 ) && ( s2 - s1 > max_sequence / 2 ))
}

impl PacketQueue for LinkedList<PacketData> {
    fn exists(&self, mut sequence: u32) -> bool {
        for iter in self.iter() {
            if iter.sequence == sequence {
                return true;
            }
        }
        return false;
    }

    fn insert_sorted(&mut self, p: PacketData, max_sequence: u32) {
        if self.is_empty() {
            self.push_back(p);
        }
        else {
            if ( !sequence_more_recent( p.sequence, self.front().unwrap().sequence, max_sequence ) ) {
                self.push_front( p );
            }
            else if ( sequence_more_recent( p.sequence, self.back().unwrap().sequence, max_sequence ) ) {
                self.push_back(p );
            }
            else {
                let mut itor = self.iter_mut();
                loop {
                    if ( sequence_more_recent( itor.next().unwrap().sequence, p.sequence, max_sequence ) ) {
                        itor.insert_next(p);
                        break;
                    }
                }
            }
        }
    }
}

struct ReliabilitySystem {
    max_sequence:    u32,                    // maximum sequence value before wrap around (used to test sequence wrap at low # values)
    local_sequence:  u32,                    // local sequence number for most recently sent packet
    remote_sequence: u32,                    // remote sequence number for most recently received packet

    sent_packets:    u32,                    // total number of packets sent
    recv_packets:    u32,                    // total number of packets received
    lost_packets:    u32,                    // total number of packets lost
    acked_packets:   u32,                    // total number of packets acked

    sent_bandwidth:  f32,                    // approximate sent bandwidth over the last second
    acked_bandwidth: f32,                    // approximate acked bandwidth over the last second
    rtt            : f32,                    // estimated round trip time
    rtt_maximum:     f32,                    // maximum expected round trip time (hard coded to one second for the moment)

    acks:            Vec<u32>,               // acked packets from last set of packet receives. cleared each update!

    sentQueue:       LinkedList<PacketData>, // sent packets used to calculate sent bandwidth (kept until rtt_maximum)
    pendingAckQueue: LinkedList<PacketData>, // sent packets which have not been acked yet (kept until rtt_maximum * 2 )
    receivedQueue:   LinkedList<PacketData>, // received packets for determining acks to send (kept up to most recent recv sequence - 32)
    ackedQueue:      LinkedList<PacketData>, // acked packets (kept until rtt_maximum * 2)
}

enum State {
    Disconnected,
    Listening,
    Connecting,
    ConnectFail,
    Connected,
}

enum Mode {
    Client,
    Server,
}
struct ReliableConnection {
    address: SocketAddrV4,
    socket: UdpSocket,

    protocolId: u32,
    state: State,
    mode: Mode,
    running: bool,
    timeout: f32,
    timeoutAccumulator: f32,

    reliabilitySystem: ReliabilitySystem,
}

impl ReliableConnection {
    fn new(&self) -> ReliableConnection {

    }
    fn start(&self, port: u16)
    {
        assert!( !running );
        println!( "start connection on port {}", port );
        if ( !socket.Open( port ) )
            return false;
        running = true;
        OnStart();
        return true;
    }

    fn stop(&self)
    {
        assert( running );
        println!("stop connection");
        bool connected = IsConnected();
        ClearData();
        socket.Close();
        running = false;
        if ( connected )
            OnDisconnect();
        OnStop();
    }

    fn is_running(&self) -> bool
    {
        self.running
    }

    fn listen(&self)
    {
        printf( "server listening for connection\n" );
        bool connected = IsConnected();
        ClearData();
        if ( connected )
            OnDisconnect();
        mode = Server;
        state = Listening;
    }

    fn connect(&self, addr: SocketAddrV4)
    {
        printf( "client connecting to %hhu.%hhu.%hhu.%hhu:%d\n",
                addr.GetA(), addr.GetB(), addr.GetC(), addr.GetD(), addr.GetPort() );
        bool connected = IsConnected();
        ClearData();
        if ( connected )
            OnDisconnect();
        mode = Client;
        state = Connecting;
        self.address = addr;
    }

    fn is_connecting(&self) -> bool
    {
        self.state == State.Connecting
    }

    fn connect_failed(&self) -> bool
    {
        self.state == State.ConnectFail
    }

    fn is_connected(&self) -> bool
    {
        self.state == State.Connected
    }

    fn is_listening(&self) -> bool
    {
        self.state == State.Listening
    }

    fn get_mode(&self) -> Mode
    {
        self.mode
    }

    fn update(&self, deltaTime: f32)
    {
        assert!( self.running );
        self.timeoutAccumulator += deltaTime;
        if ( self.timeoutAccumulator > timeout )
        {
            if ( self.state == State.Connecting )
            {
                println!( "connect timed out\n" );
                clear_data();
                self.state = State.ConnectFail;
                on_disconnect();
            }
            else if ( self.state == State.Connected )
            {
                println!( "connection timed out\n" );
                clear_data();
                if ( self.state == State.Connecting )
                    self.state = State.ConnectFail;
                on_disconnect();
            }
        }
    }
    // 添加4字节的 协议ID 后发送
    fn send_packet(&self, data: []u8, size: u32) -> bool
    {
        assert!( running );
        if ( address.GetAddress() == 0 )
            return false;
        // uchar_t packet[size + 4];
        let packet = [u8, size + 4];
        packet[0] = ( protocolId >> 24 ) as u8 ;
        packet[1] = ( ( protocolId >> 16 ) & 0xFF ) as u8;
        packet[2] = ( ( protocolId >> 8 ) & 0xFF ) as u8;
        packet[3] = ( ( protocolId ) & 0xFF ) as u8;

        // memcpy( &packet[4], data, size );
        ptr::copy_nonoverlapping(packet, data, size);
        return (socket.send_to(packet, &address)).unwrap();
    }

    fn receive_packet(&self, data: []u8,  size: u32) -> i32
    {
        assert!( running );
        // uchar_t packet[size + 4];
        let packet = [u8, size + 4];
        SocketAddrV4 sender;
        let (bytes_read, sender) = socket.recv_from(&packet);
        if ( bytes_read == 0 )
            return 0;
        if ( bytes_read <= 4 )
            return 0;
        if ( packet[0] != ( protocolId >> 24 ) as u8 ||
                packet[1] != ( ( protocolId >> 16 ) & 0xFF ) as u8 ||
                packet[2] != ( ( protocolId >> 8 ) & 0xFF ) as u8||
                packet[3] != ( protocolId & 0xFF ) as u8)
            return 0;
        if ( self.mode == Mode.Server && !is_connected )
        {
            println!( "server accepts connection from client {}", sender);
            self.state = State.Connected;
            self.address = sender;
            on_connect();
        }
        if ( sender == address )
        {
            if ( mode == Client && state == Connecting )
            {
                println!("client completes connection with server");
                self.state = Connected;
                on_connect();
            }
            self.timeoutAccumulator = 0.0f;

            //memcpy( data, &packet[4], bytes_read - 4 );
            ptr::copy_nonoverlapping(packet, data, bytes_read - 4);
        }
        return 0;
    }

    fn get_header_size() -> i32 {
        return 4;
    }

    fn on_start() {

    }
    fn on_stop() {

    }
    fn on_connect() {

    }
    fn on_disconnect() {

    }

    fn clear_data(&self) {
        self.state = State.Disconnected;
        self.timeoutAccumulator = 0.0f32;
        self.address = Address();
    }
}

#[test]
fn test_linked_list() {
    let mut ll: LinkedList<PacketData> = LinkedList::new();
    let pd0 = PacketData{sequence: 0, time: 0.0f32, size: 128u32};
    ll.insert_sorted(pd0, 128u32);
}

} // end of mod socket