use std::net::{TcpListener};
 
mod handle;

fn main() -> std::io::Result<()> 
{
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
        match stream  {
            Ok(stream) => {handle::handle_connection(stream);}
            Err(e) => {println!("{}",e);}
        }
    }
    Ok(())
}
