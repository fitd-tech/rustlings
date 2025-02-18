// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

/*
    Hint
    `Arc` is an Atomic Reference Counted pointer that allows safe, shared access
    to **immutable** data. But we want to *change* the number of `jobs_done` so
    we'll need to also use another type that will only allow one thread to mutate
    the data at a time. Take a look at this section of the book:
    https://doc.rust-lang.org/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct

    Keep reading if you'd like more hints :)

    Do you now have an `Arc<Mutex<JobStatus>>` at the beginning of `main`? Like:
    ```
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));
    ```

    Similar to the code in the following example in The Book:
    https://doc.rust-lang.org/book/ch16-03-shared-state.html#sharing-a-mutext-between-multiple-threads
*/

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            status_shared.lock().unwrap().jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
