#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#[allow(non_snake_case)]

mod shulk;

#[skyline::main(name = "dynamic_shulk_effects")]
pub fn main() {
    shulk::install();
}
