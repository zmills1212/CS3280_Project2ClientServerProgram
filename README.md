# Sales Data Transfer System

This project implements a Rust-based client-server application for secure and efficient transfer of sales data. The client encodes sales reports in Base64 and transmits them over a TCP connection to the server, which decodes and organizes the data into branch-specific folders.

## Features

**Client-side:**

* Reads sales data from a predefined file.
* Encodes the data in Base64.
* Establishes a TCP connection with the server.
* Sends branch identification and sales data.
* Logs communication and errors.

**Server-side:**

* Listens for incoming client connections.
* Handles multiple clients concurrently using multithreading.
* Creates branch-specific folders for storing sales data.
* Decodes and saves received sales reports.
* Logs communication and errors.

## How to Run

### Setup

1. **Install Rust:** Ensure Rust is installed on your machine. If not, download and install it from [rust-lang.org](https://www.rust-lang.org/).

2. **Navigate to Project Directory:**
   * Open File Explorer on your Windows machine.
   * Navigate to the project directory: `\code\src`.
   * In the File Explorer search bar, type `cmd` and press Enter to open a command prompt at the current directory.

3. **Open Separate Terminals:**
   * In one terminal window, navigate to the client directory using the `cd` command.
   * In another terminal window, navigate to the server directory using the `cd` command.

### Building

1. **Build Client and Server:**
   * In the client terminal window, run: `cargo build`
   * In the server terminal window, run: `cargo build`

### Running

1. **Start the Server:** In the server terminal window, run: `cargo run`
2. **Start the Client:** In the client terminal window, run: `cargo run`

This will initiate the data transfer process, with the client sending branch identification and weekly sales summaries to the server over a TCP connection.

## Files

* `client/src/main.rs`:  Client-side code.
* `server/src/main.rs`:  Server-side code.
* `README.md`: This file.

## Data Flow

1. The client reads sales data from a predefined file.
2. The client encodes the sales data using Base64.
3. The client establishes a TCP connection with the server.
4. The client sends the branch ID and encoded sales data to the server.
5. The server receives the data and decodes it.
6. The server creates branch-specific folders if they don't exist.
7. The server saves the decoded sales reports in the appropriate branch folders.

## Error Handling

Both client and server incorporate error handling and logging mechanisms to ensure robust operation and facilitate troubleshooting.
