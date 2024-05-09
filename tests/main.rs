use std::mem::size_of;

use nitro_rs::arm7::{PersonalData, SystemData, SystemDataRepresentation};

fn main() {
    use std::io::Read;
    println!("The struct is {} bytes in Rust.", size_of::<PersonalData>());
    let mut file = std::fs::File::open("example.obj").unwrap();
    let mut raw_obj = [0x00; 112];
    file.read(&mut raw_obj).unwrap();
    let obj: PersonalData = unsafe { std::mem::transmute_copy(&raw_obj) };
    let datum: SystemDataRepresentation = obj.sys_data.into();
    println!("{}", datum.settings_lost);
}
