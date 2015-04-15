use std::net::UdpSocket;
use std::collections::LinkedList;
use std::vec;

struct PacketData {
    sequence: u32,          // packet sequence number
    time: f32,              // time offset since packet was sent or received (depending on context)
    size: u32,              // packet size in bytes
}

trait PacketQueue {
	fn exists(&self, mut sequence: u32) -> bool;
	fn insert_sorted(&self, p: &PacketData,  max_sequence: u32);
}

#[inline]
fn sequence_more_recent(s1: u32, s2: u32, max_sequence: u32) -> bool {
	(( s1 > s2 ) && ( s1 - s2 <= max_sequence / 2 )) || (( s2 > s1 ) && ( s2 - s1 > max_sequence / 2 ))
}

impl PacketQueue for LinkedList<PacketData> {
	fn exists(&self, mut sequence: u32) -> bool {
		for (i, elt) in self.iter().enumerate() {
			if elt.sequence == sequence {
				return true;
			}
		}
		return false;
	}

	fn insert_sorted(&self, p: &PacketData, max_sequence: u32) {
		if self.is_empty() {
			self.push_back(p);
		}
		else {
            if ( !sequence_more_recent( p.sequence, self.front().sequence, max_sequence ) ) {
                self.push_front( p );
            }
            else if ( sequence_more_recent( p.sequence, self.back().sequence, max_sequence ) ) {
                self.push_back( p );
            }
            else {
                for (i, elt) in self.iter_mut().enumerate() {
                    // assert( itor->sequence != p.sequence );
                    if ( sequence_more_recent( elt.sequence, p.sequence, max_sequence ) ) {
                        self.insert_next(p);
                        break;
                    }
                }
            }
        }
	}
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

    acks: Vec<u32>,     // acked packets from last set of packet receives. cleared each update!

    sentQueue: PacketQueue,              // sent packets used to calculate sent bandwidth (kept until rtt_maximum)
    pendingAckQueue: PacketQueue,        // sent packets which have not been acked yet (kept until rtt_maximum * 2 )
    receivedQueue: PacketQueue,          // received packets for determining acks to send (kept up to most recent recv sequence - 32)
    ackedQueue: PacketQueue,             // acked packets (kept until rtt_maximum * 2)
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

#[test]
fn test_linked_list() {
	let ll: LinkedList<PacketData> = LinkedList::new();
	let pd0 = PacketData{sequence: 0, time: 0.0f32, size: 128u32};
	ll.insert_sorted(&pd0, 128u32);
}