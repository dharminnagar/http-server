# HTTP Server from Scratch

A low-level HTTP server implementation in **Rust**, built without any web frameworks to understand the fundamentals of network programming and concurrency.

## What I Learned

Exploring low-level really is interesting. Here's what I actually learnt:

- **Raw TCP networking and binding** - Direct socket programming with `TcpListener` and `TcpStream`
- **Multi-threading in Rust** - Handling concurrent connections safely with thread spawning
- **Custom Thread Pool implementation** - Avoiding unlimited CPU threads by reusing a fixed pool of workers
- **Arc, Mutex, mpsc** - Using Rust's concurrency primitives to distribute network traffic to workers
- **Graceful shutdown** - Cleaning up worker threads using the `Drop` trait
- **HTTP protocol parsing** - Manually reading and parsing HTTP request lines (method, path, version)
- **Pattern matching for routing** - Using Rust's `match` expressions to route requests to different handlers
- **Worker pattern** - Creating long-lived worker threads that wait for jobs in a loop
- **HTTP response formatting** - Constructing responses with status lines, headers (`Content-Length`), and body content
- **Channel-based job distribution** - Using `mpsc::Sender` and `mpsc::Receiver` to send closures between threads

## Features

- HTTP request routing (/, /sleep, 404)
- Thread pool with 4 workers for concurrent request handling
- Static file serving from `public/` directory
- Manual HTTP response formatting

## Running the Server

```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`

## Routes

- `GET /` - Returns the main response page
- `GET /sleep` - Simulates a slow request (5 second delay) to demonstrate concurrent handling
- All other routes return a 404 page

## Note on Production Readiness

⚠️ This project uses `.unwrap()` extensively for error handling, which should **not** be used in production code. This was purely an experimental learning project to understand the underlying mechanics of HTTP servers and Rust concurrency. In a real application, you'd want proper error handling with `Result` types and meaningful error messages.

## Architecture

The server consists of:

1. **Main thread** - Accepts incoming TCP connections
2. **Thread pool** - Fixed number of worker threads (4) that handle requests
3. **Job queue** - Uses `mpsc` channels to distribute work to available workers
4. **Workers** - Long-lived threads that process HTTP requests

Each worker shares access to the job receiver using `Arc<Mutex<mpsc::Receiver<Job>>>`, ensuring thread-safe distribution of work.
