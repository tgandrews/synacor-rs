extern crate byteorder;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;
use byteorder::{LittleEndian, ReadBytesExt};

fn get_value(val:usize, memory:&Vec<u16>, registry: &[u16]) -> usize {
    let res = memory[val] as usize;

    if res >= 32768 {
        let reg_loc = res % 32768;
        registry[reg_loc] as usize
    } else {
        res
    }
}

fn main() {
    let path = Path::new("challenge/challenge.bin");
    let mut file = match File::open(&path) {
        Err(err) => panic!("couldn't open {}: {}", path.display(), err.description()),
        Ok(file) => file,
    };

    let mut buffer = [0u8; 2];
    let mut memory:Vec<u16> = Vec::new();
    let mut registry = [0u16; 8];
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
            // HALT
            break;
        } else if op == 1u16 {
            // SET
            pointer += 1;
            let reg_loc = (memory[pointer] % 32768) as usize;
            pointer += 1;
            let value = get_value(pointer, &memory, &registry);
            registry[reg_loc] = value as u16;
        } else if op == 6u16 {
            // JMP
            pointer += 1;
            pointer = get_value(pointer, &memory, &registry);
            continue;
        } else if op == 7u16 {
            // JF
            pointer += 1;
            let comp = get_value(pointer, &memory, &registry);
            pointer += 1;
            let jmp_loc = get_value(pointer, &memory, &registry);
            if comp != 0 {
                pointer = jmp_loc;
                continue;
            }
        } else if op == 8u16 {
            // JZ
            pointer += 1;
            let comp = get_value(pointer, &memory, &registry);
            pointer += 1;
            let jmp_loc = get_value(pointer, &memory, &registry);
            if comp == 0 {
                pointer = jmp_loc;
                continue;
            }
        } else if op == 21u16 {
            // NOOP
        } else if op == 19u16 {
            // PRINT
            pointer += 1;
            let value = get_value(pointer, &memory, &registry);
            let char_val = (value as u8) as char;
            print!("{}", char_val.to_string());
        }

        pointer += 1;
    }
}
