use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

use futures::task::{waker_ref, ArcWake};
use futures::stream::StreamExt;

/// TimerFuture — Future yang selesai setelah delay
struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // simpan waker agar bisa di-"wake" dari thread
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

/// Task untuk menyimpan future dan channel pengirim
struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    task_sender: futures::channel::mpsc::Sender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.clone().try_send(cloned).unwrap();
    }
}

/// Spawner untuk mengirim task ke executor
struct Spawner {
    task_sender: futures::channel::mpsc::Sender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let future = Box::pin(future);
        let task = Arc::new(Task {
            future: Mutex::new(future),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.clone().try_send(task).unwrap();
    }
}

/// Executor untuk menjalankan semua task yang tersedia
struct Executor {
    task_receiver: futures::channel::mpsc::Receiver<Arc<Task>>,
}

impl Executor {
    async fn run(&mut self) {
        while let Some(task) = self.task_receiver.next().await {
            let mut future_slot = task.future.lock().unwrap();
            let waker = waker_ref(&task);
            let context = &mut Context::from_waker(&*waker);
            if let Poll::Pending = future_slot.as_mut().poll(context) {
                // Belum selesai
            }
        }
    }
}

fn main() {
    let (task_sender, task_receiver) = futures::channel::mpsc::channel(10);
    let spawner = Spawner { task_sender };
    let mut executor = Executor { task_receiver };

    // Spawn asynchronous task
    spawner.spawn(async {
        println!("Syarna's Computer: howdy!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("Syarna's Computer: done!");
    });

    // Tambahan untuk 1.2:
    println!("Syarna's Computer: Task has been spawned!");

    drop(spawner); // Channel ditutup

    futures::executor::block_on(executor.run());
}

