# Distributed Infrastructure Practice Projects

This repository contains hands-on exercises to practice building distributed infrastructure systems using **Rust** and **Go**. These small-medium projects are meant to enforce core concepts such as messaging, concurrency, file safety, and network communication in a distributed context.

## 📦 Project List

### 🦀 Rust Projects

#### 1. UDP Heartbeat System
- **Goal**: Build a UDP client that periodically sends JSON-encoded heartbeats to a central server.
- **Concepts**: JSON serialization (`serde`), UDP sockets with Tokio, message broadcasting.

#### 2. Concurrent TCP Log Aggregator
- **Goal**: Accept TCP connections and write each client's log lines to a shared file safely.
- **Concepts**: Tokio tasks, mutex synchronization, shared file access with `Arc<Mutex<_>>`, async file-safe writes.

#### 3. In-Memory KV Store with Message Passing
- **Goal**: Build an actor-based key-value store where clients set/get values via TCP or channels.
- **Concepts**: Message-passing patterns, shared mutable state, command parsing.


#### 4. Gossip Protocol Simulator
- **Goal**: Simulate peer-to-peer message spreading using a gossip protocol.
- **Concepts**: Random peer selection, periodic communication, topology awareness.


## 🧠 Skills Targeted

- Asynchronous programming
- TCP/UDP socket programming
- Concurrent file access
- Serialization (JSON, binary)
- Fault detection & liveness
- Messaging patterns (Pub/Sub, RPC, actor model)


## 🚀 Getting Started

Each project has its own directory. Navigate into any project folder and run:

