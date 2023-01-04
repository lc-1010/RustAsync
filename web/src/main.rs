//use std::net::TcpListener;
//use std::net::TcpStream;
// use std::io::Write;
// use std::io::Read;
use std::fs;
use std::time::Duration;
use async_std::task;
use async_std::task::spawn;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use futures::FutureExt;
use futures::stream::StreamExt;
use futures::future;
use futures::select;
use futures::stream::{Stream, StreamExt, FusedStream};

use async_std::prelude::*;

#[async_std::main]
async fn main() {
        let a = "hello";
        let s = String::from("hello");
        let z = s;
        let listener = TcpListener::bind("127.0.0.1:7989").await.unwrap();

        // for stream in listener.incoming(){
        //     let stream = stream.unwrap();
        //     handle_connection(stream).await;
        // }
        listener
        .incoming()
        .for_each_concurrent(None,|tcpstream| async move {
           let tcpstream = tcpstream.unwrap();
           spawn(handle_connection(tcpstream));
        })
        .await;
}



async fn handle_connection(mut stream:TcpStream) {
    let mut buffer = [0;1024];
     _ = stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line,filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else if buffer.starts_with(sleep){
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{status_line}{contents}");
    stream.write_all(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}


fn _select(){
     let mut a_fut = future::ready(3);    
    let mut b_fut = future::ready(4);
    let mut total =0;
    loop {
        select! {
            a = a_fut => total+=a,
            b = b_fut => total+=b,
            complete => break,
            default =>panic!(),
        }
    }
    assert_eq!(total,7);
}

async fn add_two_stream(
    mut s1: impl Stream<Item=u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item =u8> + FusedStream +Unpin,
)-> u8 {
    let mut total =0;
    loop {
        let item = select!{
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) =item{
            total += next_num;
        }
    }
    total
}

async fn get_new_num()->u8{ 5}
async fn run_on_new_num(_:u8){}

async fn run_loop(
    mut interval_timer:impl Stream<Item=()> + FusedStream + Unpin, starting_num:u8,
){
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(run_on_new_num_fut,get_new_num_fut);
}