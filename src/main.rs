use std::net::{TcpListener};
mod handle;
use tcp_rust::ThreadPool;
fn main() -> std::io::Result<()> 
{
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
        for stream in listener.incoming() {
        match stream  {
            Ok(stream) => {pool.execute(||  {handle::handle_connection(stream);});}
            Err(e) => {println!("{}",e);}
        }
    }
    Ok(())
}
