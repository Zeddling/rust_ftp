use serde::{Serialize};
use serde_json;
use std::io::{BufWriter};
use std::net::{TcpStream};

/**
 * File metadata and data
 */
#[derive(Serialize, Debug)]
struct FileInfo {
    name: String,
    file_bytes: Vec<u8>,
}

/**
 * Connects to the server and handles the connection
 */
pub fn run(server: String) {
    match TcpStream::connect(server) {
        Ok(mut stream) => {
            println!("Successfully connected to {}", stream.peer_addr().unwrap());
            send_file(&mut stream);
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Terminating...")
}

fn send_file(stream: &mut TcpStream) {
    println!("Beginning ops");
    //  Get path of the file from user
    println!("Enter path of the file you wish to send: ");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    println!("Sending {} to server", input);

    //  Remove newline
    let path = input.replace("\n", "");

    //  load file
    let file = match std::fs::read(path.clone()) {
        Ok(file) => {
            let name = String::from(
                std::path::Path::new(&path).file_name().unwrap().to_str().unwrap()
            );

            FileInfo {
                name: name,
                file_bytes: file
            }
        },
        Err(e) => {
            panic!("Could not open file with error: {}", e)
        }
    };
    let writer = BufWriter::new(stream.try_clone().unwrap());

    //  Serialize
    match serde_json::to_writer(writer, &file) {
        Ok(_) => println!("File sent successfully!"),
        Err(e) => panic!("Error while sending file: {}", e)
    };
    

}
