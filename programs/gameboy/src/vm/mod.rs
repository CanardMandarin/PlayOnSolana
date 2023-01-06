pub mod register;
pub use register::*;

pub mod serial;
pub use serial::*;

pub mod cpu;
pub use cpu::*;

pub mod mmu;
pub use mmu::*;

pub mod mbc;
pub use mbc::*;

pub mod timer;
pub use timer::*;

pub mod keypad;
pub use keypad::*;

pub mod gbmode;
pub use gbmode::*;

pub type StrResult<T> = Result<T, &'static str>;
