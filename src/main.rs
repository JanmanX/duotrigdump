use std::env;
use std::io::Read;

static ENCODING_TABLE: [char; 32] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 
    'Y', 'Z', '2', '3', '4', '5', '6', '7',
];

fn translate_chunk(chars: &mut Vec<char>, chunk: &mut Vec<u8>, n: usize) 
{
    for i in 1..=n {
        match i {
            1 => chars.push(ENCODING_TABLE[(chunk[0] >> 3) as usize]),
            2 => chars.push(ENCODING_TABLE[(((chunk[0] & 0b00000111) << 2) | (chunk[1] >> 6)) as usize]),
            3 => chars.push(ENCODING_TABLE[((chunk[1] >> 1) & 0b00011111) as usize]),
            4 => chars.push(ENCODING_TABLE[(((chunk[1] & 0b00000001) << 4) | (chunk[2] >> 4)) as usize]),
            5 => chars.push(ENCODING_TABLE[(((chunk[2] & 0b00001111) << 1) | (chunk[3] >> 7)) as usize]),
            6 => chars.push(ENCODING_TABLE[((chunk[3] >> 2) & 0b00011111) as usize]),
            7 => chars.push(ENCODING_TABLE[(((chunk[3] & 0b00000011) << 3) | (chunk[4] >> 5)) as usize]),
            8 => chars.push(ENCODING_TABLE[(chunk[4] & 0b00011111) as usize]),
            _ => panic!("Invalid value for n. Must be between 1 and 8."),
        }
    }

    // Pad the rest with "="
    for _ in n..8 {
        chars.push('=');
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut file = std::fs::File::open(file_path).unwrap();
    let byte_chunk_size = 5;


    loop {
        let mut chunk = Vec::with_capacity(8);
        let n = file.by_ref().take(byte_chunk_size as u64).read_to_end(&mut chunk).expect("Error reading file");

        // Pad chunk with 0s
        for _ in 0..(byte_chunk_size - n) {
            chunk.push(0);
        }

        let mut chars: Vec<char> = Vec::with_capacity(8);

        // Calculate number of base32 characters to print
        let b32_chars = (n as f64 / 5.0 * 8.0).ceil() as usize;

        translate_chunk(&mut chars, &mut chunk, b32_chars);

        print!("{} ", chars.iter().collect::<String>());

        if n < byte_chunk_size {
            break;
        }
    }

    println!();
}
