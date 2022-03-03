extern crate serde_json;
use serde_json::{Result,Value};
use std::sync::mpsc;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut data = Vec::<u8>::new(); // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
        handle_json(&mut data);
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            false
        }
    } {}
}




fn handle_tcp_listener(listener: &mut TcpListener) {
    //let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}



fn handle_json(data:&mut Vec<u8>)->Value{
    let m = String::from_utf8(data.clone()).unwrap();
    let v:Value = serde_json::from_str(m.as_str()).unwrap();
    v
}

// JSON SCHEMA 
// {
//      color:()
//      interp:0,1,2,3,..
//      data:[(x,y),(x,y)..]
//
//
//
// }
//
//
//
//
//

