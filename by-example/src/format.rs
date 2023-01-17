use std::fmt;


#[derive(Debug)]
pub struct UnPrintable(i32);

#[derive(Debug)]
pub struct Structure(pub i32);

impl fmt::Display for Structure{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Deep(pub Structure);

#[derive(Debug)]
pub struct Person<'a>{
    pub name: &'a str,
    pub age: u8,
}

#[derive(Debug)]
pub struct MinMax(pub i64, pub i64);
impl fmt::Display for MinMax {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f, "[{}, {}]", self.0, self.1)
    }
}

pub struct MyList(pub Vec<i32>);
impl fmt::Display for MyList{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        let vec = &self.0;

        write!(f, "[");

        for (count , v) in vec.iter().enumerate(){
            if count !=0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

pub fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (integer, boolean) = pair;

    (boolean, integer)
}