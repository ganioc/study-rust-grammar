use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;

fn inc_ten_times(counter: Arc<AtomicUsize>) -> 
    thread::JoinHandle<()> {
        thread::spawn(move || {
            for _ in 0..10{
                counter.fetch_add(1, Ordering::SeqCst);
            }
        })
    }


pub fn run_main_threading(){
    println!("main: {:?}", thread::current().id());
    let handle = thread::spawn(||{
        println!("spawned: {:?}", thread::current().id());
    });

    handle.join().unwrap();

    let message = "A message from the main thread".to_string();
    let handle2 = thread::spawn(move ||{
        println!("spawned: {:?}, with message: {}",
            thread::current().id(),
            message
        );
    });

    handle2.join().unwrap();

    let message2 = Arc::new("A message from arc".to_string());
    let message2_ = Arc::clone(&message2);
    let handle_arc = thread::spawn(move || {
        println!("spawned: {:?}, with message2: {}",
            thread::current().id(),
            message2_
    );
    });
    println!("{}", message2);
    handle_arc.join().unwrap();

    let counter = Arc::new(AtomicUsize::new(0));
    let handle11 = inc_ten_times(counter.clone());
    let handle12 = inc_ten_times(counter.clone());
    handle11.join().unwrap();
    handle12.join().unwrap();

    println!("counter is : {}", counter.load(Ordering::SeqCst));

    let (number_tx, number_rx) = mpsc::channel();
    let worker_thread = thread::spawn(move||{
        let mut total = 0;
        while let Ok(number) = number_rx.recv(){
            total += number;
            println!("received new number {}, the new total is {}",
                number,
            total);
        }
        println!("Channel closed, exiting.");
    });
    for i in 0..20{
        let tx = number_tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap()
        });
    }
    // close channel
    drop(number_tx);
    let _ = worker_thread.join();
    

}