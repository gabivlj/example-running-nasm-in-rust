use std::ffi::CString;
use std::os::raw::c_char;
use std::time;

#[link(name = "add.a")]
extern "C" {
    fn factorial_extern(a: u64) -> u64;
    fn count_bits_extern(buff: *const c_char, length: u64) -> u64;
}

fn main() {
    little_endian_example();

    // Call asm factorial
    println!("add: {}", factorial(32));

    const REPEAT: usize = 100000000;
    let tb = time::Instant::now();
    println!("number of 1 bits: {}", count_bits(" ".repeat(REPEAT)));
    let elapsed_tb = tb.elapsed().as_millis();
    println!("assembly version time elapsed: {}ms", elapsed_tb);
    let tb = time::Instant::now();
    println!(
        "number of 1 bits rust version: {}",
        count_bits_r(" ".repeat(REPEAT))
    );
    let elapsed_tb = tb.elapsed().as_millis();
    println!("rust version time elapsed: {}ms", elapsed_tb);
}

/// Count number of 1 bits in the buffer
fn count_bits_r<T: Into<Vec<u8>>>(string: T) -> u64 {
    let buffer = string.into();
    let mut count: u64 = 0;
    for byte in &buffer {
        let mut n = *byte;
        while n != 0 {
            count += (n & 1) as u64;
            n >>= 1;
        }
    }

    count as u64
}

/// Count number of 1 bits in the buffer (asm version)
fn count_bits<T: Into<Vec<u8>>>(string: T) -> u64 {
    let buff = string.into();
    let size = buff.len();
    unsafe { count_bits_extern(CString::new(buff).unwrap().as_ptr(), size as u64) }
}

fn factorial(a: u64) -> u64 {
    unsafe { factorial_extern(a) }
}

fn little_endian_example() {
    // [1]
    let x = 0x1_u16;
    // get memory address as a u8 pointer
    let dir: *mut u8 = unsafe { std::mem::transmute(&x) };
    // get memory address as a u64 and add 1
    let mut next_dir: u64 = unsafe { std::mem::transmute(dir) };
    next_dir += 1;
    unsafe {
        // we would put in memory [..., dir = 1, next_dir = 2, ...]
        // would be in little endian = 0x0201
        std::ptr::write_volatile(std::mem::transmute(next_dir), 2 as u8);
    }
    assert_eq!(x, 0x0201);
}
