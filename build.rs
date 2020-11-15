extern crate cc;
extern crate nasm_rs;

fn main() {
    nasm_rs::compile_library("add.a", &["add.s", "count.asm"]);
}
