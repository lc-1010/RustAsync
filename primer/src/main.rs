#![feature(is_some_with)]
use core::{time};
use std::thread::sleep;
use std::str::FromStr;

use futures::executor::block_on;//阻塞当前线程
#[derive(Debug)]
struct Song{}

async fn learn_song(t:u64)->Song{
     sleep(time::Duration::from_secs(t));
    Song{}
}
async fn sing_song(song:Song){
    println!("song{:?}",song);
}
async fn dance(i:u8){
    println!("dance--------{}",i);
}

fn block_song(){
   let song =  block_on(learn_song(1));
   block_on(sing_song(song));
}

async fn lean_and_sing(sleep:u64){
    let song = learn_song(sleep).await;
    sing_song(song).await;
}
async fn async_main(){
    let sleep_time= 4;
    let start =  std::time::Instant::now();
    println!("lean_and_sing will sleep {}sec,time:{:?}",sleep_time, start);
    let f1 = lean_and_sing(sleep_time);
    let f2 = dance(1);
    futures::join!(f1,f2);
    println!("{:?}", start.duration_since(start));


}


fn main() {

    // let mut numbers = Vec::new();
    // for arg in std::env::args().skip(2){
    //     numbers.push(u64::from_str(&arg) 
    //                    .expect("error parsing argument"));
    // }

    // if numbers.len() == 0{
    //     eprintln!("Usage: gcd Number ...");
    //     std::process::exit(1);
    // }
    // let mut d = numbers[0];
    // for m in &numbers[1..]{
    //     d = gcd(d, *m);
    // }
    // println!("the greatest common divisor of {:?} is {}",numbers,d);


    let future = hello_world();
    block_on(async_main()); 
    block_on(future);
    println!("10 %3 {}",gcd(10,3));
    println!("=========start block on song======");
    
    block_on(dance(3));
    block_song();
    
    
    let optional = Some(3);
    match optional{
        Some(i)=>{
            println!("this is a really{:?}",i)
                },
        _=>{},
    };
    if let Some(3) = optional {
        println!("three");
    }
    if let Some(i) = optional{
        println!("this is a long int{:}",i);
    }
    
    let a = if  optional.is_some_and(|&x|x>30) {
        10
    }else{
        23
    };
    println!("now a assert eq {}",a );
    

    let mut woptional = Some(0);
    loop{
        match woptional{
            Some(i)=>{
                if i>9 {
                    println!("Greater than 9,quit!");
                    woptional = None;
                }else{
                    println!("i is {:?} try again",i);
                    woptional = Some(i+1);
                }
            },
            _ => {break;}
        };
    }
    println!("i is {:?} ==None",woptional);
    if woptional.is_none(){
        woptional = Some(9);
    }
    
    while let Some(i) = woptional{
        if i>19 {
            println!("Greater than 19,quit!");
            woptional = None;
        }else{
            println!("i is {:?} try again",i);
            woptional = Some(i+1);
        }
    }

    
}

#[allow(unused)]
async fn hello_world(){
    println!("hello world!");
}
//. awit 异步等待


fn gcd(mut n:u64,mut m: u64)-> u64{
    assert!(n!=0 && m!= 0);
    while m != 0 {
        if m<n {
            let t = m;
            m=n;
            n=t;
        }
        m = m %n;
    }
    n
}

// trait SimpleFuter{
//     type Output;
//     fn poll(&mut self, wake:fn())->Poll<Self::Output>;
// }
// enum Poll<T>{
//     Read(T),
//     Pening,
// }
