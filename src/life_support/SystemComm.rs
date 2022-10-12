#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn VirtualMemoryRead(vma: isize, object: *mut LispObj) -> u32;
}
pub type i32 = u32;
pub type u32 = libc::c_uint;
pub type u64 = libc::c_ulong;
pub type i32 = i32;
pub type ui32 = u32;
pub type u64 = u64;
pub type EmbWord = i32;
pub type isize = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub union LispObj {
    pub parts: _LispObj,
    pub whole: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _LispObj {
    pub tag: ui32,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: ui32,
    pub s: i32,
    pub f: libc::c_float,
}
pub type ptrdiff_t = libc::c_long;
#[no_mangle]
pub  fn ReadSystemCommSlot(
    mut slot: u32,
    mut objectPointer: *mut LispObj,
) {
    VirtualMemoryRead(
        (0xf8041100 as libc::c_long as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong)
            .wrapping_add(slot as ptrdiff_t as libc::c_ulong),
        objectPointer,
    );
}
