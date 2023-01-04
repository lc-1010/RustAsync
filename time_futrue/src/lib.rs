use std::{
    future::Future,
    pin::Pin,
    sync::{Arc,Mutex},
    task::{Context,Poll,Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}
//Arc<Mutex<..>> to communicate between the thread and the Future

struct SharedState {
    //timer tricker elapsed 
    completed: bool,
    // the waker for the task is runing on
    waker:Option<Waker>,
}

impl Future for TimerFuture{
    type Output = ();
    fn poll(self:Pin<&mut Self>,cx:&mut Context<'_>)->Poll<Self::Output>{
        let mut shard_state = self.shared_state.lock().unwrap();
        if shard_state.completed {
            Poll::Ready(())
        }else{
            shard_state.waker = Some(cx.waker().clone());
            //we clone the Waker for the current task and pass it to shared_state.
            //waker so that the thread can wake the task back up.
            Poll::Pending
        }
    }
}

//定时器
impl TimerFuture{
    pub fn new (duration: Duration)->Self{
        let shared_state = Arc::new(Mutex::new(SharedState{
            completed:false,
            waker :None,
        }));
        let thread_shared_state = shared_state.clone();
        thread::spawn(move||{
             thread::sleep(duration);
             let mut shared_state = thread_shared_state.lock().unwrap();
             shared_state.completed = true;
             if let Some(waker) = shared_state.waker.take(){
                waker.wake()
             }
        }); 
        TimerFuture { shared_state }
    }
}