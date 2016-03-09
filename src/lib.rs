//#![feature(no_std, lang_items, asm, core, core_str_ext)]
//#![no_std]
#![feature(asm, libc)]
#![no_main]

//#![feature(std_panic, recover)]

#![allow(dead_code)] // yells about consts otherwise
#![allow(unused_variables)]

/// Dryad --- the world's first non-functional, yet-to-be-implemented, might be impossible or more likely inefficient --- parallel, dynamic linker.
/// Many, many thanks to Mutabah, durka42, aatch, tilpner, niconii, bluss, and so many others on the IRC channel for answering my stupid questions.
/// TODO: add support for invoking ./dryad <binary> <args>

mod auxv;
mod kernel_block;
mod utils;
mod binary;
mod relocate;
mod link_map;
//mod scoped_thread;
pub mod linker;

use std::mem;

use kernel_block::KernelBlock;
use utils::*;

extern crate libc;

// below is gcc attrs for this function...
//extern "C"
//void __attribute__((noinline)) __attribute__((visibility("default")))
// unused; someone figure out how to get gdb working when running as a dyld
extern {
    fn rtld_db_dlactivity();
}

extern {
    /// ELF abi requires `_start`; this must be in assembly because we need
    /// the raw stack pointer as the argument to `_dryad_init`;
    /// i.e., kernel calls symbol `_start` on dynamic linker with the kernel argument block, etc.,
    /// which in our case then calls _back_ into `dryad_init` with the pointer to the raw arguments that form the kernel block
    /// see `arch/x86/asm.s`
    fn _start();
}

/*
fn maintain_illusion (block: &kernel_block::KernelBlock) {
    use auxv::*;
    let mut i = 0;
    unsafe {
        loop {
            let mut auxv = block.auxv.offset(i);
            match (*auxv).a_type {
                AT_NULL => return,
                  AT_PHDR => (*auxv).a_val = 0,
                AT_PHNUM => (*auxv).a_val = 0,
                // AT_ENTRY => (*auxv).a_val = 0,
                AT_EXECFN => (*auxv).a_val = 0,
                _ => (),
            }
            i += 1;
        }
    }
}
*/

fn dryad_main (dryad: &mut linker::Linker, block: &kernel_block::KernelBlock) -> Result<(), String> {
    println!("Dryad:\n  {:#?}", &dryad);
    println!("BEGIN EXE LINKING");
//    linker::LINKER_ADDR = Some(dryad);
    let name = utils::as_str(block.argv[0]);
    let phdr_addr = block.getauxval(auxv::AT_PHDR).unwrap();
    let phnum  = block.getauxval(auxv::AT_PHNUM).unwrap();
    let main_image = try!(binary::elf::image::SharedObject::from_executable(name, phdr_addr, phnum as usize));
    println!("Main Image:\n  {:#?}", &main_image);

    dryad.link_executable(main_image)
    // TODO: according to the libc implementation, the binary needs to get a stack and argc that looks like it was executed directly --- but the auxv already has the phdr_addr set correctly, so i'm not sure what all that code is for...?
    //maintain_illusion(&block);
}

#[no_mangle]
pub extern fn _dryad_init (raw_args: *const u64) -> u64 {

    // the linker is currently tied to the lifetime of the kernel block... but really it's static
    let block = KernelBlock::new(raw_args);
    unsafe {
        // TODO: this is getting EFAULT badd address because it's trying to print string locals which haven't be relocated yet...
        block.unsafe_print();
    }

    let linker_base = block.getauxval(auxv::AT_BASE).unwrap();
    let entry  = block.getauxval(auxv::AT_ENTRY).unwrap();

    let start_addr = _start as *const u64 as u64;    
    // TODO: add support for invoking ./dryad <binary> <args>
    if start_addr == entry {
        // because it's _tradition_
        // (https://fossies.org/dox/glibc-2.22/rtld_8c_source.html)
        // line 786:
        // > Ho ho.  We are not the program interpreter!  We are the program itself!
        unsafe { write(&"-=|dryad====-\nHo ho.  We are not the program interpreter!  We are the program itself!\n"); } // TODO: add box drawing random character gen here cause it'll be _cute_
        _exit(0);
        return 0xd47ad // to make compiler happy
    }

    match linker::Linker::new(linker_base, &block) {
        Ok (mut dryad) => {

            if let Err(msg) = dryad_main(&mut dryad, &block) {
                println!("{}", msg);
                _exit(1);
                0xd47ad

            } else {
                // if we don't forget the entire dryad linker then internal heap-allocated strings get corrupted, like SharedObject.name
                mem::forget(dryad);
                entry
            }
        },
        Err (msg) => {
            // relocating self failed somehow; we try to write the error message and exit
            unsafe { write(&msg); }
            _exit(1);
            0xd47ad
        }
    }
}
