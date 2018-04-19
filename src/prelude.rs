#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod ctypes {

    pub type c_uint = u32;
    pub type c_int = i32;
    pub type c_void = usize;
    pub type c_char = u8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_short = i16;
    pub type c_ushort = u16;

}
