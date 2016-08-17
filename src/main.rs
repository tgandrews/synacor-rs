extern crate byteorder;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;
use byteorder::{LittleEndian, ReadBytesExt};


fn main() {
    let path = Path::new("challenge/challenge.bin");
    let mut file = match File::open(&path) {
        Err(err) => panic!("couldn't open {}: {}", path.display(), err.description()),
        Ok(file) => file,
    };

    let mut buffer = [0u8; 2];
    let mut memory:Vec<u16> = Vec::new();
    let mut count = file.read(&mut buffer).unwrap();
    while count > 0 {
        let val = Cursor::new(buffer).read_u16::<LittleEndian>().unwrap();
        memory.push(val);
        count = file.read(&mut buffer).unwrap();
    }

    let mut pointer = 0;

    loop {
        let op = memory[pointer];
        if op == 0u16 {
            break;
        } else if op == 21u16 {
            // NOOP
        } else if op == 19u16 {
            pointer += 1;
            let char_val = (memory[pointer] as u8) as char;
            print!("{}", char_val.to_string());
        }

        pointer += 1;
    }
}
