use actix::prelude::*;
use std::collections::HashMap;
use tokio;

#[derive(Debug)]
enum Value {
    Text(String),
    Number(u16),
}

struct Record {
    key: String,
    value: Value,
}

#[derive(Message)]
#[rtype("()")]

enum Command {
    Set(Record),
    Get(String),
    Delete(String),
}

type Store = HashMap<String, Value>;
struct KVstore {
    store: Store,
}

impl Actor for KVstore {
    type Context = Context<Self>;
}

impl Handler<Command> for KVstore {
    type Result = (); // <- Message response type

    fn handle(&mut self, msg: Command, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            Command::Set(record) => {
                let key_str = record.key.clone();
                let key_val = match &record.value {
                    Value::Text(val) => val.clone(),

                    Value::Number(val) => val.to_string(),
                };

                self.store.insert(record.key, record.value);
                println!("Set {} to Value {:?}", key_str, key_val.parse::<u16>())
            }
            Command::Get(key) => {
                let val = self.store.get(&key);
                println!("From key {}: Value {:?}", key, Some(&val))
            }
            Command::Delete(key) => {
                let key = self.store.remove(&key);
                println!("Deleted key: {:?}", Some(&key))
            }
            _ => println!("Not a command"),
        }
    }
}

#[actix::main] // <- starts the system and block until future resolves
async fn main() {
    let hash: HashMap<String, Value> = HashMap::new();
    let addr = KVstore { store: hash }.start();

    let record = Record {
        key: "Secret".to_string(),
        value: Value::Number(42),
    };
    let res = addr.send(Command::Set(record)).await; // <- send message and get future for result

    match res {
        Ok(_result) => println!("key set"),
        _ => println!("Communication to the actor has failed"),
    }
}
