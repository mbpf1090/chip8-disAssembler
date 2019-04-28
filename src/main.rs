use std::fs::File;
use std::io::{Read, Error};

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
    let path = "pong.ch8";
    let rom = match read_rom(path) {
        Ok(rom) => rom,
        Err(error) => panic!("There was a problem opening the file: {:?}", error)
    };
    for chunk in rom.chunks(2) {
        if chunk[0] == 0 && chunk[1] == 0 {
            let opcode = "EMPTY";
            println!("{0:08b} {1:08b} | {0:02X}{1:02X} | {0:03} {1:03} || {2}", chunk[0], chunk[1], opcode);
            continue;
        }
        let nnn = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
        let n = chunk[1] & 0x0F;
        let x = chunk[0] & 0x0F;
        let y = chunk[1] >> 4;
        let kk = chunk[1];

        let opcode = match chunk[0] >> 4 {
            0x0 => match chunk[0] >> 3 {
                    0x0 => match chunk[1] & 0x0F {
                            0xE => String::from("RET"),
                            0x0 => String::from("CLS"),
                            _ => format!("ERROR {:02X} {:02X}", chunk[0], chunk[1])
                    },
                    _ => format!("{: <5} {:02X}", "SYS", nnn)
            },
            0x2 => format!("{: <5} {:2X}", "CALL", nnn),
            0x1 => format!("{: <5} {:<10X}", "JP", nnn),
            0x3 => format!("{: <5} {:02X} {:02X}", "SE", x, kk),
            0x4 => format!("{: <5} {:02X} {:02X}", "SNE", x, kk),
            0x5 => format!("{: <5} {:02X} {:02X}", "SE", x, y),
            0x6 => format!("{: <5} {:02X} {:02X}", "LD", x, kk),
            0x7 => format!("{: <5} {:02X} {:02X}", "ADD", x, kk),
            0x8 => match chunk[1] & 0x0F {
                    0x0 => format!("{: <5} {:02X} {:02X}", "LD", x, y),
                    0x1 => format!("{: <5} {:02X} {:02X}", "OR", x, y),
                    0x2 => format!("{: <5} {:02X} {:02X}", "AND", x, y),
                    0x3 => format!("{: <5} {:02X} {:02X}", "XOR", x, y),
                    0x4 => format!("{: <5} {:02X} {:02X}", "ADD", x, y),
                    0x5 => format!("{: <5} {:02X} {:02X}", "SUB", x, y),
                    0x6 => format!("{: <5} {:02X} {:02X}", "SHR", x, y),
                    0x7 => format!("{: <5} {:02X} {:02X}", "SUBN", x, y),
                    0xE => format!("{: <5} {:02X} {:02X}", "SHL", x, y),
                    _ => format!("0x8 ERROR {:02X} {:02X}", chunk[0], chunk[1])
            },
            0x9 => format!("{: <5} {:02X} {:02X}", "SNE", x, y),
            0xA => format!("{: <5} {:02X}", "LD I", nnn),
            0xB => format!("{: <5} {:02X}", "JP V0", nnn),
            0xC => format!("{: <5} {:02X} {:02X}", "RND", x, kk),
            0xD => format!("{: <5} {:02X} {:02X} {:02X}", "DRW", x, y, n),
            0xE => match chunk[1] {
                    0x9E => format!("{: <5} {:02X}", "SKP", x),
                    0xA1 => format!("{: <5} {:02X}", "SKNP", x,),
                    _ => format!("0xE Error {:02X} {:02X}", chunk[0], chunk[1])
            },
            0xF => match chunk[1] {
                    0x07 => format!("{: <5} {:02X}", "LD DT", x),
                    0x0A => format!("{: <5} {:02X}", "LD K", x),
                    0x15 => format!("{: <5} {:02X}", "LD DT", x),
                    0x18 => format!("{: <5} {:02X}", "LD ST", x),
                    0x1E => format!("{: <5} {:02X}", "ADD I", x),
                    0x29 => format!("{: <5} {:02X}", "LD F", x),
                    0x33 => format!("{: <5} {:02X}", "LD B", x),
                    0x55 => format!("{: <5} {:02X}", "LF I", x),
                    0x65 => format!("{: <5} {:02X}", "LD I", x),
                    _ => format!("0xF ERROR {:02X} {:02X}", chunk[0], chunk[1])
            },
            _ => String::from("none"),
        };
        println!("{0:08b} {1:08b} | {0:02X}{1:02X} | {0:03} {1:03} || {2}", chunk[0], chunk[1], opcode);
    }
}
