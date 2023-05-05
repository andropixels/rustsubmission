# Rust Chat Server with Tokio

This Rust project is a simple chat server built with Tokio. It broadcasts messages to all clients connected to the server, except for the sender of the message.

Getting Started
To run the chat server, navigate to the project directory and execute the following command:
    ## cargo run 

This will start the chat server on localhost at port 8080.

# Connecting to the Server
To connect to the chat server, you can use any Telnet client. For example, on macOS or Linux, you can use the built-in telnet command. On Windows, you can use PuTTY or another Telnet client.

To connect to the chat server, execute the following command in your terminal:

telnet localhost 8080

 