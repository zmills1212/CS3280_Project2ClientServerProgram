Sales Data Transfer System

Project Description

This Sales Data Transfer System is a Rust-based client-server application designed to transfer sales data securely and efficiently.
The client reads sales reports, encodes them in Base64, and sends them over a TCP connection. The server receives, decodes, and stores the reports in organized branch folders.

Features:

Client-side-

.Reads sales data from a predefined file.

.Encodes the file in Base64.

.Establishes a TCP connection with the server.

.Sends branch identification and sales data.

.Logs communication and errors.

.Server-side

.Listens for incoming client connections.

.Handles multiple clients using multithreading.

.Creates branch-specific folders for storing sales data.

.Decodes and saves received sales reports.

.Logs communication and errors.





How to Run the Programs

Set Up:

Make sure Rust is installed on your machine. If not, install it from rust-lang.org.
Open File Explorer on your Windows machine.
Navigate to the project directory: \code\src.
In the File Explorer search bar, type cmd and press Enter. This will open a command prompt at the current directory location.

Navigate to the client project within the project folder by using cd and changing the directory to the client
In another terminal window, navigate to the server directory by using the cd command in the terminal 

Building the Program:

In the command prompt, run:
cargo build for both client and server.
Must build client in client terminal window and server in server terminal window 

Running the Program:

After building, run the program by typing:
cargo run in the server window first to create the server 
Cargo run in the client window so that the client can now connect to the server

Data for the branches and their weekly summaries are sent from the client  to the server
Through a tcp connection.
