// 1.Server -> Start (update waiting for data)
// 2.Client -> Connect to Server
// 3.Server -> Send snapshot to all clients
// snapshot 的计算和渲染：http://gamedev.stackexchange.com/questions/87553/how-would-a-game-state-snapshot-system-be-implemented-for-networked-real-time-ga
// Quake3 相关 snapshot 参考：https://github.com/id-Software/Quake-III-Arena/blob/dbe4ddb10315479fc00086f08e25d968b4b43c49/code/server/sv_snapshot.c

trait Connection {
	fn isConnect(&self) -> bool;
}

trait ReliableConnection {
	fn isConnect(&self) -> bool;
}

impl Connection for i32 {
	name: String;
	fn isConnect(&self) -> bool {
		false
	}
}

fn main() {
	println!("{}",5.isConnect());
}
