use std::ffi::CString;
use std::os::raw::c_char;
use std::time;

#[link(name = "add.a")]
extern "C" {
    fn factorial_extern(a: u64) -> u64;
    fn count_bits_extern(buff: *const c_char, length: u64) -> u64;
    fn switch() -> u64;
    fn count_max_freq(buff: *const c_char, ascii: *mut c_char) -> u64;
}

fn main() {
    little_endian_example();
    unsafe {
        println!("result {}", switch());
    }
    let tb = time::Instant::now();
    println!("add: {}", factorial_r(14));
    let elapsed_tb = tb.elapsed().as_nanos();
    println!("rust version time elapsed: {}ns", elapsed_tb);

    // Call asm factorial
    let tb = time::Instant::now();
    println!("add: {}", factorial(14));
    let elapsed_tb = tb.elapsed().as_nanos();
    println!("asm version time elapsed: {}ns", elapsed_tb);

    const REPEAT: usize = 100_000_000;
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

fn factorial_r(s: u64) -> u64 {
    if s == 1 {
        return 1;
    }
    return s * factorial_r(s - 1_u64);
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

///
/// gets the most repeated character and its frequency
/// the string must be a valid CString, so don't expect
/// non-ascii characters to work
///
pub fn get_most_repeated_character<T: Into<Vec<u8>>>(string: T) -> (u64, char) {
    let buff = string.into();
    let mut ch: c_char = 0;
    let ch_ptr = &mut ch;
    (
        unsafe { count_max_freq(CString::new(buff).unwrap().as_ptr(), ch_ptr) },
        ch as u8 as char,
    )
}

/// Count number of 1 bits in the buffer (asm version)
pub fn count_bits<T: Into<Vec<u8>>>(string: T) -> u64 {
    let buff = string.into();
    let size = buff.len();
    unsafe { count_bits_extern(CString::new(buff).unwrap().as_ptr(), size as u64) }
}

fn factorial(a: u64) -> u64 {
    unsafe { factorial_extern(a) }
}

fn little_endian_example() {
    // [0x0001] => [0x01, 0x02] => 0x0201
    let x: u16 = 1;
    // get memory address as a u8 pointer
    // [0x01, 0x00]
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

#[cfg(test)]
mod tests {
    use super::get_most_repeated_character;

    #[test]
    fn test_frequencies() {
        assert_eq!(get_most_repeated_character("aaaaaae"), (6, 'a'));
        assert_eq!(get_most_repeated_character("aaeee"), (3, 'e'));
        assert_eq!(get_most_repeated_character("aaaeee"), (3, 'e'));
        assert_eq!(get_most_repeated_character("aaeeea"), (3, 'a'));
        assert_eq!(get_most_repeated_character(""), (0, 0 as char));
    }
}
