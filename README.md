# Rust TCP Echo Server

This project is a simple TCP echo server implemented in Rust. The server echoes back all data it receives from the client, which can be used to gain a better understanding of TCP socket communication.

## Getting Started

### Prerequisites

To run this project, you will need Rust and Cargo installed on your machine.

### Installation and Usage

1. Clone this repository to your local machine.
2. Navigate to the project directory and run the command `cargo run`.
3. The server will start listening for incoming connections on port 8080.
4. You can connect to the server using a TCP client, such as `netcat`, by running the command `nc localhost 8080`.
5. Type any text and press Enter. The server will echo back your message.

## Contributing

Contributions to this project are welcome.

## License

This project is licensed under the MIT License. 
