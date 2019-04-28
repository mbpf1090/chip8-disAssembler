use std::fs::File;
use std::io::{Read, Error};
use std::env;

mod opcode;

fn read_rom(file_path: &str) -> Result<Vec<u8>, Error> {
    let mut f = File::open(file_path)?;
    let mut buff = vec!();
    let result = f.read_to_end(&mut buff)?;
    Ok(buff)
}

//nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
//n or nibble - A 4-bit value, the lowest 4 bits of the instruction
//x - A 4-bit value, the lower 4 bits of the high byte of the instruction
//y - A 4-bit value, the upper 4 bits of the low byte of the instruction
//kk or byte - An 8-bit value, the lowest 8 bits of the instruction
//  HI      LO
//  hi  lo  hi  lo
//      n   n   n
//              n
//      x
//          y
//          k   k


fn main() {
    let mut args = env::args();
    args.next();

    let path = match args.next() {
        Some(path) => path,
        None => panic!("Please provide a valid path to a CHIP-8 rom!")
     };

    let rom = match read_rom(&path) {
        Ok(rom) => rom,
        Err(error) => panic!("There was a problem opening the file: {:?}", error)
    };
    for chunk in rom.chunks(2) {
        let opcode = opcode::get_opcode(chunk);
        println!("{0:08b} {1:08b} | {0:02X}{1:02X} | {0:03} {1:03} || {2}", chunk[0], chunk[1], opcode);
    }
}
