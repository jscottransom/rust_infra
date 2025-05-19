# Distributed Infrastructure Practice Projects

This repository contains hands-on exercises to practice building distributed infrastructure systems using **Rust** and **Go**. These small, idiomatic projects are meant to help you understand core concepts such as messaging, concurrency, file safety, and network communication in a distributed context.

## 📦 Project List

### 🦀 Rust Projects

#### 1. UDP Heartbeat System
- **Goal**: Build a UDP client that periodically sends JSON-encoded heartbeats to a central server.
- **Concepts**: JSON serialization (`serde`), UDP sockets with Tokio, message broadcasting.
- **Outcome**: Understand stateless communication over UDP and how to structure minimal, efficient network messages.

#### 2. Concurrent TCP Log Aggregator
- **Goal**: Accept TCP connections and write each client's log lines to a shared file safely.
- **Concepts**: Tokio tasks, mutex synchronization, shared file access with `Arc<Mutex<_>>`, async file-safe writes.
- **Outcome**: Gain experience handling many clients concurrently and managing shared resources in an async environment.

#### 3. In-Memory KV Store with Message Passing
- **Goal**: Build an actor-based key-value store where clients set/get values via TCP or channels.
- **Concepts**: Message-passing patterns, shared mutable state, command parsing.
- **Outcome**: Practice state encapsulation and message-based concurrency.

#### 4. Gossip Protocol Simulator
- **Goal**: Simulate peer-to-peer message spreading using a gossip protocol.
- **Concepts**: Random peer selection, periodic communication, topology awareness.
- **Outcome**: Learn how decentralized coordination works without a central controller.

#### 5. Distributed File Writer
- **Goal**: Accept chunked data from multiple clients and write to specific parts of a file based on offsets.
- **Concepts**: File seeks, partial writes, chunked data protocols.
- **Outcome**: Practice atomic file operations and spatial coordination.

---

### 🐹 Go Projects

#### 1. HTTP-Based Service Registry
- **Goal**: Build a minimal Consul-like registry where services can register and query others.
- **Concepts**: HTTP routing, maps for in-memory storage, TTLs for expiration.
- **Outcome**: Understand service discovery mechanisms and TTL-based availability.

#### 2. Pub/Sub Over TCP
- **Goal**: Implement a publisher-subscriber system where clients subscribe to topics and receive real-time messages.
- **Concepts**: Goroutines, channels, topic mapping, broadcast fan-out.
- **Outcome**: Learn to decouple producers and consumers using idiomatic concurrency.

#### 3. Leader Election via Heartbeats
- **Goal**: Simulate multiple nodes and determine a leader based on liveness and priority.
- **Concepts**: Goroutines, timers, heartbeat messages, simple election algorithms.
- **Outcome**: Understand consensus fundamentals and availability checks.

#### 4. Fan-Out Metrics Collector
- **Goal**: Build a central collector that ingests metrics from edge nodes and logs them with timestamps.
- **Concepts**: TCP servers, batching, timestamp tagging, centralized ingestion.
- **Outcome**: Practice data collection from distributed inputs.

#### 5. File Transfer Over TCP with Checksums
- **Goal**: Send files in chunks with integrity verification using checksums.
- **Concepts**: TCP streaming, hashing (SHA-256), chunking, reassembly.
- **Outcome**: Gain experience with transport-layer data validation.

---

## 🧠 Skills Targeted

- Asynchronous programming
- TCP/UDP socket programming
- Concurrent file access
- Serialization (JSON, binary)
- Fault detection & liveness
- Messaging patterns (Pub/Sub, RPC, actor model)


## 🚀 Getting Started

Each project has its own directory. Navigate into any project folder and run:

