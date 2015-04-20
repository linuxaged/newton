#![feature(collections)]
mod socket {

use std::default::Default;
use std::net::UdpSocket;
use std::collections::LinkedList;
use std::vec;
use std::vec::Vec;
use std::ptr;

struct PacketData {
    sequence: u32,          // packet sequence number
    time:     f32,          // time offset since packet was sent or received (depending on context)
    size:     u32,          // packet size in bytes
}

impl Default for PacketData {
    fn default() -> PacketData {
        PacketData{sequence: 0, time: 0.0f32, size: 0}
    }
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

#[deriving(Default)]
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

impl ReliabilitySystem {
    pub fn new(&self) -> ReliabilitySystem {

    }

    pub fn reset(&self)
    {
        self.local_sequence = 0;
        self.remote_sequence = 0;
        self.sentQueue.clear();
        self.receivedQueue.clear();
        self.pendingAckQueue.clear();
        self.ackedQueue.clear();
        self.sent_packets = 0;
        self.recv_packets = 0;
        self.lost_packets = 0;
        self.acked_packets = 0;
        self.sent_bandwidth = 0.0f32;
        self.acked_bandwidth = 0.0f32;
        self.rtt = 0.0f32;
        self.rtt_maximum = 1.0f32;
    }

    fn PacketSent(&self, size: u32 )
    {
        if ( self.sentQueue.exists( self.local_sequence ) )
        {
            println!( "local sequence {} exists", self.local_sequence );
            for itor in self.sentQueue.iter() {
                println!(itor.sequence)
            }
        }
        assert!( !sentQueue.exists( local_sequence ) );
        assert!( !pendingAckQueue.exists( local_sequence ) );
        let data = PacketData::default();
        data.sequence = self.local_sequence;
        data.time = 0.0f32;
        data.size = size;
        sentQueue.push_back( data );
        pendingAckQueue.push_back( data );
        sent_packets = sent_packets + 1;
        local_sequence = sent_packets + 1;
        if ( local_sequence > max_sequence ) {
            local_sequence = 0;
        }
    }

    fn PacketReceived(&self, sequence: u32, size: u32 )
    {
        recv_packets = recv_packets + 1;
        if ( receivedQueue.exists( sequence ) ) {
            return;
        }
        let data = PacketData::default();
        data.sequence = sequence;
        data.time = 0.0f32;
        data.size = size;
        receivedQueue.push_back( data );
        if ( sequence_more_recent( sequence, remote_sequence, max_sequence ) ){
            remote_sequence = sequence;
        }
    }

    fn GenerateAckBits() -> u32
    {
        return generate_ack_bits( GetRemoteSequence(), receivedQueue, max_sequence );
    }

    fn ProcessAck( ack: u32, ack_bits: u32 )
    {
        process_ack( ack, ack_bits, pendingAckQueue, ackedQueue, acks, acked_packets, rtt, max_sequence );
    }

    fn Update( deltaTime: f32 )
    {
        acks.clear();
        AdvanceQueueTime( deltaTime );
        UpdateQueues();
        UpdateStats();

        // Validate();

    }

    // fn Validate()
    // {
    //     sentQueue.verify_sorted( max_sequence );
    //     receivedQueue.verify_sorted( max_sequence );
    //     pendingAckQueue.verify_sorted( max_sequence );
    //     ackedQueue.verify_sorted( max_sequence );
    // }

    // utility functions

    fn sequence_more_recent(  s1: u32,  s2: u32,  max_sequence: u32 ) -> bool
    {
        return (( s1 > s2 ) && ( s1 - s2 <= max_sequence / 2 )) || (( s2 > s1 ) && ( s2 - s1 > max_sequence / 2 ));
    }

    fn bit_index_for_sequence( sequence: u32,  ack: u32, max_sequence: u32 ) -> i32
    {
        assert!( sequence != ack );
        assert!( !sequence_more_recent( sequence, ack, max_sequence ) );
        if ( sequence > ack )
        {
            assert!( ack < 33 );
            assert!( max_sequence >= sequence );
            return ack + ( max_sequence - sequence );
        }
        else
        {
            assert!( ack >= 1 );
            assert!( sequence <= ack - 1 );
            return ack - 1 - sequence;
        }
    }

    fn generate_ack_bits(  ack: u32, received_queue: &PacketQueue , max_sequence: u32) -> u32
    {
        let ack_bits = 0u32;
        for itor in received_queue.itor() {
            if ( itor.sequence == ack || sequence_more_recent( itor.sequence, ack, max_sequence ) ){
                break;
            }
            let bit_index = bit_index_for_sequence( itor.sequence, ack, max_sequence );
            if ( bit_index <= 31 ) {
                ack_bits |= 1 << bit_index;
            }
        }

        return ack_bits;
    }

    fn process_ack(&self, ack: u32,  ack_bits: u32,
                             pending_ack_queue: &PacketQueue, acked_queue: &PacketQueue,
                             acks: &Vec<u32>, acked_packets: u32,
                             rtt: &f32, max_sequence: u32 )
    {
        if ( pending_ack_queue.empty() ) {
            return;
        }

        for itor in self.pending_ack_queue.itor()
        {
            let acked = false;

            if ( itor.sequence == ack )
            {
                acked = true;
            }
            else if ( !sequence_more_recent( itor.sequence, ack, max_sequence ) )
            {
                let bit_index = bit_index_for_sequence( itor.sequence, ack, max_sequence );
                if ( bit_index <= 31 ) {
                    acked = ( ack_bits >> bit_index ) & 1;
                }
            }

            if ( acked )
            {
                rtt += ( itor.time - rtt ) * 0.1f32;

                acked_queue.insert_sorted( *itor, max_sequence );
                acks.push_back( itor.sequence );
                acked_packets = acked_packets + 1;
                itor = pending_ack_queue.erase( itor );
            }
        }
    }

    // data accessors

    fn GetLocalSequence(&self) -> u32
    {
        self.local_sequence
    }

    fn GetRemoteSequence(&self) -> u32
    {
        self.remote_sequence
    }

    fn GetMaxSequence(&self) -> u32
    {
        self.max_sequence
    }

    fn GetSentPackets() -> u32
    {
        self.sent_packets
    }

    fn GetReceivedPackets() -> u32
    {
        self.recv_packets
    }

    fn GetLostPackets() -> u32
    {
        self.lost_packets
    }

    fn GetAckedPackets() -> u32
    {
        self.acked_packets
    }

    fn GetSentBandwidth() -> f32
    {
        self.sent_bandwidth
    }

    fn GetAckedBandwidth(&self) -> f32
    {
        self.acked_bandwidth
    }

    fn GetRoundTripTime(&self) -> f32
    {
        self.rtt
    }

    fn GetHeaderSize() -> i32
    {
        12
    }

    fn AdvanceQueueTime(&self, deltaTime: f32 )
    {
        for itor in self.sentQueue.itor() {
            itor.time += deltaTime;
        }

        for itor in self.receivedQueue.itor() {
            itor.time += deltaTime;
        }

        for itor in self.pendingAckQueue.itor() {
            itor.time += deltaTime;
        }

        for itor in self.ackedQueue.itor() {
            itor.time += deltaTime;
        }
    }

    fn UpdateQueues()
    {
        let epsilon = 0.001f32;

        while ( sentQueue.size() && sentQueue.front().unwrap().time > rtt_maximum + epsilon ) {
            sentQueue.pop_front();
        }

        if ( receivedQueue.size() )
        {
            let latest_sequence = receivedQueue.back().unwrap().sequence;
            let minimum_sequence = if latest_sequence >= 34 { latest_sequence - 34 } else { max_sequence - ( 34 - latest_sequence) };
            // let minimum_sequence = latest_sequence >= 34 ? ( latest_sequence - 34 ) : max_sequence - ( 34 - latest_sequence );
            while ( receivedQueue.size() && !sequence_more_recent( receivedQueue.front().unwrap().sequence, minimum_sequence, max_sequence ) ) {
                receivedQueue.pop_front();
            }
        }

        while ( ackedQueue.size() && ackedQueue.front().unwrap().time > rtt_maximum * 2 - epsilon ) {
            ackedQueue.pop_front();
        }

        while ( pendingAckQueue.size() && pendingAckQueue.front().unwrap().time > rtt_maximum + epsilon )
        {
            pendingAckQueue.pop_front();
            lost_packets += 1;
        }
    }

    fn UpdateStats()
    {
        let sent_bytes_per_second = 0;
        for itor in sentQueue.itor() {
            sent_bytes_per_second += itor.size;
        }
        let acked_packets_per_second = 0;
        let acked_bytes_per_second = 0;
        for itor in ackedQueue.itor()
        {
            if ( itor.time >= rtt_maximum )
            {
                acked_packets_per_second += 1;
                acked_bytes_per_second += itor.size;
            }
        }
        sent_bytes_per_second /= rtt_maximum;
        acked_bytes_per_second /= rtt_maximum;
        sent_bandwidth = sent_bytes_per_second * ( 8 / 1000.0f32 );
        acked_bandwidth = acked_bytes_per_second * ( 8 / 1000.0f32 );
    }
}

enum State {
    Disconnected,
    Listening,
    Connecting,
    ConnectFail,
    Connected,
}

enum Mode {
    Null,
    Client,
    Server,
}

struct ReliableConnection {
    address:            SocketAddrV4,
    socket:             UdpSocket,

    protocolId:         u32,
    state:              State,
    mode:               Mode,
    running:            bool,
    timeout:            f32,
    timeoutAccumulator: f32,

    reliabilitySystem:  ReliabilitySystem::default(),
}

impl Default for ReliableConnection {
    fn default () -> ReliableConnection {

        ReliableConnection {
            address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 1234),
            socket: try!(UdpSocket::bind(&address)),
            protocolId: 0,
            state: State.Disconnected,
            mode: Null,
            running: false,
            timeout: 0.0f32,
            timeoutAccumulator: 0.0f32,

            reliabilitySystem: ReliabilitySystem
        }

    }
}

impl ReliableConnection {
    fn new(&self,pId: u32, TO: f32) -> ReliableConnection {
        ReliableConnection {
            protocolId: pId,
            timeout: TO,
            ..Default::default()
        }

    }
    fn start(&self, addr: SocketAddrV4)
    {
        assert!( !self.running );
        println!( "start connection on port {}", port );
        // if ( !socket.bind( addr ) )
        //     return false;
        self.running = true;
        on_start();
        return true;
    }

    fn stop(&self)
    {
        assert!( running );
        println!("stop connection");
        self.connected = is_connected();
        clear_data();
        drop(socket);
        self.running = false;
        if ( self.connected ) {
            on_disconnect();
        }
        on_stop();
    }

    fn is_running(&self) -> bool
    {
        self.running
    }

    fn listen(&self)
    {
        printf( "server listening for connection\n" );
        self.connected = is_connected();
        clear_data();
        if ( connected ) {
            OnDisconnect();
        }
        mode = Server;
        state = Listening;
    }

    fn connect(&self, addr: SocketAddrV4)
    {
        printf( "client connecting to {}", addr);
        self.connected = is_connected();
        clear_data();
        if ( self.connected ) {
            on_disconnect();
        }
        self.mode = Mode.Client;
        self.state = State.Connecting;
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

    pub fn update(&self, deltaTime: f32)
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
                if ( self.state == State.Connecting ) {
                    self.state = State.ConnectFail;
                }
                on_disconnect();
            }
        }
    }
    // 添加4字节的 协议ID 后发送
    pub fn send_packet(&self, data: &[u8], size: u32) -> bool
    {
        assert!( running );
        if ( address.GetAddress() == 0 ) {
            return false;
        }
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

    pub fn receive_packet(&self, data: &[u8],  size: u32) -> i32
    {
        assert!(self.running);
        // uchar_t packet[size + 4];
        let packet = [u8, size + 4];
        let (bytes_read, sender) = socket.recv_from(&packet);
        if ( bytes_read == 0 ) {
            return 0;
        }
        if ( bytes_read <= 4 ) {
            return 0;
        }
        if ( packet[0] != ( protocolId >> 24 ) as u8 ||
                packet[1] != ( ( protocolId >> 16 ) & 0xFF ) as u8 ||
                packet[2] != ( ( protocolId >> 8 ) & 0xFF ) as u8||
                packet[3] != ( protocolId & 0xFF ) as u8) {
            return 0;
        }

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
            self.timeoutAccumulator = 0.0f32;

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
    fn on_disconnect(&self) {
        self.clear_data();
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