use crossbeam_channel::{bounded, Sender, Receiver};
use std::thread;

fn main() {
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = bounded(10);

    // Producer threads
    let producer1 = {
        let sender = sender.clone();
        thread::spawn(move || {
            for i in 0..5 {
                sender.send(i).unwrap();
                println!("Producer 1 sent {}", i);
            }
        })
    };

    let producer2 = thread::spawn(move || {
        for i in 5..10 {
            sender.send(i).unwrap();
            println!("Producer 2 sent {}", i);
        }
    });

    // Consumer threads
    let consumer1 = thread::spawn(move || {
        while let Ok(value) = receiver.recv() {
            println!("Consumer 1 received {}", value);
        }
    });

    let consumer2 = {
        let receiver = receiver.clone();
        thread::spawn(move || {
            while let Ok(value) = receiver.recv() {
                println!("Consumer 2 received {}", value);
            }
        })
    };

    producer1.join().unwrap();
    producer2.join().unwrap();

    // Since the sender is dropped, the consumers will exit their loops
    // once they receive all the messages.
    drop(sender);
    consumer1.join().unwrap();
    consumer2.join().unwrap();
}

