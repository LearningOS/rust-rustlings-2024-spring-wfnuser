// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

struct SimpleRNG {
    state: u64,
}

impl SimpleRNG {
    // 初始化生成器
    fn new(seed: u64) -> Self {
        SimpleRNG { state: seed }
    }

    // 生成下一个随机数
    fn next_int(&mut self) -> u64 {
        let a: u64 = 1103515245;
        let c: u64 = 12345;
        let m: u64 = 2u64.pow(31);
        self.state = (a * self.state + c) % m;
        self.state
    }

    // 生成一个指定范围的随机数
    fn rand_range(&mut self, min: u64, max: u64) -> u64 {
        min + self.next_int() % (max - min)
    }
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for i in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            let mut rng = SimpleRNG::new(i);  // 使用一个种子初始化生成器
            let sleep_time = rng.rand_range(500, 3000); // 生成一个介于500毫秒到3000毫秒之间的随机数
            thread::sleep(Duration::from_millis(sleep_time));
            // thread::sleep(Duration::from_millis(2500));
            // TODO: You must take an action before you update a shared value
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        let status = status.lock().unwrap();
        println!("jobs completed {}", status.jobs_completed);
    }
    let status = status.lock().unwrap();
    println!("jobs completed {}", status.jobs_completed);
}
