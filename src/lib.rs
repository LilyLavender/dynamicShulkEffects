#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
//#![feature(asm)]

mod shulk;

#[skyline::main(name = "dynamic_shulk_effects")]
pub fn main() {
    shulk::install();
}
