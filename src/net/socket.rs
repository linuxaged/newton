use std::net::UdpSocket;
use std::collections::LinkedList;

struct PacketData
{
    sequence: u32,          // packet sequence number
    time: f32,                     // time offset since packet was sent or received (depending on context)
    size: u32,                       // packet size in bytes
};

trait PacketQueue {
	fn exists(mut sequence: u32) -> bool;
	fn insert_sorted(p: &PacketData,  max_sequence: u32);
	fn verify_sorted(max_sequence: u32);
}

impl PacketQueue for LinkedList<PacketData> {
	
}

struct ReliabilitySystem {
    max_sequence: u32,          // maximum sequence value before wrap around (used to test sequence wrap at low # values)
    local_sequence: u32,        // local sequence number for most recently sent packet
    remote_sequence: u32,       // remote sequence number for most recently received packet

    sent_packets: u32,          // total number of packets sent
    recv_packets: u32,          // total number of packets received
    lost_packets: u32,          // total number of packets lost
    acked_packets: u32,         // total number of packets acked

    sent_bandwidth: f32,               // approximate sent bandwidth over the last second
    acked_bandwidth: f32,              // approximate acked bandwidth over the last second
    rtt            : f32,              // estimated round trip time
    rtt_maximum: f32,                  // maximum expected round trip time (hard coded to one second for the moment)

    std::vector<uint32_t> acks;     // acked packets from last set of packet receives. cleared each update!

    PacketQueue sentQueue;              // sent packets used to calculate sent bandwidth (kept until rtt_maximum)
    PacketQueue pendingAckQueue;        // sent packets which have not been acked yet (kept until rtt_maximum * 2 )
    PacketQueue receivedQueue;          // received packets for determining acks to send (kept up to most recent recv sequence - 32)
    PacketQueue ackedQueue;             // acked packets (kept until rtt_maximum * 2)
}

struct ReliableConnection {
	socket: UdpSocket,

	timeout: f32,
	timeoutAccumulator: f32,

	reliableSsytem: ReliableConnection,
}

impl ReliableConnection {
	fn Update() {

	}
}