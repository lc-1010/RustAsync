use {
    futures::{
         future::{BoxFuture,FutureExt},
         task::{waker_ref,ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel,Receiver,SyncSender},
        sync::{Arc,Mutex},
        task::{Context,Poll},
        time::Duration,
    },
     time_futrue::TimerFuture,
};


//Our executor will work by sending tasks to run over a channel
// executor->pull 

/// task executor ,tasks off a channel and runs them
struct Executor{
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor{
    fn run(&self){
        while let Ok(task)= self.ready_queue.recv(){
            // 获取一个future 是some 则进行poll
            let mut future_slot = task.future.lock().unwrap();
            // task.future ==> Mutex<Option<BoxFuture<'static,()>>> 
            match future_slot.take() {
                Some(mut future) => {
                    // 创建一个 lockwaker
                    let waker = waker_ref(&task);
                    let context = &mut Context::from_waker(&*waker);
                    // Boxfutrue<T> 是// `BoxFuture<T>` is a type alias for
                    // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
                    // We can get a `Pin<&mut dyn Future + Send + 'static>
                    if future.as_mut().poll(context).is_pending(){
                        // 如果是pending 则放回去 继续poll
                        *future_slot = Some(future);
                    }
                }
                _ => (),
            }
        }
    }
}

// Spawner new futures onto the task channel
#[derive(Clone)]
struct Spawner{
    task_sender: SyncSender<Arc<Task>>,
}
impl Spawner{
    fn spawn(&self, future: impl Future<Output=()> + 'static +Send ){
        let future = future.boxed();
        let task = Arc::new(Task{
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued 满啦～ ");
    }
}

/// a future that can reschedule itself to be polled by an Executor
struct Task {
    future: Mutex<Option<BoxFuture<'static,()>>>,
    //pub type BoxFuture<'a, T> = Pin<alloc::boxed::Box<dyn Future<Output = T> + Send + 'a>>;
    // Pin<Box<dyn Future<Output = T> + Send + 'static>>
    task_sender: SyncSender<Arc<Task>>,
}
///在执行器 poll 一个 Future 之前，首先需要调用 wake 方法进行唤醒，然后再由
///  Waker 负责调度该任务并将其放入任务通道中。创建 Waker 的最简单的方式就是实现 ArcWake 特征，
/// 先来为我们的任务实现 ArcWake 特征，这样它们就能被转变成 Waker 然后被唤醒:
/// create a waker
impl ArcWake for Task{
    fn wake_by_ref(arc_self: &Arc<Self>){
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued 满啦");
    } 
}

fn new_executor_and_spawner()-> (Executor,Spawner){
    const MAX_QUEUE_TASKS: usize = 10_000;
    let (task_sender,ready_queue) = sync_channel(MAX_QUEUE_TASKS);
    (Executor{ready_queue},Spawner{task_sender})
}



fn main(){
    let(executor,spawner) = new_executor_and_spawner();
    spawner.spawn(async {
        println!("howdy!");
        TimerFuture::new(Duration::new(2,0));
        println!("done!");
    });
    
    spawner.spawn(sing());
    drop(spawner);
    executor.run();

    ///================
    let mut io_blokcer = IoBlocker::new();
    io_blocker.add_io_event_interest(&socket_1, Event{id:1,signals: READABLE,});

}

async fn sing(){
    println!("sing a song");
}

// struct IoBlocker{

// }
// struct Event {
//     id:usize,
//     signals: Signals,
// }

// impl IoBlocker{
//     fn new()->Self{}
//     fn add_io_event_interest{
//         &self,
//         io_object:&IoObject,
//         evnet:Event,
//     }{}
//     fn block(&self)->Event{}
// }
