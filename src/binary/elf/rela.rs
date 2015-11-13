#[repr(C)]
pub struct Elf64_Rela {
  pub r_offset: u64, /* Address */
  pub r_info: u64,/* Relocation type and symbol index */
  pub r_addend:i64,/* Addend */
}

#[inline]
pub fn r_sym (info: u64) -> u64 {
    info >> 32
}

#[inline]
pub fn r_type (info: u64) -> u64 {
    info & 0xffffffff
}

#[inline]
pub fn r_info (sym: u64, typ: u64) -> u64 {
    (sym << 32) + typ
}

impl Elf64_Rela {
    
}

pub const R_X86_64_NONE:u64 = 0; /* No reloc */
pub const R_X86_64_64:u64 = 1; /* Direct 64 bit  */
pub const R_X86_64_PC32:u64 = 2; /* PC relative 32 bit signed */
pub const R_X86_64_GOT32:u64 = 3; /* 32 bit GOT entry */
pub const R_X86_64_PLT32:u64 = 4; /* 32 bit PLT address */
pub const R_X86_64_COPY:u64 = 5; /* Copy symbol at runtime */
pub const R_X86_64_GLOB_DAT:u64 = 6; /* Create GOT entry */
pub const R_X86_64_JUMP_SLOT:u64 = 7; /* Create PLT entry */
pub const R_X86_64_RELATIVE:u64 = 8; /* Adjust by program base */
pub const R_X86_64_GOTPCREL:u64 = 9; /* 32 bit signed PC relative offset to GOT */
pub const R_X86_64_32:u64 = 10; /* Direct 32 bit zero extended */
pub const R_X86_64_32S:u64 = 11; /* Direct 32 bit sign extended */
pub const R_X86_64_16:u64 = 12; /* Direct 16 bit zero extended */
pub const R_X86_64_PC16:u64 = 13; /* 16 bit sign extended pc relative */
pub const R_X86_64_8:u64 = 14; /* Direct 8 bit sign extended  */
pub const R_X86_64_PC8:u64 = 15; /* 8 bit sign extended pc relative */
pub const R_X86_64_DTPMOD64:u64 = 16; /* ID of module containing symbol */
pub const R_X86_64_DTPOFF64:u64 = 17; /* Offset in module's TLS block */
pub const R_X86_64_TPOFF64:u64 = 18; /* Offset in initial TLS block */
pub const R_X86_64_TLSGD:u64 = 19; /* 32 bit signed PC relative offset to two GOT entries for GD symbol */
pub const R_X86_64_TLSLD:u64 = 20; /* 32 bit signed PC relative offset to two GOT entries for LD symbol */
pub const R_X86_64_DTPOFF32:u64 = 21; /* Offset in TLS block */
pub const R_X86_64_GOTTPOFF:u64 = 22; /* 32 bit signed PC relative offset to GOT entry for IE symbol */
pub const R_X86_64_TPOFF32:u64 = 23; /* Offset in initial TLS block */
pub const R_X86_64_PC64:u64 = 24; /* PC relative 64 bit */
pub const R_X86_64_GOTOFF64:u64 = 25; /* 64 bit offset to GOT */
pub const R_X86_64_GOTPC32:u64 = 26; /* 32 bit signed pc relative offset to GOT */
pub const R_X86_64_GOT64:u64 = 27; /* 64-bit GOT entry offset */
pub const R_X86_64_GOTPCREL64:u64 = 28; /* 64-bit PC relative offset to GOT entry */
pub const R_X86_64_GOTPC64:u64 = 29; /* 64-bit PC relative offset to GOT */
pub const R_X86_64_GOTPLT64:u64 = 30; /* like GOT64, says PLT entry needed */
pub const R_X86_64_PLTOFF64:u64 = 31; /* 64-bit GOT relative offset to PLT entry */
pub const R_X86_64_SIZE32:u64 = 32; /* Size of symbol plus 32-bit addend */
pub const R_X86_64_SIZE64:u64 = 33; /* Size of symbol plus 64-bit addend */
pub const R_X86_64_GOTPC32_TLSDESC:u64 = 34; /* GOT offset for TLS descriptor.*/
pub const R_X86_64_TLSDESC_CALL:u64 = 35; /* Marker for call through TLS descriptor.  */
pub const R_X86_64_TLSDESC:u64 = 36; /* TLS descriptor.  */
pub const R_X86_64_IRELATIVE:u64 = 37; /* Adjust indirectly by program base */
pub const R_X86_64_RELATIVE64:u64 = 38; /* 64-bit adjust by program base */
pub const R_X86_64_NUM:u64 = 39; 
