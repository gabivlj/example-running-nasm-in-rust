use nasm_rs;

fn main() {
    nasm_rs::compile_library(
        "add.a",
        &["add.s", "count.asm", "switch.asm", "count_max_freq.asm"],
    )
    .unwrap();
}
