
use std::collections::BTreeSet;
use std::collections::BTreeMap;

fn replace_index_with_5(data: &mut[u8], idx: usize){
    data[idx] = 5;
}
fn sort_in_place(data: &mut[u8]){
    data.sort();
}

fn main(){
    let my_array:[u8; 3] = [ 1 , 2, 3];

    println!("collection main()");
    // assert_eq!(vec![], Vec::new());

    // A pointer to the data and the capacity
    // pub struct RawVec<T, A:Allocator=Global>{
    //     ptr:Unique<T>,
    //     cap: usize,
    //     alloc: A,
    // }
    // The actual Vec, with the length,
    // pub struct Vec<T, A:Allocator=Global>{
    //     buf: RawVec<T,A>,
    //     len: usize,
    // }

    let mut animals = vec!["sheep", "unicorn", "snail"];
    animals[2] = "octupus";
    animals.push("fish");
    for ele in animals {
        println!("{}", ele);
    }

    let mut v = vec![1,2,3,4,5];
    replace_index_with_5(&mut v, 1);
    assert_eq!(v[1],5);
    sort_in_place(&mut v);
    assert_eq!(v, vec![1,3,4,5,5]);

    let max_buckets = 3;
    let mut naive_hashmap = vec![Vec::<String>::new();max_buckets];
    let data = "blog.x5ff.xyz".to_string();
    let hash = data.len();
    let hashmap_position=hash%max_buckets;
    naive_hashmap[hashmap_position].push(data.clone());

    let mut a: BTreeSet<usize> = vec![1,2,3].into_iter().collect();
    let mut b: BTreeSet<usize> = vec![2,3,4].into_iter().collect();

    a.insert(4);
    a.contains(&4);

    let mut animal_counter = BTreeMap::new();
    animal_counter.insert("a", 1_usize);
    println!("{}", animal_counter.get);
}
