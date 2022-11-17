use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::{Arc};


pub fn concurr_run(){
    let v = vec![1,2,3,4];

    println!("concurr_run()");
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread! {:?}", i, v);
            thread::sleep(Duration::from_millis(2));
        }
    });

    handle.join().unwrap();

    for i in 1..5{
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(10));
    }

    let (tx, rx) = mpsc::channel();

    let _handle_channel = thread::spawn(move ||{
        let val = String::from("hi");
        tx.send(val).unwrap();

        let vals = vec![String::from("go"),
            String::from("from"),
            String::from("me")];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // println!("val is {}", val);
    });

    // handle_channel.join().unwrap();

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
    for received in rx {
        println!("Got: {}", received);
    }

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());

}