use std::net::TcpListener;

use rustserv::connection_handler::ConnectionHandler;
use rustserv::error::Result;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut handler = ConnectionHandler::new(&mut stream);
                handler.handle_connection()?;
            }
            Err(e) => {
                println!("Connection failed: {e}");
            }
        }
    }
    Ok(())
}
