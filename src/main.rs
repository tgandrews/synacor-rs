extern crate byteorder;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Cursor};
use std::path::Path;
use byteorder::{LittleEndian, ReadBytesExt};
use std::num::Wrapping;

fn get_value(val:usize, memory:&Vec<u16>, registry: &[u16]) -> u16 {
    let res = memory[val];

    if res >= 32768u16 {
        let reg_loc = (res % 32768) as usize;
        registry[reg_loc]
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
    let mut stack:Vec<u16> = Vec::new();
    let mut input_buffer:Vec<char> = Vec::new();
    let mut input_buffer_pos = 0;

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
        } else if op == 2u16 {
            // PUSH
            pointer += 1;
            let val = get_value(pointer, &memory, &registry);
            stack.push(val);
        } else if op == 3u16 {
            // POP
            pointer += 1;
            let reg_loc = (memory[pointer] % 32768) as usize;
            registry[reg_loc] = stack.pop().unwrap();
        } else if op == 4u16 {
            // EQ
            pointer += 1;
            let reg_loc = (memory[pointer] % 32768) as usize;
            pointer += 1;
            let comp1 = get_value(pointer, &memory, &registry);
            pointer += 1;
            let comp2 = get_value(pointer, &memory, &registry);
            let res = if comp1 == comp2 {
                1
            } else {
                0
            };
            registry[reg_loc] = res;
        } else if op == 5u16 {
            // GT
            pointer += 1;
            let reg_loc = (memory[pointer] % 32768) as usize;
            pointer += 1;
            let comp1 = get_value(pointer, &memory, &registry);
            pointer += 1;
            let comp2 = get_value(pointer, &memory, &registry);
            let res = if comp1 > comp2 {
                1
            } else {
                0
            };
            registry[reg_loc] = res;
        } else if op == 6u16 {
            // JMP
            pointer += 1;
            pointer = get_value(pointer, &memory, &registry) as usize;
            continue;
        } else if op == 7u16 {
            // JF
            pointer += 1;
            let comp = get_value(pointer, &memory, &registry);
            pointer += 1;
            let jmp_loc = get_value(pointer, &memory, &registry)  as usize;
            if comp != 0 {
                pointer = jmp_loc;
                continue;
            }
        } else if op == 8u16 {
            // JZ
            pointer += 1;
            let comp = get_value(pointer, &memory, &registry);
            pointer += 1;
            let jmp_loc = get_value(pointer, &memory, &registry) as usize;
            if comp == 0 {
                pointer = jmp_loc;
                continue;
            }
        } else if op == 9u16 {
            // ADD
            pointer += 1;
            let reg_loc = (memory[pointer] % 32768) as usize;
            pointer += 1;
            let op1 = get_value(pointer, &memory, &registry);
            pointer += 1;
            let op2 = get_value(pointer, &memory, &registry);
            let result = (op1 + op2) % 32768;
            registry[reg_loc] = result;
        } else if op == 10u16 {
            // MUL
            pointer += 1;
            let reg_loc = (memory[pointer] % 32768) as usize;
            pointer += 1;
            let op1 = get_value(pointer, &memory, &registry);
            pointer += 1;
            let op2 = get_value(pointer, &memory, &registry);
            let result = (Wrapping(op1) * Wrapping(op2)) % Wrapping(32768u16);
            registry[reg_loc] = result.0;
        } else if op == 11u16 {
            pointer += 1;
            let reg_loc = (memory[pointer] % 32768) as usize;
            pointer += 1;
            let op1 = get_value(pointer, &memory, &registry);
            pointer += 1;
            let op2 = get_value(pointer, &memory, &registry);
            let result = op1 % op2;
            registry[reg_loc] = result;
        } else if op == 12u16 {
            // AND
            pointer += 1;
            let reg_loc = (memory[pointer] % 32768) as usize;
            pointer += 1;
            let op1 = get_value(pointer, &memory, &registry);
            pointer += 1;
            let op2 = get_value(pointer, &memory, &registry);
            let result = op1 & op2;
            registry[reg_loc] = result;
        } else if op == 13u16 {
            // OR
            pointer += 1;
            let reg_loc = (memory[pointer] % 32768) as usize;
            pointer += 1;
            let op1 = get_value(pointer, &memory, &registry);
            pointer += 1;
            let op2 = get_value(pointer, &memory, &registry);
            let result = op1 | op2;
            registry[reg_loc] = result;
        } else if op == 14u16 {
            // NOT
            pointer += 1;
            let reg_loc = (memory[pointer] % 32768) as usize;
            pointer += 1;
            let op = get_value(pointer, &memory, &registry);
            let result = !op & 32767;
            registry[reg_loc] = result;
        } else if op == 15u16 {
            // READ MEM
            pointer += 1;
            let reg_loc = (memory[pointer] % 32768) as usize;
            pointer += 1;
            let mem_loc = get_value(pointer, &memory, &registry) as usize;
            registry[reg_loc] = memory[mem_loc];
        } else if op == 16u16 {
            // WRITE MEM
            pointer += 1;
            let write_location = get_value(pointer, &memory, &registry) as usize;
            pointer += 1;
            let value = get_value(pointer, &memory, &registry);
            memory[write_location] = value;
        } else if op == 17u16 {
            // CALL
            pointer += 1;
            let new_loc = get_value(pointer, &memory, &registry) as usize;
            let next_pointer = (pointer + 1) as u16;
            stack.push(next_pointer);
            pointer = new_loc;
            continue;
        } else if op == 18u16 {
            // RET
            let new_loc = match stack.pop() {
                Some(loc) => loc,
                None => break
            };
            pointer = new_loc as usize;
            continue;
        } else if op == 19u16 {
            // PRINT
            pointer += 1;
            let value = get_value(pointer, &memory, &registry);
            let char_val = (value as u8) as char;
            print!("{}", char_val.to_string());
        } else if op == 20u16 {
            // READ IN
            pointer += 1;
            let reg_loc = (memory[pointer] % 32768) as usize;

            if input_buffer.len() == 0 {
                let mut line = String::new();
                let stdin = io::stdin();
                stdin.lock().read_line(&mut line);
                input_buffer = line.chars().collect::<Vec<char>>();
            }

            let character = input_buffer[input_buffer_pos] as u16;
            input_buffer_pos += 1;

            if character == 10u16 {
                input_buffer = Vec::new();
                input_buffer_pos = 0;
            }
            registry[reg_loc] = character;

        } else if op == 21u16 {
            // NOOP
        } else {
            panic!("Unknown op: {}", op)
        }

        pointer += 1;
    }
}
