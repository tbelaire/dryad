use std::fmt;
use std::slice;

#[repr(C)]
pub struct Sym {
 pub st_name: u32, /* Symbol name (string tbl index) */
 pub st_info: u8, /* Symbol type and binding */
 pub st_other: u8, /* Symbol visibility */
 pub st_shndx: u16, /* Section index */
 pub st_value: u64, /* Symbol value */
 pub st_size: u64 /* Symbol size */
}

pub const SIZEOF_SYM: usize = 4 + 1 + 1 + 2 + 8 + 8;

#[inline]
pub fn st_bind(info: u8) -> u8 {
    info >> 4
}

#[inline]
pub fn st_type(info: u8) -> u8 {
    info & 0xf
}

#[inline(always)]
pub fn is_import(sym: &Sym) -> bool {
    let binding = st_bind(sym.st_info);
    binding == STB_GLOBAL && sym.st_value == 0
}

// sym bindings
pub const STB_LOCAL:u8 = 0;/* Local symbol */
pub const STB_GLOBAL:u8 = 1;/* Global symbol */
pub const STB_WEAK:u8 = 2;/* Weak symbol */
pub const STB_NUM:u8 = 3;/* Number of defined types.  */
pub const STB_LOOS:u8 = 10;/* Start of OS-specific */
pub const STB_GNU_UNIQUE:u8 = 10;/* Unique symbol.  */
pub const STB_HIOS:u8 = 12;/* End of OS-specific */
pub const STB_LOPROC:u8 = 13;/* Start of processor-specific */
pub const STB_HIPROC:u8 = 15;/* End of processor-specific */

#[inline]
pub fn bind_to_str(typ: u8) -> &'static str {
    match typ {
        STB_LOCAL => "LOCAL",
        STB_GLOBAL => "GLOBAL",
        STB_WEAK => "WEAK",
        STB_NUM => "NUM",
        STB_GNU_UNIQUE => "GNU_UNIQUE",
        _ => "UNKNOWN_STB"
    }
}

// sym types
pub const STT_NOTYPE:u8 = 0;/* Symbol type is unspecified */
pub const STT_OBJECT:u8 = 1;/* Symbol is a data object */
pub const STT_FUNC:u8 = 2;/* Symbol is a code object */
pub const STT_SECTION:u8 = 3;/* Symbol associated with a section */
pub const STT_FILE:u8 = 4;/* Symbol's name is file name */
pub const STT_COMMON:u8 = 5;/* Symbol is a common data object */
pub const STT_TLS:u8 = 6;/* Symbol is thread-local data object*/
pub const STT_NUM:u8 = 7;/* Number of defined types.  */
pub const STT_LOOS:u8 = 10;/* Start of OS-specific */
pub const STT_GNU_IFUNC:u8 = 10;/* Symbol is indirect code object */
pub const STT_HIOS:u8 = 12;/* End of OS-specific */
pub const STT_LOPROC:u8 = 13;/* Start of processor-specific */
pub const STT_HIPROC:u8 = 15;/* End of processor-specific */

#[inline]
pub fn type_to_str(typ: u8) -> &'static str {
    match typ {
        STT_NOTYPE => "NOTYPE",
        STT_OBJECT => "OBJECT",
        STT_FUNC => "FUNC",
        STT_SECTION => "SECTION",
        STT_FILE => "FILE",
        STT_COMMON => "COMMON",
        STT_TLS => "TLS",
        STT_NUM => "NUM",
        STT_GNU_IFUNC => "GNU_IFUNC",
        _ => "UNKNOWN_STT"
        
    }
}

impl fmt::Debug for Sym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bind = st_bind(self.st_info);
        let typ = st_type(self.st_info);
        write!(f, "st_name: {} {} {} st_other: {} st_shndx: {} st_value: {:x} st_size: {}",
               self.st_name, bind_to_str(bind), type_to_str(typ), self.st_other, self.st_shndx, self.st_value, self.st_size)
    }
}

pub fn get_symtab<'a> (symp: *const Sym, count: usize) -> &'a [Sym] {
    unsafe { slice::from_raw_parts(symp, count) }
}
