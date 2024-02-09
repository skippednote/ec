use std::{
    sync::{
        mpsc::{self, channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

struct Task {
    id: u32,
    payload: String,
}

fn create_task(id: u32, payload: String) -> Task {
    Task { id, payload }
}

struct Worker {
    id: u32,
}

impl Worker {
    fn process_task(&self, task: Task) -> String {
        thread::sleep(Duration::from_secs(1));
        format!(
            "Worker {} processed Task {} with payload: {}",
            self.id, task.id, task.payload
        )
    }
}

fn create_worker(id: u32) -> Worker {
    Worker { id }
}

fn main() {
    let (tx, rx): (Sender<Task>, Receiver<Task>) = channel();
    let rx = Arc::new(Mutex::new(rx));
    let (result_tx, result_rx): (Sender<String>, Receiver<String>) = channel();

    let total_tasks = 10;
    let total_workers = 5;
    let workers_per_tasks = total_tasks / total_workers;

    let tasks = Vec::from_iter(
        (1..=total_tasks).map(|i| create_task(i, format!("Task {}", i).to_string())),
    );
    let workers = Vec::from_iter((1..=total_workers).map(create_worker));

    let mut thread_handles = vec![];

    for worker in workers {
        let thread_rx = Arc::clone(&rx);
        let thread_result_tx = result_tx.clone();

        let handle = thread::spawn(move || {
            let mut counter = 0;
            while let Ok(task) = thread_rx.lock().unwrap().recv() {
                let result = worker.process_task(task);
                thread_result_tx.send(result).unwrap();
                counter += 1;
                if counter == workers_per_tasks {
                    break;
                }
            }
        });
        thread_handles.push(handle);
    }

    for task in tasks {
        let thread_tx = tx.clone();
        thread_tx.send(task).expect("Failed to send a task");
    }

    drop(tx);
    drop(result_tx);

    loop {
        match result_rx.recv_timeout(Duration::from_secs(10)) {
            Ok(result) => println!("Result => {}", result),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                println!("No more results within 10 second");
                break;
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("Results channel disconnected");
                break;
            }
        }
    }

    for handle in thread_handles {
        handle.join().expect("Failed to join the thread");
    }
}
