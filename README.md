# go2go

Welcome to go2go! This project is a peer-to-peer Go (goban) game platform developed in Rust, featuring a server that manages game states and a client with a graphical user interface (GUI) built using WebAssembly (Wasm). The client is designed to run in the browser, providing a smooth and interactive experience for playing Go.

## Project Structure

```
goban-project/
│
├── server/
│   ├── src/
│   │   ├── main.rs
│   │   └── game_logic.rs
│   ├── Cargo.toml
│   └── Cargo.lock
│
├── client/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── components.rs
│   │   └── game_state.rs
│   ├── static/
│   │   └── index.html
│   ├── Cargo.toml
│   └── Cargo.lock
│
├── README.md
└── .gitignore
```

### Server

The server handles the game logic, state management, and communication between clients. It is built using the following technologies:
- **Warp**: A lightweight web framework for building the HTTP server.
- **Tokio**: An asynchronous runtime for handling concurrent tasks.
- **Serde**: A framework for serializing and deserializing Rust data structures.

#### Running the Server

To run the server, navigate to the `server/` directory and execute the following command:

```bash
cd server
cargo run
```

The server will start on `localhost:3030`.

### Client

The client is a web application built using the Yew framework, which allows for creating front-end applications with Rust and WebAssembly. It provides a graphical user interface for playing the game and communicates with the server for real-time updates.

#### Building and Running the Client

To build the client for WebAssembly, follow these steps:

1. Navigate to the `client/` directory and build the client:

   ```bash
   cd client
   wasm-pack build --target web
   ```

2. Serve the static files in the `static/` directory using a simple HTTP server:

   ```bash
   cd static
   http-server .
   ```

3. Open `http://localhost:8080` (or the port provided by your HTTP server) in your web browser to view the client.

## Features

- **Peer-to-peer communication**: The project supports peer-to-peer communication for a decentralized gaming experience.
- **WebAssembly client**: The client is built with Yew and compiled to WebAssembly, providing a modern and efficient web application experience.
- **Cross-platform**: The client can run on any platform that supports modern web browsers.

## Contributing

Contributions are welcome! If you'd like to contribute to this project, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgements

- [Yew](https://yew.rs/) for providing a robust framework for building web applications in Rust.
- [Warp](https://github.com/seanmonstar/warp) for creating an ergonomic web framework in Rust.
- [Tokio](https://tokio.rs/) for asynchronous programming in Rust.
