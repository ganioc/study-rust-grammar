use std::marker::PhantomData;
use std::ops::Add;

struct Container(i32, i32);

trait Contains{
    type A;
    type B;

    fn contains(&self, _:&Self::A, _:&Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container{
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains> (container: &C) -> i32{
    container.last() - container.first()
}
// 允许这种类型进行相等的测试, equality test, 
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B>{ first: A, phaontom: PhantomData<B>}

// B 不会分配存储空间，不能参与运算,

#[derive(Debug, Clone, Copy)]
enum Inch{}

#[derive(Debug, Clone, Copy)]
enum Mm{}

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl <Unit> Add for Length<Unit>{
    type Output =  Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

pub fn run_associate(){
    println!("\nrun associate()");
    let number_1 = 12;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2)
    );

    println!("First number : {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is: {}", difference(&container));


    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1 : PhantomStruct<char, f32> = PhantomStruct { first: 'Q', phaontom: PhantomData };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct { first: 'Q', phaontom: PhantomData };

    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meter = one_meter + one_meter;

    println!("one foot + one foot = {:?} in", two_feet.0);
    println!("one meter + one meter = {:?} mm", two_meter.0);
    


}