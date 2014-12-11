use std::io::File;
use boyermoore::BoyerMoore;

// parse .dae file
// <mesh> <source id = "">  </source> </mesh>
// 需要先解析 source, 然后是 verteies, triangles
enum ParseType {
    vertex,
    triangle,
    none,
}

struct Collada {
    content: String,
    tag: ParseType,
}

impl Collada {
	pub fn new(path: &str) -> Collada {
		let content = File::open(&Path::new(path)).read_to_string().unwrap();
		Collada{content: content, tag: ParseType::none}
	}

	fn parse_vertex(&self) {
		let idx_lib_geo_start = BoyerMoore::new(self.content.as_slice(), "<library_geometries>").search(false).unwrap();
		let idx_lib_geo_end = BoyerMoore::new(self.content.as_slice()[idx_lib_geo_start[0]..self.content.len()], "</library_geometries>").search(false).unwrap();
		println!("{},{}",idx_lib_geo_start, idx_lib_geo_end);
	}
	pub fn parse(&self, t: ParseType) {
		match t {
			ParseType::vertex => self.parse_vertex(),
			ParseType::triangle => (),
			ParseType::none => (),
		}
	}
}

#[test]
fn test_parse() {
	let collada = Collada::new("example/models/Badblue_fly.dae");
	collada.parse(ParseType::vertex);
}
