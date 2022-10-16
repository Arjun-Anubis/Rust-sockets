extern crate log;
use std::io::prelude::*;



fn handle( mut stream: std::net::TcpStream ) -> std::io::Result<usize> {
        stream.write(  b"Hi, from server\n" )
}
fn server() {
    let listener = std::net::TcpListener::bind("127.0.0.1:5000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection from {:?}", stream.peer_addr() );
        std::thread::spawn( move || {
            match handle( stream ) {
                Ok(bytes) => log::info!( "Wrote {} bytes", bytes ),
                Err(..) => log::warn!( "Could not send" )
            }
        });
    }
}

fn main() {
    pretty_env_logger::init();
    log::info!("We're Up and running!");
    server()
}
