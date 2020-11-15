use std::ffi::CString;
use std::os::raw::c_char;
use std::time;

#[link(name = "add.a")]
extern "C" {
    fn factorial_extern(a: u64) -> u64;
    fn count_bits_extern(buff: *const c_char, length: u64) -> u64;
}

fn main() {
    let x = 0x1_u16;
    // get memory direction as a u8 pointer
    let dir: *mut u8 = unsafe { std::mem::transmute(&x) };
    // get memory dir as a u64 and add 1
    let mut next_dir: u64 = unsafe { std::mem::transmute(dir) };
    next_dir += 1;
    unsafe {
        // Write in memory : [dir = 1, next_dir = 2] being in little endian : 0x201
        std::ptr::write_volatile(std::mem::transmute(next_dir), 2 as u8);
    }
    // expect 0x201
    println!("0x{:x}", x);
    // Call asm factorial
    println!("add: {}", factorial(32));

    const REPEAT: usize = 1000000;
    let tb = time::Instant::now();
    println!("number of 1 bits: {}", count_bits(" ".repeat(REPEAT)));
    let elapsed_tb = tb.elapsed().as_millis();
    println!("assembly version time elapsed: {}", elapsed_tb);
    let tb = time::Instant::now();
    println!(
        "number of 1 bits rust version: {}",
        count_bits_r(" ".repeat(REPEAT))
    );
    let elapsed_tb = tb.elapsed().as_millis();
    println!("rust version time elapsed: {}", elapsed_tb);
}

fn count_bits_r<T: Into<Vec<u8>>>(string: T) -> u64 {
    let buffer = string.into();
    let mut count: u64 = 0;
    for byte in buffer {
        let mut n = byte;
        while n != 0 {
            count += (n & 1) as u64;
            n >>= 1;
        }
    }
    count as u64
}

fn count_bits<T: Into<Vec<u8>>>(string: T) -> u64 {
    let buff = string.into();
    let size = buff.len();
    unsafe { count_bits_extern(CString::new(buff).unwrap().as_ptr(), size as u64) }
}

fn factorial(a: u64) -> u64 {
    unsafe { factorial_extern(a) }
}
