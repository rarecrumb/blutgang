// All as floats so we have an easier time getting averages, stats and terminology copied from flood.
#[derive(Debug, Clone, Default, Copy)]
pub struct Status {
	pub is_erroring: bool,
	pub latency: f64,
	pub throughput: f64,
}

#[derive(Debug, Clone)]
pub struct Rpc {
	pub url: String, // url of the rpc we're forwarding requests to.
	pub rank: i32, // rank of the rpc, higer is better.
	pub status: Status, // stores stats related to the rpc.
}

// implement new for rpc
impl Rpc {
	pub fn new(url: String) -> Self {
		Self{
			url: url,
			rank: 0,
			status: Status::default(),
		}
	}
}
