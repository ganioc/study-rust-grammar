// use 
use futures::executor;
use futures::executor::ThreadPool;
use futures::channel::mpsc;
use futures::StreamExt;
use std::sync::mpsc::{Sender, channel};
use std::slice;

unsafe fn unsafe_casting(
    sender_ptr: *mut Sender<u8>,
    data_ptr: *const u8,
    data_len: usize,
){
    let data: &[u8] = slice::from_raw_parts(data_ptr, data_len);
    let sender: Box<Sender<u8>> = Box::from_raw(sender_ptr);
    for d in data{
        let _ = sender.send(*d);
    }
    std::mem::drop(sender);
}

const TWO: usize = 2;
fn print_n<const N:usize>(){
    println!("n={}", N);
}

pub fn run_main_async(){
    let my_future = async{
        let x = 10;
        let double_task = async move {
            x*x

        };
        println!("task result: {}", double_task.await);
    };
    executor::block_on(my_future); // pull it until finished,

    // let pool = ThreadPool::new().expect("Failed to build pool");
    // let (tx, rx) = mpsc::unbounded::<i32>();
    // let fut_values = async{
    //     let fut_tx_result = async move{
    //         (0..10).for_each(|v|{
    //             tx.unbounded_send(v).expect("Failed to send");
    //         })
    //     };
    //     pool.spawn_ok(fut_tx_result);
    //     let fut_values = rx.map(|v|{ v*2 }).collect();
    //     fut_values.await

    // };
    // let values = executor::block_on(fut_values);
    // println!("Values = {:?}", values);

    print_n::<42>();

    // unsafe main
    let (tx, rx) = channel::<u8>();
    let data = vec![1,2,3,4];
    unsafe{
        let tx_ptr = Box::into_raw(Box::new(tx));
        let data_ptr = data.as_ptr();
        unsafe_casting(tx_ptr, data_ptr, data.len());
    }

    while let Ok(n) =  rx.recv() {
        println!("Received: {}", n);
    }


}