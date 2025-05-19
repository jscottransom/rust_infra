use std::{
    io::{prelude::*, Result, Error},
    fs::File,
    sync::{Arc}
};

use tokio::net::{TcpListener, TcpStream};
use tokio::io::BufReader;
use tokio::io::AsyncBufReadExt;
use tokio::sync::Mutex;


#[tokio::main]
async fn main() -> Result<() >{
    let file = File::create("log_lines.txt")?;
    let safe_file = Arc::new(Mutex::new(file));
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    
    loop {
        let (socket, _) = listener.accept().await?;
        let file_clone = safe_file.clone();
        tokio::spawn(async move {
            
            // Process each socket concurrently.
            if let Err(e) = handle(socket, file_clone).await {
                println!("Error: {}", e)
            }
        });
    }
}

async fn handle(socket: TcpStream, safe_file: Arc<Mutex<File>>) -> Result<()> {
    let buf_reader = BufReader::new(socket);
    let mut lines = buf_reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        
        
        if line.trim().is_empty() {
            break;
        }

        let mut file = safe_file.lock().await;
        writeln!(file, "{}", line)?;
    }

    Ok(())

}