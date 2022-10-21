mod PhysicalAccessControl;

use PhysicalAccessControl::Door;
use PhysicalAccessControl::Window;


use std::mem;

struct DoorDoor {
    sub_door: Door,
}

// mod PhysicalAccessControl {
//     pub struct Door{
//         pub width: u32,
//         pub height: u32,
//         pub is_open: bool,
//     }

//     impl Door {
//         pub fn new(width: u32, 
//             height: u32, 
//             is_open: bool) -> Self{
//             Door{
//                 width,
//                 height,
//                 is_open,
//             }
//         }
//     }
// }


fn four() -> i32{
    return 4;
}


fn main(){
    println!("result: {}", four());
    // let living_room_door = Door{
    //     width: 100,
    //     height: 200,
    //     is_open: false,
    // };
    let mut living_room_door = Door::new(100,200,false);
    living_room_door.open();
    assert!(living_room_door.is_open);

    println!("Size of a Door: {} bytes", 
        mem::size_of::<Door>());
    println!("Size of the members: {} bytes",
        mem::size_of::<(u32, u32,bool)> ());
    println!("Size of a DoorDoor: {} bytes",
        mem::size_of::<DoorDoor>());
    
    let my_door = PhysicalAccessControl::Door::new(100,200,false);


}