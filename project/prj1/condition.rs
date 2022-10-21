
fn test(){
    let needle = 3;
    let haystack = [1,2,3,4,5,6];
    let mut i = 0;
    while i < haystack.len() && haystack[i] <= needle {
        println!("i: {} {}", i, haystack[i]);
        i += 1;
    }
}

fn main(){
    let greater_than : bool = 5<10;
    println!("greater_than {}", greater_than);

    let mut i = 0;
    'outer: loop{
        'inner: loop{
            i+= 1;
            break 'inner;
        }
        break 'outer;
    }
    println!("i: {}", i);

    // loop
    for ind in 0..10{
        println!("index: {}", ind);
    }
    for e in & [ 1,2,3,4]{
        println!("e: {}", e);
    }

    let mut haystack = [4,6,3,2,8];
    println!("unsorted haystack: {:?}", haystack);

    for _ in 0..haystack.len() {
        let mut swaps = 0;
        for i in 1..haystack.len() {
            if haystack[i-1] > haystack[i] {
                let tmp = haystack[i];
                haystack[i] = haystack[i-1];
                haystack[i-1] = tmp;
                swaps += 1;
            }
        }
        println!("swapped times: {}", swaps);
    }
    println!("unsorted haystack: {:?}", haystack);

    let mut iindex = 0;
    while iindex <= 10 {
        iindex += 1;
    }
    println!("iindex: {}", iindex);

    test();

}