/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum GX_TRANSFER_FORMAT {
    GX_TRANSFER_FMT_RGBA8 = 0,
    GX_TRANSFER_FMT_RGB8 = 1,
    GX_TRANSFER_FMT_RGB565 = 2,
    GX_TRANSFER_FMT_RGB5A1 = 3,
    GX_TRANSFER_FMT_RGBA4 = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum GX_TRANSFER_SCALE {
    GX_TRANSFER_SCALE_NO = 0,
    GX_TRANSFER_SCALE_X = 1,
    GX_TRANSFER_SCALE_XY = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum GX_FILL_CONTROL {
    GX_FILL_TRIGGER = 1,
    GX_FILL_FINISHED = 2,
    GX_FILL_16BIT_DEPTH = 0,
    GX_FILL_24BIT_DEPTH = 256,
    GX_FILL_32BIT_DEPTH = 512,
}
extern "C" {
    pub static mut gxCmdBuf: *mut u32_;
}
extern "C" {
    pub fn GX_RequestDma(src: *mut u32_, dst: *mut u32_, length: u32_)
     -> Result;
    pub fn GX_ProcessCommandList(buf0a: *mut u32_, buf0s: u32_, flags: u8_)
     -> Result;
    pub fn GX_MemoryFill(buf0a: *mut u32_, buf0v: u32_, buf0e: *mut u32_,
                         control0: u16_, buf1a: *mut u32_, buf1v: u32_,
                         buf1e: *mut u32_, control1: u16_) -> Result;
    pub fn GX_DisplayTransfer(inadr: *mut u32_, indim: u32_,
                              outadr: *mut u32_, outdim: u32_, flags: u32_)
     -> Result;
    pub fn GX_TextureCopy(inadr: *mut u32_, indim: u32_, outadr: *mut u32_,
                          outdim: u32_, size: u32_, flags: u32_) -> Result;
    pub fn GX_FlushCacheRegions(buf0a: *mut u32_, buf0s: u32_,
                                buf1a: *mut u32_, buf1s: u32_,
                                buf2a: *mut u32_, buf2s: u32_) -> Result;
}
use ::types::*;