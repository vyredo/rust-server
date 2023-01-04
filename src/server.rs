use std::{net::TcpListener, io::Read};
use crate::http::Request; // crate means root of this project (src)
use std::convert::{TryFrom, TryInto};

pub struct Server {
	addr: String,
}

impl Server {
	pub fn new(addr: String) -> Self {
		Self { addr }
	}

	// method is not part of the struct
	pub fn run(self) {
		println!("Listening on {}", self.addr);


		// slice is a ref to an array
		// unwrap is the OK Functor
		let listener = TcpListener::bind(&self.addr).unwrap();


		loop {
			match listener.accept() {
				Ok((mut stream, addr)) => {

					println!("new client: {addr:?}");
					let mut buf = [0; 1024];
					match stream.read(&mut buf) {
						Ok(_) => {
							println!("request: {}", String::from_utf8_lossy(&buf));


							match Request::try_from(&buf[..]) {
								Ok(request) => {},
								Err(e) => println!("failed to parse request: {e:?}"),
							}


						}
						Err(e) => println!("failed to read from connection: {e:?}"),
					}
				},
				Err(e) => println!("couldn't get client: {e:?}"),
			}

		}
	}
}
