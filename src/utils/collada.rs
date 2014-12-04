pub mod collada {
	// parse .dae file
	// <mesh> <source id = "">  </source> </mesh>
	// 需要先解析 source, 然后是 verteies, triangles
	enum ParseType {
	    vertex,
	    triangle,
	}

	struct Collada {
	    content: String,
	    tag: str,
	}

	impl Collada {
		fn new(path: str) -> Collada {
			let content = File::open(Path::new(path).read_to_string().unwrap());
			Collada{content: content, tag: ""}
		}
		fn parse_vertex(&self) -> [f32..] {
	        let path = Path::new("/tmp/data.txt");
			let raw_string = File::open(&path).read_to_string().unwrap();
			let idx_lib_geo_start = BoyerMoore::new(raw_string.as_slice(), "<library_geometries>").search();
			let idx_lib_geo_end = BoyerMoore::new(raw_string.as_slice()[idx_lib_geo_start, raw_string.len()], "</library_geometries>").search();
		}
		pub fn parse(&self, t: ParseType) -> [f32..] {
			match t {
				// library_geometries: geometry: mesh:
				self.tag = ""
				ParseType::vertex => parse_vertex()
			}
		}
	}
	
}