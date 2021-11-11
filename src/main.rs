use std::net::{TcpStream, TcpListener}; 
use std::thread; 
use std::io::{Read, Write, Error}; 


//  Handles a single client 

fn client_handler(mut stream: TcpStream) -> Result<(), Error> { 
    //  Printing the the remote endpoint address and port, and we define a buffer to hold data temporarily 
    println!("Incoming connection from: {}", stream.peer_addr()?); 

    let mut buf = [0; 512];
    
    //  A loop to read the data incoming from the stream 
    //  .read will return the length of the data it has read 
    loop {
        let bytes_read = stream.read(&mut buf)?;

        //  If .read will return 0, if it has reached the end of the TCP Stream  => We break out of the loop
        if bytes_read == 0 { 
            return Ok(()); 
        }
        //  We write the data back to the stream using the slice syntax
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    //  Listen to incoming connections from new clients. Address and Port is set to a well-known port 
    //  We call bind on the local ip and port pair to create a local lstening socket. I chose "8888" 
    //  -> any client that can reach a network connected to this host will be able to talk to the host
    
    let listener = TcpListener::bind("0.0.0.0:8888")
        .expect("Could not bind to this client"); 

    //  The incoming method on listener returns an iterator over streams that have connected to the server 
    //  We check if any of those encountered an error, if so, we match it with a particular error message and move on to the next client 
    for stream in listener.incoming() { 
        match stream { 

            //  if successful, we spawn a worker thread to handle each client connection
            Ok(stream) => { 
                //  We use a move closure: to send the stream variable to the client0-handler function 
                thread::spawn(move || { 
                    client_handler(stream)
                    .unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
            Err(e) => {
                eprintln!("Failed to listen: {}", e)
            }
        }
    }
}
