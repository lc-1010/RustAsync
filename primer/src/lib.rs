use std::task::Waker;

use futures::{Future};

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake:fn())->Poll<Self::Output>;
}

enum Poll<T>{
    Ready(T),
    Pening,
}

pub struct Join<FutrueA,FutrueB>{
    a:Option<FutrueA>,
    b:Option<FutrueB>,
}

impl <FutrueA,FutrueB> SimpleFuture for Join<FutrueA,FutrueB>
where 
FutrueA:SimpleFuture<Output = ()>,
FutrueB:SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake:fn())->Poll<Self::Output>{
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake){
                self.a.take();
            }
        }
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake){
                self.b.take();
            }
        }
        if self.a.is_none() && self.b.is_none(){
            Poll::Ready(())
        }else{
            Poll::Pening
        }
    }
}

pub struct AndThenFut<FutrueA,FutrueB>{
    first:Option<FutrueA>,
    second: FutrueB,
}

impl <FutureA,FutrueB> SimpleFuture for AndThenFut<FutureA,FutrueB>
where 
FutureA:SimpleFuture<Output = ()>,
FutrueB:SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake:fn())->Poll<Self::Output>{
        if let Some(first) = &mut self.first {
            match first.poll(wake){
                Poll::Ready(()) => self.first.take(),
                Poll::Pening => return Poll::Pening,
            };
        }
        self.second.poll(wake)
    }
}
 
struct Socket {
    has_data_to_read:bool,
}
impl  Socket { 
    fn read_buf(&self)->Vec<u8> { 
        vec![]
      }
    fn set_readable_callback(&self,wake: Waker){
 

    }
}


pub struct SocketRead<'a>{
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead <'_>{
   
    type Output = Vec<u8>;
    fn poll(&mut self, wake:fn())->Poll<Self::Output> {
        if self.socket.has_data_to_read{
            Poll::Ready(self.socket.read_buf())
        }else{
            self.socket.set_readable_callback(wake);
            Poll::Pening
        }
    }
}
// trait MyFuture{
//     type Output;
//     fn poll(self:Pin<&mut Self> , cx:&mut Context<'_> )->Poll<Self::Output>;
// }

// struct MyFut{
//     a:i32,
//     ptr_to_a: *const i32
// }

// pub enum Poll<T> {
//     Ready(T),
//     Pending,
// }

// struct FutOne;
// struct FutTwo;
// struct AsyncFuture {
//     fut_one: FutOne,
//     fut_two:FutTwo,
//     state:State,
// }

// enum State{
//     AwaitingFutOne,
//     AwaitingFutTwo,
//     Done,
// }
// impl Future for AsyncFuture {
//     type Output = ();
//     fn poll()->Poll<()> {
        
//     }
// }

struct IoBlocker{
    event:Event,
}

struct Event{
    id:usize,
    signals:Signals,
}
enum Signals {
    READABLE,
    WRITABLE,
}

impl IoBlocker{
    fn new()->Self{
        IoBlocker { 
            event:Event {id:0,
            signals:Signals::WRITABLE,} 
         }
    }
    fn add_io_evnet_interest(&self, io_object:&IoBlocker,event: Event){
        
    }

    fn block(&self)->Event{
        Event { id: self.event.id, signals: Signals::WRITABLE }
    }
}

