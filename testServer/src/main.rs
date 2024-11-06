use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::sync::{Arc, Mutex};
use std::thread;
use std::path::Path;
use std::error::Error;

mod lib;

fn main() {
    if let Err(e) = start_listening("127.0.0.1:8080") {
        eprintln!("Server encountered an error: {}", e);
    }
}
//listen for client connection
fn start_listening(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?;
    println!("Server listening on {}", address);

    let listener = Arc::new(Mutex::new(listener));

    for stream in listener.lock().unwrap().incoming() {
        match stream {
            Ok(stream) => {
                let client_ip = stream.peer_addr()?.ip();
                println!("Connection received from: {}", client_ip);

                let listener = Arc::clone(&listener);
                thread::spawn(move || handle_client(stream, listener));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, _listener: Arc<Mutex<TcpListener>>) {
    let mut buffer = [0; 1024];

    // Read branch code
    if let Ok(bytes_read) = stream.read(&mut buffer) {
        let branch_code_message = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
        println!("Received branch code: {}", branch_code_message);

        let branch_code = branch_code_message.trim_start_matches("bcode~");
        let branch_dir = Path::new("data").join(branch_code);
        if !branch_dir.exists() {
            if let Err(e) = create_dir_all(&branch_dir) {
                eprintln!("Failed to create branch directory: {}", e);
                return;
            }
        }

        // Send "OK" acknowledgment for branch code
        if let Err(e) = stream.write_all(b"OK") {
            eprintln!("Failed to send acknowledgment for branch code: {}", e);
            return;
        }

        // Reading Base64 encoded file content
        if let Ok(bytes_read) = stream.read(&mut buffer) {
            let encoded_content = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            println!("Received (Base64): {}", encoded_content);

            // Check for ~ delimiters, then decode Base64 content
            if encoded_content.starts_with('~') && encoded_content.ends_with('~') {
                let trimmed_content = encoded_content.trim_matches('~');
                match lib::decode_from_base64(trimmed_content) {
                    Ok(decoded_content) => {
                        println!("Decoded content: {}", decoded_content);

                        let file_path = branch_dir.join("branch_weekly_sales.txt");
                        if let Err(e) = File::create(&file_path).and_then(|mut file| file.write_all(decoded_content.as_bytes())) {
                            eprintln!("Failed to write to file: {}", e);
                        }

                        // Send "OK" acknowledgment for file content
                        if let Err(e) = stream.write_all(b"OK") {
                            eprintln!("Failed to send acknowledgment for file content: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Failed to decode Base64 content: {}", e),
                }
            } else {
                eprintln!("Invalid message format; expected Base64 content wrapped in '~'.");
            }
        }
    }

    // closing  connection gracefully
    if let Err(e) = stream.shutdown(Shutdown::Both) {
        eprintln!("Warning: Failed to close the connection gracefully: {:?}", e);
    }
}