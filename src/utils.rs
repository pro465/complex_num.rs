//! directly copied from math.rs
//! https://github.com/nagisa/math.rs

pub const F64_SIGN_MASK: u64 = 1 << 63;
pub const F32_SIGN_MASK: u32 = 1 << 31;
pub const F32_MANTISSA_MASK: u32 = 0x007F_FFFF;
pub const F64_MANTISSA_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;
pub const F32_EXP_MASK: u32 = 0x7F80_0000;
pub const F64_EXP_MASK: u64 = 0x7FF0_0000_0000_0000;

pub const F32_MAX_EXP: i32 = 127;
pub const F32_DENORMAL_EXP: i32 = -127;
pub const F32_NAN_EXP: i32 = 128;

pub const F64_MAX_EXP: i32 = 1023;
pub const F64_DENORMAL_EXP: i32 = -1023;
pub const F64_NAN_EXP: i32 = 1024;
