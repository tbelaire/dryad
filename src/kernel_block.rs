use auxv;
use utils::*;

use std::slice;

const AUX_CNT:usize = 38;

pub struct KernelBlock<'a>{
    pub argc: isize,
    pub argv: &'a[*const u8],
    pub envc: isize,
    pub env: &'a[*const u8],
    pub auxv: *mut auxv::Elf64_auxv_t,
}

impl<'b> KernelBlock<'b> {
    pub fn getauxval(&self, t:u64) -> Option<u64> {
        unsafe {
            let ptr = self.auxv.clone();
            let mut i = 1;
            let mut v = &*ptr;
            while v.a_type != auxv::AT_NULL {
                if v.a_type == t {
                    return Some (v.a_val)
                }
                v = &*ptr.offset(i);
                i += 1;
            }
        }
        None
    }

    pub fn getenv<'a>(&self, name:&'static str) -> Option<&'a str> {
        for i in 0..self.envc - 1 {
            let evar = as_str(self.env[i as usize]);
            if evar.starts_with(name) { // perhaps add custom search to check if starts with, then if so, return the chars after the =, for linear return; but probably who cares
                let idx = evar.find("=").unwrap() + 1; // this unwrap probably safe since it would mean the environment variable wasn't properly constructed
                let (_, res) = evar.split_at(idx as usize);
                return Some (res)
            }
        }
        None
    }

    // TODO: add auxc and make auxv a slice of auxv_t
    pub fn new<'a> (args: *const u64) -> KernelBlock<'a> {
        unsafe {
            let argc = (*args) as isize;
            let argv = args.offset(1) as *const *const u8;
            let envp = argv.offset(argc + 1);

            let mut p = envp;
            let mut envc = 1;
            // two null pointers mark end of envp
            // and beginning of the auxillary vectors
            while !(*p).is_null() {
                p = p.offset(1);
                envc += 1;
            }
            p = p.offset(1);
            let auxv = p as *mut auxv::Elf64_auxv_t;
            KernelBlock {
                argc: argc,
                argv: slice::from_raw_parts(argv, argc as usize),
                envc: envc,
                env: slice::from_raw_parts(envp, envc as usize),
                auxv: auxv,
            }
        }
    }

    // consider using inout stack-allocated &[u64] slice?
    pub fn get_aux (&self) -> Vec<u64> {
        let mut aux:Vec<u64> = vec![0; AUX_CNT];
        let mut i:isize = 0;
        unsafe {
            while (&*self.auxv.offset(i)).a_val != auxv::AT_NULL {
                let auxv_t = &*self.auxv.offset(i);
                // musl wants the aux a_val array to be indexed by AT_<TYPE>
                aux[auxv_t.a_type as usize] = auxv_t.a_val;
                i += 1;
            }
        }
        aux
    }

    pub unsafe fn unsafe_print (&self) -> () {
        write(&"argc: ");
        write_u64(self.argc as u64, false);
        write(&"\n");
        write(&"argv[0]: ");
        write_chars_at(self.argv[0]);
        write(&"\n");
        write(&"envc: ");
        write_u64(self.envc as u64, false);
        write(&"\n");
    }
}
