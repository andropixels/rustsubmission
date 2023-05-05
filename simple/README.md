# Rust Simple Client for Collecting BTC Prices

This Rust project provides a simple client that can be run in two modes: cache and read.

# Running the Client

./simple  --mode=<MODE>

# Cache Mode

In cache mode, the client connects via socket for 10 seconds only, collects all USD prices of BTC that it can during the 10 second period, and calculates an aggregate using these prices. The client then saves both the result of the aggregate and the data points used to create the aggregate to a file.

./simple --mode=cache 


# Read Mode
In read mode, the client simply reads the data points and aggregate result from the file created by the client in cache mode and prints them to the console.

./simple --mode=read