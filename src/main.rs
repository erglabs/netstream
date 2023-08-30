/*
Copyright (C) 2023 ErgLabs <dev@erglabs.org>.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::{
    collections::HashMap,
    io::{self, Read, Write},
    str::from_utf8,
};

use anyhow::anyhow;
use mio::{
    event::Event,
    net::{TcpListener, TcpStream},
    Events,
    Interest,
    Poll,
    Registry,
    Token,
};
pub mod logger;
pub mod netframe;
pub mod netstream;
const SERVER: Token = Token(0);
const DATA: &[u8] = b"Hello world!\n";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init_subscriber().unwrap();
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    let addr = "127.0.0.1:6669".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;
    // Register the server with poll we can receive events for it.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;
    let mut connections = HashMap::new();
    let mut connection_tocket = Token(SERVER.0 + 1);
    println!("You can connect to the server using `nc`:");
    println!(" $ nc 127.0.0.1 6669");
    loop {
        poll.poll(&mut events, None)?;
        for event in events.iter() {
            match event.token() {
                SERVER => {
                    loop {
                        // Received an event for the TCP server socket, which
                        // indicates we can accept an connection.
                        let (mut connection, address) = match server.accept() {
                            Ok((connection, address)) => (connection, address),
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // If we get a `WouldBlock` error we know our
                                // listener has no more incoming connections queued,
                                // so we can return to polling and wait for some
                                // more.
                                break;
                            }
                            Err(e) => {
                                // If it was any other kind of error, something went
                                // wrong and we terminate with an error.
                                return Err(anyhow!(e));
                            }
                        };
                        println!("Accepted connection from: {}", address);
                        let token = next(&mut connection_tocket);
                        poll.registry().register(
                            &mut connection,
                            token,
                            Interest::READABLE.add(Interest::WRITABLE),
                        )?;
                        connections.insert(token, connection);
                    }
                }

                token => {
                    // Maybe received an event for a TCP connection.
                    let done = if let Some(connection) = connections.get_mut(&token) {
                        handle_connection_event(poll.registry(), connection, event).await?
                    } else {
                        // Sporadic events happen, we can safely ignore them.
                        false
                    };
                    if done {
                        if let Some(mut connection) = connections.remove(&token) {
                            poll.registry().deregister(&mut connection)?;
                        }
                    }
                }
            }
        }
    }
}
fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}
/// Returns `true` if the connection is done.
async fn handle_connection_event(
    registry: &Registry,
    connection: &mut TcpStream,
    event: &Event,
) -> io::Result<bool> {
    if event.is_writable() {
        tracing::info!("is_writable");
        // We can (maybe) write to the connection.
        loop {
            match connection.write(DATA) {
                // We want to write the entire `DATA` buffer in a single go. If we
                // write less we'll return a short write error (same as
                // `io::Write::write_all` does).
                Ok(n) if n < DATA.len() => return Err(io::ErrorKind::WriteZero.into()),

                Ok(_) => {
                    // After we've written something we'll reregister the connection
                    // to only respond to readable events.
                    registry.reregister(connection, event.token(), Interest::READABLE)?;
                    break;
                }

                // Would block "errors" are the OS's way of saying that the
                // connection is not actually ready to perform this I/O operation.
                Err(ref err) if would_block(err) => break,

                // Got interrupted (how rude!), we'll try again.
                Err(ref err) if interrupted(err) => continue,

                // Other errors we'll consider fatal.
                Err(err) => return Err(err),
            }
        }
    }
    if event.is_readable() {
        tracing::info!("is_readable");
        let mut connection_closed = false;
        let mut received_data = vec![0; 4096];
        let mut bytes_read = 0;
        // We can (maybe) read from the connection.
        loop {
            match connection.read(&mut received_data[bytes_read..]) {
                Ok(0) => {
                    // Reading 0 bytes means the other side has closed the
                    // connection or is done writing, then so are we.
                    connection_closed = true;
                    tracing::info!("read loop break, connection closed or done writing");
                    break;
                }

                Ok(n) => {
                    bytes_read += n;
                    if bytes_read == received_data.len() {
                        received_data.resize(received_data.len() + 1024, 0);
                    }
                    tracing::info!("bytes read... {}", bytes_read);
                }

                // Would block "errors" are the OS's way of saying that the
                // connection is not actually ready to perform this I/O operation.
                Err(ref err) if would_block(err) => break,

                Err(ref err) if interrupted(err) => continue,

                // Other errors we'll consider fatal.
                Err(err) => return Err(err),
            }
        }
        if bytes_read != 0 {
            let received_data = &received_data[..bytes_read];
            if let Ok(str_buf) = from_utf8(received_data) {
                println!("Received data: {}", str_buf.trim_end());
            } else {
                println!("Received (none UTF-8) data: {:?}", received_data);
            }
        }
        if connection_closed {
            println!("Connection closed");
            return Ok(true);
        }
    }
    Ok(false)
}
fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}
fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
