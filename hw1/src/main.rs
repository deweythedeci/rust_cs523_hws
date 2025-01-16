const BITS_SIZE: usize = 8;
const DEFAULT_INIT: [bool; BITS_SIZE] = [true, false, true, false, false, true, false, false]; 
const ONE_CHAR: char = '*';
const ZERO_CHAR: char = '.';
const STEPS: u32 = 10;

fn calc_bit(bits: [bool; 3]) -> bool {
    let val: u8 = if bits[0] { 4 } else { 0 }
        + if bits[1] { 2 } else { 0 }
        + if bits[2] { 1 } else { 0 };
    match val {
        7 => false,
        6 => true,
        5 => true,
        4 => false,
        3 => true,
        2 => true,
        1 => true,
        0 => false,
        _ => unreachable!(),
    }
}

fn rule110(bits: [bool; BITS_SIZE]) -> [bool; BITS_SIZE] {
    let mut new_bits: [bool; BITS_SIZE] = [false; BITS_SIZE]; 
    for i in 0..BITS_SIZE {
        let adj_bits: [bool; 3] = [bits[(i + BITS_SIZE) % BITS_SIZE], bits[i], bits[(i + 1) % BITS_SIZE]];
        new_bits[i] = calc_bit(adj_bits);
    }
    new_bits
}

fn print_bits(bits: [bool; BITS_SIZE]) -> () {
    let mut line: String = "".to_string();
    for i in 0..BITS_SIZE {
        if bits[i] {
            line.push(ONE_CHAR);
        }
        else {
            line.push(ZERO_CHAR);
        }
    }
    println!("{}", line);
}

fn main() {
    let mut bits: [bool; BITS_SIZE] = DEFAULT_INIT;
    for _i in 0..STEPS {
        print_bits(bits);
        bits = rule110(bits);
    }
}