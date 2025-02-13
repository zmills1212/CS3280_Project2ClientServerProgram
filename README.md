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
