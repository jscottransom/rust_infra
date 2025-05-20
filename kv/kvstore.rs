use tokio;
use std::io;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

enum Value {
    Text(String),
    Number(u16),
}

enum Command {
    Set,
    Get,
    Delete,
}
struct Record {
    key: String,
    value: Value
}

type Kvstore = HashMap<String, Value>;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Setup in-memory kv store
    let store = Kvstore::new();
    let safe_store = Arc::new(Mutex::new(store));
    
    // Setup TCP Server, move safe store into spawned threads and handle requests
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    }

    Ok(())
}





