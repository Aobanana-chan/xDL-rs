use libc::*;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XdlInfoT {
    pub dli_fname: *const ::std::os::raw::c_char,
    pub dli_fbase: *mut ::std::os::raw::c_void,
    pub dli_sname: *const ::std::os::raw::c_char,
    pub dli_saddr: *mut ::std::os::raw::c_void,
    pub dli_ssize: usize,
    pub dlpi_phdr: *const Elf32_Phdr,
    pub dlpi_phnum: usize,
}
pub const XDL_DEFAULT: i32 = 0;
pub const XDL_TRY_FORCE_LOAD: i32 = 1;
pub const XDL_ALWAYS_FORCE_LOAD: i32 = 2;
pub const XDL_FULL_PATHNAME: i32 = 1;
pub const XDL_DI_DLINFO: i32 = 1;

extern "C" {
    pub fn xdl_open(
        filename: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xdl_close(handle: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xdl_sym(
        handle: *mut ::std::os::raw::c_void,
        symbol: *const ::std::os::raw::c_char,
        symbol_size: *mut usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xdl_dsym(
        handle: *mut ::std::os::raw::c_void,
        symbol: *const ::std::os::raw::c_char,
        symbol_size: *mut usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xdl_addr(
        addr: *mut ::std::os::raw::c_void,
        info: *mut XdlInfoT,
        cache: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xdl_addr_clean(cache: *mut *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn xdl_iterate_phdr(
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut dl_phdr_info,
                arg2: usize,
                arg3: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        data: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xdl_info(
        handle: *mut ::std::os::raw::c_void,
        request: ::std::os::raw::c_int,
        info: *mut XdlInfoT,
    ) -> ::std::os::raw::c_int;
}