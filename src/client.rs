use std::io::prelude::*;

fn client() -> std::io::Result<()> {
    let mut stream = std::net::TcpStream::connect( "127.0.0.1:5000" )?;
    let mut data = String::new();

    stream.read_to_string( &mut data )?;
    println!( "Recieved {}", data );

    Ok(())
}
fn main() {
    log::info!( "Starting client" );
    client().unwrap();
}
