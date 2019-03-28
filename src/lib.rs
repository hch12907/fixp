#![no_std]

mod fixp16;
mod fixp22;

pub use fixp16::Fixed16_16 as Q16_16;
pub use fixp22::Fixed22_10 as Q22_10;