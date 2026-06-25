use std::{sync::mpsc, thread, time::Duration};

// multithreading + message passing
fn main() {
    let (tx, rx) = mpsc::channel();
    let mut total_sum: u64 = 0;
    for count in 0..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut sum = 0;
            let start = 100000000 * count;
            let end = 100000000 * (count + 1);
            for i in start..end {
                sum = sum + i;
            }
            producer.send(sum).unwrap();
        });
    }
    drop(tx);
    for received in rx {
        total_sum = total_sum + received;
        println!("found value {}",received)
    }
    println!("sum is {}", total_sum);
}
