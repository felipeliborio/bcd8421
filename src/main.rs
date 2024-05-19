use rand::Rng;
use std::time::Instant;

fn main() {
    let input = input_generator();

    let mut index = 0;

    let mut counter: u64 = 0;
    let now = Instant::now();
    while index < input.len() - 6 {
        let _decoded = bcd8421_decode(&input[index..index + 6]);
        // println!("Output: {}", decoded);

        index += 6;

        let _encoded = bcd8421_encode(_decoded);
        // println!("Encoded: {:#04x?}", &encoded[4..10]);
        counter += _encoded[9] as u64;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Counter: {}", counter);
}

fn input_generator() -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let mut input = Vec::new();
    for _ in 0..6_000_000 {
        let lower = rng.gen_range(0..10);
        let upper = rng.gen_range(0..10);
        input.push(lower << 4 | upper);
    }

    input
}

fn bcd8421_decode(input: &[u8]) -> u64 {
    let mut output: u64 = 0;
    for i in 0..input.len() {
        output = 100 * output + 10 * (input[i] >> 4) as u64 + (input[i] & 0x0F) as u64;
    }
    output
}

fn bcd8421_encode(input: u64) -> [u8; 10] {
    let mut output = [0; 10];
    let mut input = input;

    let mut index = 9;
    while input > 0 {
        output[index] = (((input % 100) / 10) << 4) as u8 | (input % 10) as u8;
        input /= 100;
        index -= 1;
    }

    output
}
