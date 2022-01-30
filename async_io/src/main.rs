// Simple TCP server using the Mio async I/O library

use std::{
    collections::HashMap,
};

use mio::{
    Events,
    Interest,
    Poll,
    Token,
    net::{
        TcpListener,
        TcpStream
    }
};

const INBOUND: Token = Token(0);


fn main() -> std::io::Result<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    let mut listener = TcpListener::bind("0.0.0.0:8888".parse().unwrap())?;

    // Register the listener as pollable with the token INBOUND
    poll.registry()
        .register(&mut listener, INBOUND, Interest::READABLE)?;
    
    let mut connections: HashMap<Token, TcpStream> = HashMap::new();
    let mut id: usize = 1;

    loop {
        poll.poll(&mut events, None)?;

        // Process the events that the poller has received...
        for event in events.iter() {
            match event.token() {
                // Event was from the source registered with the INBOUND token
                INBOUND => {
                    // Accept the incoming connection
                    let (mut conn, addr) = listener.accept().expect("Failed to accept");

                    // Register the new connection to be pollable with a unique ID.
                    poll.registry()
                        .register(&mut conn, Token(id), Interest::READABLE)?;

                    // Add the connection into our hash map with the ID as our key.
                    connections.insert(Token(id), conn);

                    // Update the ID for the next connection
                    id += 1;

                },

                // Event was from another source
                token => {
                    println!("Got event from source ID={}", token.0);

                    // Extract the connection that gave the event
                    let mut conn = match connections.get_mut(&token) {
                        Some(x) => x,
                        None => continue
                    };

                    // Handle it
                    if event.is_readable() {
                        // We can read from the event
                    }
                    if event.is_writable() {
                        // We can write to the event
                    }
                }
            }
        }

        // After all events are processed, continue to do other tasks before polling again...
    }


    Ok(())
}