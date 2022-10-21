
pub struct Door{
    pub width: u32,
    pub height: u32,
    pub is_open: bool   
}
impl Door {
    pub fn new(width:u32, height:u32,is_open:bool) -> Self{
        Door{
            width,
            height,
            is_open,
        }
    }
    pub fn open(& mut self){
        self.is_open = true;
    }
    pub fn close(& mut self){
        self.is_open = false;
    }
}

// private to this module
pub struct Window{
    width: u32,
    height: u32
}


