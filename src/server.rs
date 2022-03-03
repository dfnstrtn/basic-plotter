use crate::common::*;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::io::{BufRead,BufReader};
use std::sync::mpsc;




fn handle_client(mut stream: TcpStream,tx:mpsc::Sender<DataFormat>){
    let mut reader = BufReader::new(&stream);
    let mut strdata = String::new();
    while match reader.read_line(&mut strdata){
        Ok(size) =>{
            if size!=0{
                let z:DataFormat = serde_json::from_str(&strdata).unwrap();
                println!("z={:?}",z);
                tx.send(z);
                strdata.clear();
                true
            }else{
                false
            }
        }

        Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                false
        }
    }{} 
    stream.shutdown(Shutdown::Both).unwrap();
}



pub fn handle_connections(tx:mpsc::Sender<DataFormat>,addr:&str){
    let listener = TcpListener::bind(addr).unwrap();
    println!("listening at {:?}",addr);
    for stream in listener.incoming(){
        match stream{
            Ok(stream)=>{
                let cx = tx.clone();
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream,cx);
                });
            }
            Err(e)=>{
                println!("Unexpected error :{}",e);
            }
        }
    }
    drop(listener);
}




// useless function just to remember my formats 
pub fn testy_main(){
    let (tx,rx) = mpsc::channel::<DataFormat>();
    let tx = tx.clone();
    thread::spawn(move||{
        handle_connections(tx,"127.0.0.1:3333");
    });
    loop{
        match rx.try_recv(){
            Ok(a)=>{println!("OK {:?}",a);}
            Err(_)=>{}
        };
    }
}





