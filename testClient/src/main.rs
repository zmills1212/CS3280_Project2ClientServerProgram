use std::fs;
use std::io::{self, Read, Write};
use std::net::{TcpStream, Shutdown};
use std::str;

mod lib;

fn main() {
    let data_directory = "data"; // Directory containing branch folders

    // Iterate over each branch directory and send its data
    if let Err(e) = start_data_transfer("127.0.0.1:8080", data_directory) {
        eprintln!("Error during branch data transmission: {}", e);
    }
}

/// Function to send all branch data to the server, file by file
fn start_data_transfer(server_address: &str, data_directory: &str) -> io::Result<()> {
    for entry in fs::read_dir(data_directory)? {
        let entry = entry?;
        let branch_path = entry.path();

        // Check if the entry is a directory
        if branch_path.is_dir() {
            let branch_code = entry.file_name().into_string().unwrap_or_default();
            let file_path = branch_path.join("branch_weekly_sales.txt");

            // Verify that the branch file exists
            if file_path.exists() {
                println!("Sending data for branch: {}", branch_code);

                // Attempt to send this branch's data
                match send_branch_data(server_address, &branch_code, file_path.to_str().unwrap()) {
                    Ok(_) => println!("Data sent successfully for branch: {}", branch_code),
                    Err(e) => eprintln!("Failed to send data for branch {}: {}", branch_code, e),
                }
            } else {
                eprintln!("File not found for branch: {}", branch_code);
            }
        }
    }

    Ok(())
}

/// Function to handle data transmission for a single branch
fn send_branch_data(server_address: &str, branch_code: &str, file_path: &str) -> io::Result<()> {
    // Establish a connection to the server
    let mut stream = TcpStream::connect(server_address)?;
    println!("Connected to server for branch: {}", branch_code);

    //  Send the branch code to the server
    let branch_code_message = format!("bcode~{}", branch_code);
    stream.write_all(branch_code_message.as_bytes())?;
    stream.flush()?;
    println!("Sent branch code: {}", branch_code);

    //  Wait for "OK" response from the server
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let response = str::from_utf8(&buffer[..bytes_read])
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?  // Convert Utf8Error to io::Error
        .trim();
    if response != "OK" {
        return Err(io::Error::new(io::ErrorKind::Other, format!("Unexpected server response: {}", response)));
    }
    println!("Received '{}' confirmation for branch code", response);

    // Encode and send the file content
    let encoded_content = lib::encode_to_base64(file_path)?;
    let file_message = format!("~{}~", encoded_content);
    stream.write_all(file_message.as_bytes())?;
    stream.flush()?;
    println!("Sent encoded file content for branch: {}", branch_code);

    //  Wait for "OK" response for the file content
    let bytes_read = stream.read(&mut buffer)?;
    let response = str::from_utf8(&buffer[..bytes_read])
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?  // Convert Utf8Error to io::Error
        .trim();
    if response != "OK" {
        return Err(io::Error::new(io::ErrorKind::Other, format!("File transfer failed: {}", response)));
    }
    println!("File transfer confirmed for branch: {}", branch_code);

    // Close the connection after completing the transmission
    stream.shutdown(Shutdown::Both)?;

    Ok(())
}
