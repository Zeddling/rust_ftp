use serde::{Deserialize};
use serde_json;
use std::io::{BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

const HOST: &str = "127.0.0.1";

/**
 * File metadata and data
 */
#[derive(Deserialize, Debug)]
struct FileInfo {
    name: String,
    file_bytes: Vec<u8>,
}

/**
 * Handles server instance data
 */
pub struct Server {
    host: String,
    port: String,
}

impl Server { 
    /**
     * Hanndles file receiving and saving from each client
     */
    fn handle_connection(stream: TcpStream) {
        let reader = BufReader::new(stream);

        let info: FileInfo = match serde_json::from_reader(reader) {
            Ok(info) => info,
            Err(e) => panic!("Couldn't read stream: {}", e)
        };

        //  write file to output path
        let mut path = String::from("output/");
        path.push_str(info.name.as_ref());
        match std::fs::write(path, info.file_bytes) {
            Ok(_) => println!("File has been created!"),
            Err(e) => println!("File could not be created: {}", e)
        }
        println!("Got {} file", info.name);
    }

    /**
     * Starts server and listens for connections
     */
    pub fn run(&self) {
        let url = format!(
            "{}:{}",
            self.host,
            self.port
        );
        
        let listener = TcpListener::bind(
           url 
        ).unwrap();

        println!("Server listening on port {}", self.port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        Self::handle_connection(stream);
                    });
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            };
        }

        drop(listener);
    }
}
/**
 * Iterates between ports 8000 - 9000 to search for
 * a free port.
 */
fn find_port() -> Option<u16> {
    (8000..9000).
        find(|port| {
            match TcpListener::bind((HOST, *port)) {
                Ok(_) => true,
                Err(_) => false,
            }
        })
}

/**
 * Creates a new Server instance
 */
pub fn new() -> Server {
    let port = find_port().expect("No available port");

    Server {
        host: String::from(HOST),
        port: port.to_string(),
    }
}
