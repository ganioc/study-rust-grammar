// use 
use futures::executor;
use futures::executor::ThreadPool;
use futures::channel::mpsc;
use futures::StreamExt;

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

}