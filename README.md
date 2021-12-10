# File Server
This is a simple FTP server and client implementation.<br>
Client sends a struct: <br>
```
FileInfo {
    name: String           //  Represents the filename
    file_bytes: Vec<u8>    //  Represents the file contents  
}
```
<br>
To run the server:  cargo run server <br>
To run the client:  cargo run client <i>server's-port</i>
