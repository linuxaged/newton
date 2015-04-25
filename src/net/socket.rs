// todo:
// use vector or deque instead of linkedlist

#![feature(collections)]
mod socket {

use std::default::Default;
use std::net::{UdpSocket, SocketAddr, SocketAddrV4, Ipv4Addr};
use std::collections::VecDeque;
use std::vec;
use std::vec::Vec;
use std::ptr;

#[derive(Clone, Copy)]
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

fn bit_index_for_sequence( sequence: u32,  ack: u32, max_sequence: u32 ) -> u32
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

impl PacketQueue for VecDeque<PacketData> {
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
            if ( sequence_more_recent( p.sequence, self.back().unwrap().sequence, max_sequence ) ) {
                self.push_back(p );
            } else if ( !sequence_more_recent( p.sequence, self.front().unwrap().sequence, max_sequence )) {
                self.push_front( p );
            }
            else {
                for i in 0..self.len() {
                    if ( sequence_more_recent( self.get(i).unwrap().sequence, p.sequence, max_sequence ) ) {
                        self.insert(i + 1, p);
                        break;
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct ReliabilitySystem {
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

    sentQueue:       VecDeque<PacketData>, // sent packets used to calculate sent bandwidth (kept until rtt_maximum)
    pendingAckQueue: VecDeque<PacketData>, // sent packets which have not been acked yet (kept until rtt_maximum * 2 )
    receivedQueue:   VecDeque<PacketData>, // received packets for determining acks to send (kept up to most recent recv sequence - 32)
    ackedQueue:      VecDeque<PacketData>, // acked packets (kept until rtt_maximum * 2)
}

impl ReliabilitySystem {
    pub fn reset(&mut self)
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

    fn PacketSent(&mut self, size: u32 )
    {
        if ( self.sentQueue.exists( self.local_sequence ) )
        {
            println!( "local sequence {} exists", self.local_sequence );
            for itor in self.sentQueue.iter() {
                println!("{}",itor.sequence);
            }
        }
        assert!( !self.sentQueue.exists( self.local_sequence ) );
        assert!( !self.pendingAckQueue.exists( self.local_sequence ) );
        let mut data = PacketData::default();
        data.sequence = self.local_sequence;
        data.time = 0.0f32;
        data.size = size;
        self.sentQueue.push_back( data );
        self.pendingAckQueue.push_back( data );
        self.sent_packets += 1;
        self.local_sequence = self.sent_packets + 1;
        if ( self.local_sequence > self.max_sequence ) {
            self.local_sequence = 0;
        }
    }

    fn PacketReceived(&mut self, sequence: u32, size: u32 )
    {
        self.recv_packets += 1;
        if ( self.receivedQueue.exists( sequence ) ) {
            return;
        }
        let mut data = PacketData::default();
        data.sequence = sequence;
        data.time = 0.0f32;
        data.size = size;
        self.receivedQueue.push_back( data );
        if ( sequence_more_recent( sequence, self.remote_sequence, self.max_sequence ) ){
            self.remote_sequence = sequence;
        }
    }

    fn GenerateAckBits(&self) -> u32
    {
        return self.generate_ack_bits( self.GetRemoteSequence(), &self.receivedQueue, self.max_sequence );
    }

    fn ProcessAck(&self, ack: u32, ack_bits: u32 )
    {
        self.process_ack( ack, ack_bits, &self.pendingAckQueue, &self.ackedQueue, &self.acks, self.acked_packets, &mut self.rtt, self.max_sequence );
    }

    fn Update(&mut self, deltaTime: f32 )
    {
        self.acks.clear();
        self.AdvanceQueueTime( deltaTime );
        self.UpdateQueues();
        self.UpdateStats();

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



    fn generate_ack_bits(&self, ack: u32, received_queue: &VecDeque<PacketData> , max_sequence: u32) -> u32
    {
        let mut ack_bits = 0u32;
        for itor in received_queue.iter() {
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
                             pending_ack_queue: &VecDeque<PacketData>, acked_queue: &VecDeque<PacketData>,
                             acks: &Vec<u32>, acked_packets: u32,
                             rtt: &mut f32, max_sequence: u32 )
    {
        if ( pending_ack_queue.is_empty() ) {
            return;
        }

        let mut index = 0usize;
        for itor in pending_ack_queue.iter()
        {
            let mut acked = false;

            if ( itor.sequence == ack )
            {
                acked = true;
            }
            else if ( !sequence_more_recent( itor.sequence, ack, max_sequence ) )
            {
                let bit_index = bit_index_for_sequence( itor.sequence, ack, max_sequence );
                if ( bit_index <= 31 ) {
                    acked = (( ack_bits >> bit_index ) & 1) != 0;
                }
            }

            if ( acked )
            {
                *rtt += ( itor.time - *rtt ) * 0.1f32;

                acked_queue.insert_sorted( *itor, max_sequence );
                acks.push( itor.sequence );
                acked_packets = acked_packets + 1;
                pending_ack_queue.remove( index );
            }
            index += 1;
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

    fn GetSentPackets(&self) -> u32
    {
        self.sent_packets
    }

    fn GetReceivedPackets(&self) -> u32
    {
        self.recv_packets
    }

    fn GetLostPackets(&self) -> u32
    {
        self.lost_packets
    }

    fn GetAckedPackets(&self) -> u32
    {
        self.acked_packets
    }

    fn GetSentBandwidth(&self) -> f32
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

    fn AdvanceQueueTime(&mut self, deltaTime: f32 )
    {
        for itor in self.sentQueue.iter_mut() {
            itor.time += deltaTime;
        }

        for itor in self.receivedQueue.iter_mut() {
            itor.time += deltaTime;
        }

        for itor in self.pendingAckQueue.iter_mut() {
            itor.time += deltaTime;
        }

        for itor in self.ackedQueue.iter_mut() {
            itor.time += deltaTime;
        }
    }

    fn UpdateQueues(&mut self)
    {
        let epsilon = 0.001f32;

        while ( (self.sentQueue.len() > 0) && self.sentQueue.front().unwrap().time > self.rtt_maximum + epsilon ) {
            self.sentQueue.pop_front();
        }

        if ( self.receivedQueue.len() > 0 )
        {
            let latest_sequence = self.receivedQueue.back().unwrap().sequence;
            let minimum_sequence = if latest_sequence >= 34 { latest_sequence - 34 } else { self.max_sequence - ( 34 - latest_sequence) };
            // let minimum_sequence = latest_sequence >= 34 ? ( latest_sequence - 34 ) : max_sequence - ( 34 - latest_sequence );
            while ( (self.receivedQueue.len() > 0) && !sequence_more_recent( self.receivedQueue.front().unwrap().sequence, minimum_sequence, self.max_sequence ) ) {
                self.receivedQueue.pop_front();
            }
        }

        while ( (self.ackedQueue.len() > 0) && self.ackedQueue.front().unwrap().time > self.rtt_maximum * 2.0f32 - epsilon ) {
            self.ackedQueue.pop_front();
        }

        while ( (self.pendingAckQueue.len() > 0) && self.pendingAckQueue.front().unwrap().time > self.rtt_maximum + epsilon )
        {
            self.pendingAckQueue.pop_front();
            self.lost_packets += 1;
        }
    }

    fn UpdateStats(&mut self)
    {
        let mut sent_bytes_per_second = 0;
        for itor in self.sentQueue.iter() {
            sent_bytes_per_second += itor.size;
        }
        let mut acked_packets_per_second = 0;
        let mut acked_bytes_per_second = 0;
        for itor in self.ackedQueue.iter()
        {
            if ( itor.time >= self.rtt_maximum )
            {
                acked_packets_per_second += 1;
                acked_bytes_per_second += itor.size;
            }
        }
        sent_bytes_per_second /= sent_bytes_per_second / self.rtt_maximum as u32;
        acked_bytes_per_second /= acked_bytes_per_second / self.rtt_maximum as u32;
        self.sent_bandwidth = sent_bytes_per_second as f32 * ( 8.0f32 / 1000.0f32 );
        self.acked_bandwidth = acked_bytes_per_second as f32 * ( 8.0f32 / 1000.0f32 );
    }
}
#[derive(PartialEq)]
pub enum State {
    Disconnected,
    Listening,
    Connecting,
    ConnectFail,
    Connected,
}
#[derive(PartialEq)]
pub enum Mode {
    Null,
    Client,
    Server,
}

pub struct ReliableConnection {
    address:            SocketAddr,
    socket:             UdpSocket,

    protocolId:         u32,
    state:              State,
    mode:               Mode,
    running:            bool,
    timeout:            f32,
    timeoutAccumulator: f32,

    reliabilitySystem:  ReliabilitySystem,
}

impl Default for ReliableConnection {
    fn default () -> ReliableConnection {

        ReliableConnection {
            address: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 1234)),
            socket: UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 1234)).unwrap(),
            protocolId: 0,
            state: State::Disconnected,
            mode: Mode::Null,
            running: false,
            timeout: 0.0f32,
            timeoutAccumulator: 0.0f32,

            reliabilitySystem: Default::default()
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
    fn start(&mut self, addr: SocketAddr) -> bool
    {
        assert!( !self.running );
        println!( "start connection on addr {}", addr );
        // if ( !socket.bind( addr ) )
        //     return false;
        self.running = true;
        return true;
    }

    fn stop(&mut self)
    {
        assert!( self.running );
        println!("stop connection");
        let connected = self.is_connected();
        self.clear_data();
        drop(self.socket);
        self.running = false;
        if ( connected ) {
            self.on_disconnect();
        }
        self.on_stop();
    }

    fn is_running(&self) -> bool
    {
        self.running
    }

    fn listen(&mut self)
    {
        println!( "server listening for connection\n" );
        let connected = self.is_connected();
        self.clear_data();
        if ( connected ) {
            self.on_disconnect();
        }
        self.mode = Mode::Server;
        self.state = State::Listening;
    }

    fn connect(&mut self, addr: SocketAddr)
    {
        println!( "client connecting to {}", addr);
        let connected = self.is_connected();
        self.clear_data();
        if ( connected ) {
            self.on_disconnect();
        }
        self.mode = Mode::Client;
        self.state = State::Connecting;
        self.address = addr;
    }

    fn is_connecting(&self) -> bool
    {
        self.state == State::Connecting
    }

    fn connect_failed(&self) -> bool
    {
        self.state == State::ConnectFail
    }

    fn is_connected(&self) -> bool
    {
        self.state == State::Connected
    }

    fn is_listening(&self) -> bool
    {
        self.state == State::Listening
    }

    fn get_mode(&self) -> Mode
    {
        self.mode
    }

    pub fn update(&self, deltaTime: f32)
    {
        assert!( self.running );
        self.timeoutAccumulator += deltaTime;
        if ( self.timeoutAccumulator > self.timeout )
        {
            if ( self.state == State::Connecting )
            {
                println!( "connect timed out\n" );
                self.clear_data();
                self.state = State::ConnectFail;
                self.on_disconnect();
            }
            else if ( self.state == State::Connected )
            {
                println!( "connection timed out\n" );
                self.clear_data();
                if ( self.state == State::Connecting ) {
                    self.state = State::ConnectFail;
                }
                self.on_disconnect();
            }
        }
    }
    // 添加4字节的 协议ID 后发送
    pub fn send_packet(&self, data: &mut [u8], size: u32) -> bool
    {
        assert!( self.running );

        // uchar_t packet[size + 4];
        let mut packet: Vec<u8> = Vec::with_capacity((size + 4) as usize);
        packet[0] = ( self.protocolId >> 24 ) as u8 ;
        packet[1] = ( ( self.protocolId >> 16 ) & 0xFF ) as u8;
        packet[2] = ( ( self.protocolId >> 8 ) & 0xFF ) as u8;
        packet[3] = ( ( self.protocolId ) & 0xFF ) as u8;

        // memcpy( &packet[4], data, size );
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), &mut packet[4], size as usize);
        }
        
        match self.socket.send_to(&packet, &self.address) {
            Ok(result) => return true,
            Err(..) => return false
        }
    }

    pub fn receive_packet(&mut self, data: &[u8],  size: u32) -> i32
    {
        assert!(self.running);
        // uchar_t packet[size + 4];
        let mut packet: Vec<u8> = Vec::with_capacity((size + 4) as usize);

        let mut bytes_read = 0usize;
        let mut sender = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 1234));
        match self.socket.recv_from(&mut packet) {
            Ok(result) => {
                let (_bytes_read, _sender) = result;
                bytes_read = _bytes_read;
                sender = _sender;
            },
            Err(..) => return 0,
        }
        if ( bytes_read == 0 ) {
            return 0;
        }
        if ( bytes_read <= 4 ) {
            return 0;
        }
        if ( packet[0] != ( self.protocolId >> 24 ) as u8 ||
                packet[1] != ( ( self.protocolId >> 16 ) & 0xFF ) as u8 ||
                packet[2] != ( ( self.protocolId >> 8 ) & 0xFF ) as u8||
                packet[3] != ( self.protocolId & 0xFF ) as u8) {
            return 0;
        }

        if ( (self.mode == Mode::Server) && !self.is_connected() )
        {
            println!( "server accepts connection from client {}", sender);
            self.state = State::Connected;
            self.address = sender;
        }
        if ( sender == self.address )
        {
            if ( (self.mode == Mode::Client) && (self.state == State::Connecting) )
            {
                println!("client completes connection with server");
                self.state = State::Connected;
            }
            self.timeoutAccumulator = 0.0f32;

            //memcpy( data, &packet[4], bytes_read - 4 );
            unsafe {
                ptr::copy_nonoverlapping(data.as_ptr(), &mut packet[4], bytes_read - 4);
            }
            
        }
        return 0;
    }

    fn get_header_size() -> i32 {
        return 4;
    }

    fn on_stop(&self) {
        self.clear_data();
    }

    fn on_connect() {

    }

    fn on_disconnect(&self) {
        self.clear_data();
    }

    fn clear_data(&mut self) {
        self.state = State::Disconnected;
        self.timeoutAccumulator = 0.0f32;
        self.address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 1234));
    }
}

#[test]
fn test_linked_list() {
    let mut ll: VecDeque<PacketData> = VecDeque::new();
    let pd0 = PacketData{sequence: 0, time: 0.0f32, size: 128u32};
    ll.insert_sorted(pd0, 128u32);
}

} // end of mod socket