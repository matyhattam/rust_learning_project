use std::io::Read;
use std::net::TcpListener;

use crate::http::Request;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address: address }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                // By adding underscore in the tuple we say that this value is not important to us
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(Request) => {}
                                Err(error) => println!("Failes to parse the request: {}", error),
                            }
                        }
                        Err(error) => println!("Failed to read from connection: {}", error),
                    }
                }
                // We could just have added an _ to wrap the other value of the match if not needed for the rest
                Err(error) => println!("Failed to establish a connection: {}", error),
            }
        }
    }
}
