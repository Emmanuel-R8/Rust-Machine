use crate::common::constants::{
    QTag,
    VMAttribute,
    VMResultCode,
    CDR,
    MEMORYWAD_ADDRESS_SHIFT,
    MEMORY_ADDRESS_PAGE_SHIFT,
    MEMORY_PAGE_MASK,
    PROT_EXEC,
    PROT_READ,
    PROT_WRITE,
    VMATTRIBUTE_ACCESS_FAULT,
    VMATTRIBUTE_EMPTY,
    VMATTRIBUTE_EPHEMERAL,
    VMATTRIBUTE_EXISTS,
    VMATTRIBUTE_MODIFIED,
    VMATTRIBUTE_TRANSPORT_DISABLE,
    VMATTRIBUTE_TRANSPORT_FAULT,
    VMATTRIBUTE_WRITE_FAULT,
};
use crate::common::types::{ Address, MemoryCell };
use crate::emulator::emulator::GlobalContext;
use crate::utils::{dpb, ldb};

// From https://github.com/mohanson/gameboy/blob/master/src/memory.rs
pub trait Memory {
    fn get(&self, a: Address) -> MemoryCell;

    fn set(&mut self, a: Address, v: MemoryCell);

}

// Constants
// const OBJECT_T: MemoryCell = MemoryCell::CdrTagData(QCDRTagData {
//     `cdr`: CDR::Jump,
//     tag: QTag::Symbol,
//     data: u32::Addr(ADDRESS_T),
// });

// const OBJECT_NIL: MemoryCell = MemoryCell::CdrTagData(QCDRTagData {
//     `cdr`: CDR::Jump,
//     tag: QTag::Symbol,
//     data: u32::Addr(ADDRESS_NIL),
// });

// const OBJECT_CDR_MASK: MemoryCell = MemoryCell::CdrTagData(QCDRTagData {
//     `cdr`: CDR::Jump,
//     tag: QTag::TagCdrMask,
//     data: u32::Unsigned(0),
// });

// pub fn make_lisp_obj(c: CDR, t: QTag, d: u32) -> MemoryCell {
//     return MemoryCell::CdrTagData(QCDRTagData {
//         `cdr`: c,
//         tag: t,
//         data: d,
//     });
// }

// pub fn make_lisp_obj_u(c: CDR, t: QTag, val: u32) -> MemoryCell {
//     return MemoryCell::CdrTagData(QCDRTagData {
//         `cdr`: c,
//         tag: t,
//         data: u32::Unsigned(val),
//     });
// }

// pub fn make_lisp_obj_i(c: CDR, t: QTag, val: i32) -> MemoryCell {
//     return MemoryCell::CdrTagData(QCDRTagData {
//         `cdr`: c,
//         tag: t,
//         data: u32::Signed(val),
//     });
// }

// pub fn make_lisp_obj_f(c: CDR, t: QTag, val: f32) -> MemoryCell {
//     return MemoryCell::CdrTagData(QCDRTagData {
//         `cdr`: c,
//         tag: t,
//         data: u32::Float(val),
//     });
// }

// pub fn get_cdr(q: MemoryCell) -> Option<CDR> {
//     return match q {
//         MemoryCell::CdrTagData(p) => Some(p.cdr),
//         _ => None,
//     };
// }

// pub fn get_tag(q: MemoryCell) -> Option<QTag> {
//     return match q {
//         MemoryCell::CdrTagData(p) => Some(p.tag),
//         _ => None,
//     };
// }

// pub fn get_data(q: MemoryCell) -> Option<u32> {
//     return match q {
//         MemoryCell::CdrTagData(p) => Some(p.data),
//         _ => None,
//     };
// }

// pub fn set_cdr(q: &mut MemoryCell, newcdr: CDR) {
//     match q {
//         MemoryCell::CdrTagData(mut p) => {
//             p.`cdr` = newcdr;
//         }
//         _ => {}
//     }
// }

// pub fn set_tag(q: &mut MemoryCell, newtag: QTag) {
//     match q {
//         MemoryCell::CdrTagData(mut p) => {
//             p.tag = newtag;
//         }
//         _ => {}
//     }
// }

// pub fn set_data(q: &mut MemoryCell, newdata: u32) {
//     match q {
//         MemoryCell::CdrTagData(mut p) => {
//             p.data = u32::Unsigned(newdata);
//         }
//         _ => {}
//     }
// }

pub fn memory_page_number(vma: u32) -> u32 {
    return vma >> MEMORY_ADDRESS_PAGE_SHIFT;
}
pub fn memory_page_offset(vma: u32) -> u32 {
    return vma & MEMORY_PAGE_MASK;
}
pub fn page_number_memory(vpn: u32) -> u32 {
    return vpn << MEMORY_ADDRESS_PAGE_SHIFT;
}

pub fn access_fault(vma: &VMAttribute) -> bool {
    return (vma & VMATTRIBUTE_ACCESS_FAULT) != 0;
}
pub fn write_fault(vma: &VMAttribute) -> bool {
    return (vma & VMATTRIBUTE_WRITE_FAULT) != 0;
}
pub fn transport_fault(vma: &VMAttribute) -> bool {
    return (vma & VMATTRIBUTE_TRANSPORT_FAULT) != 0;
}
pub fn transport_disable(vma: &VMAttribute) -> bool {
    return (vma & VMATTRIBUTE_TRANSPORT_DISABLE) != 0;
}
pub fn ephemeral(vma: &VMAttribute) -> bool {
    return vma & VMATTRIBUTE_EPHEMERAL > 0;
}
pub fn modified(vma: &VMAttribute) -> bool {
    return (vma & VMATTRIBUTE_MODIFIED) != 0;
}
pub fn exists(vma: &VMAttribute) -> bool {
    return (vma & VMATTRIBUTE_EXISTS) != 0;
}

pub fn set_vmaccess_fault(mut vma: VMAttribute) {
    return {
        vma = vma | VMATTRIBUTE_ACCESS_FAULT;
    };
}
pub fn set_vmwrite_fault(mut vma: VMAttribute) {
    return {
        vma = vma | VMATTRIBUTE_WRITE_FAULT;
    };
}
pub fn set_vmtransport_fault(mut vma: VMAttribute) {
    return {
        vma = vma | VMATTRIBUTE_TRANSPORT_FAULT;
    };
}
pub fn set_vmtransport_disable(mut vma: VMAttribute) {
    return {
        vma = vma | VMATTRIBUTE_TRANSPORT_DISABLE;
    };
}
pub fn set_vmephemeral(mut vma: VMAttribute) {
    return {
        vma = vma | VMATTRIBUTE_EPHEMERAL;
    };
}
pub fn set_vmmodified(mut vma: VMAttribute) {
    return {
        vma = vma | VMATTRIBUTE_MODIFIED;
    };
}
pub fn set_vmexists(mut vma: VMAttribute) {
    return {
        vma = vma | VMATTRIBUTE_EXISTS;
    };
}

pub fn clear_vmaccess_fault(mut vma: VMAttribute) {
    return {
        vma = vma & (VMATTRIBUTE_ACCESS_FAULT ^ 0b1111_1111);
    };
}
pub fn clear_vmwrite_fault(mut vma: VMAttribute) {
    return {
        vma = vma & (VMATTRIBUTE_WRITE_FAULT ^ 0b1111_1111);
    };
}
pub fn clear_vmtransport_faultlt(mut vma: VMAttribute) {
    return {
        vma = vma & (VMATTRIBUTE_TRANSPORT_FAULT ^ 0b1111_1111);
    };
}
pub fn clear_vmtransport_disable(mut vma: VMAttribute) {
    return {
        vma = vma & (VMATTRIBUTE_TRANSPORT_DISABLE ^ 0b1111_1111);
    };
}
pub fn clear_vmephemeral(mut vma: VMAttribute) {
    return {
        vma = vma & (VMATTRIBUTE_EPHEMERAL ^ 0b1111_1111);
    };
}
pub fn clear_vmmodified(mut vma: VMAttribute) {
    return {
        vma = vma & (VMATTRIBUTE_MODIFIED ^ 0b1111_1111);
    };
}
pub fn clear_vmexists(mut vma: VMAttribute) {
    return {
        vma = vma & (VMATTRIBUTE_EXISTS ^ 0b1111_1111);
    };
}

#[derive(Debug)]
pub struct VMMemory {
    pub tags: [u8; 1 << 31],  /* 2^32 bytes of tags + data */
    pub data: [u32; 1 << 31], /* 2^32 bytes of tags + data */
    pub attribute: [VMAttribute; 1 << (32 - MEMORY_ADDRESS_PAGE_SHIFT)],
}

impl VMMemory {
    pub fn map_virtual_address_data(&self, start: usize, count: usize) -> Option<Vec<MemoryCell>> {
        let s = self.data;

        if count == 0 {
            None
        } else {
            let m = s[start..start + count]
                .into_iter()
                .map(|&u| MemoryCell::new_cdr_tag_u(CDR::Jump, QTag::Fixnum, u))
                .collect();

            Some(m)
        }
    }
}

static ENABLE_IDS: bool = false;

pub fn default_attributes(faultp: bool, worldp: bool) -> u8 {
    let mut accessfaultp = VMATTRIBUTE_EMPTY;
    if faultp {
        accessfaultp = VMATTRIBUTE_ACCESS_FAULT;
    }

    let mut modifiedp = VMATTRIBUTE_EMPTY;
    if ENABLE_IDS && worldp {
        modifiedp = VMATTRIBUTE_MODIFIED;
    }

    return VMATTRIBUTE_EXISTS | VMATTRIBUTE_MODIFIED | accessfaultp | modifiedp;
}

pub fn vmcommand_opcode(command: u32) -> u32 {
    return ldb(13, 19, command);
}

pub fn vmcommand_operand(command: u32) -> u32 {
    return ldb(19, 0, command);
}

pub fn set_vmreply_result(reply: u32, result: u32) -> u32 {
    let r: u32;

    if result != 0 {
        r = VMResultCode::Success as u32;
    } else {
        r = VMResultCode::Failure as u32;
    }

    return dpb(r, 13, 19, reply);
}

pub fn memory_wad_number(vma: u32) -> u32 {
    return vma >> MEMORYWAD_ADDRESS_SHIFT;
}
pub fn memory_wad_offset(vma: u32) -> u32 {
    return vma & MEMORY_PAGE_MASK;
}
pub fn wad_number_memory(vwn: u32) -> u32 {
    return vwn << MEMORYWAD_ADDRESS_SHIFT;
}

// f-ing poor excuse for a macro language
impl<'a> GlobalContext<'a> {
    pub fn wad_created(&self, vma: u32) -> bool {
        // WADs are 8 contiguous memory pages
        let wad_addr = memory_wad_number(vma) << 3;
        let mut is_created = true;

        for vma in wad_addr..wad_addr + 8 {
            is_created = is_created && self.vma_created_p(vma);
        }

        return is_created;
    }
}

// Computes the PROT_XXX setting for a particular combination of
// VMAttribute's.  C.f., segv_handler, which translates resulting segfault
// back to appropriate Lisp fault
pub fn compute_protection(mut vma: VMAttribute) -> u32 {
    //  Don't cause transport faults if they are overridden
    if (vma & VMATTRIBUTE_TRANSPORT_DISABLE) != 0 {
        clear_vmtransport_disable(vma);
    }

    // We would have liked Transport to use write-only pages, but that is not guaranteed by
    // OSF/Unix, so we just use none
    if (vma & (VMATTRIBUTE_EXISTS | VMATTRIBUTE_TRANSPORT_FAULT | VMATTRIBUTE_ACCESS_FAULT))
        != VMATTRIBUTE_EXISTS
    {
        return PROT_READ | PROT_EXEC;
    }

    // Unless the modified and ephemeral bits are set, use read-only, so we can update them
    if (vma & (VMATTRIBUTE_MODIFIED | VMATTRIBUTE_EPHEMERAL | VMATTRIBUTE_WRITE_FAULT))
        != (VMATTRIBUTE_MODIFIED | VMATTRIBUTE_EPHEMERAL)
    {
        return PROT_READ | PROT_EXEC;
    }
    return PROT_READ | PROT_EXEC | PROT_WRITE;
}

// pub fn ensure_virtual_address(ctx:GlobalContext, vma: u32) -> bool {
//     let mut data: u64 = 0;
//     let mut tag: u64 = 0;
//     let mut aligned_vma = memory_page_offset(vma);

//     if ctx.vma_created_p(vma) {
//         return true
//     }

//     data = &mut *DataSpace.offset(aligned_vma) as *mut u32 as u64;
//     tag = &mut *TagSpace.offset(aligned_vma) as *mut Tag as u64;
//     if data
//         != mmap(
//             data,
//             ::std::mem::size_of::<[u32; 8192]>(),
//             0x1 | 0x2,
//             0x20 | 0x2 | 0x10,
//             -(1),
//             0 as __off_t,
//         ) as u64
//     {
//         printf(b"Couldn't map data page at %s for VMA %016lx\0", data, vma);
//     }
//     if tag
//         != mmap(
//             tag,
//             ::std::mem::size_of::<[Tag; 8192]>(),
//             0x1 | 0x2,
//             0x20 | 0x2 | 0x10,
//             -(1),
//             0 as __off_t,
//         ) as u64
//     {
//         printf(b"Couldn't map tag page at %s for VMA %016lx\0", tag, vma);
//     }
//     VMAttributeTable[(vma >> 13)] = (0o1 | 0o4 | 0o100) as VMAttribute;

//     return vma;
// }

// pub fn ensure_virtual_address_range(virtualaddress: u32, count: u32, faultp: bool) -> u32 {
//     // let mut pages: u32 = (count + (0x2000 - 1)) / 0x2000;
//     // let mut data: u64 = "";
//     // let mut tag: u64 = "";
//     // let mut aligned_vma: u32 = virtualaddress.wrapping_sub(virtualaddress & (0x2000 - 1));

//     let mut pages = ceiling(count, MEMORY_PAGE_SIZE);
//     let aligned_vma = virtualaddress - memory_page_offset(virtualaddress);

//     // let mut n: u32 = 0;
//     // while pages != 0 {
//     //     n = 0;
//     //     while VMAttributeTable[(virtualaddress >> 13)] & 0o100 == 0 && pages != 0 {
//     //         n += 1;
//     //         pages -= 1;
//     //         VMAttributeTable[(virtualaddress >> 13)] = (0o1 | 0o4 | 0o100) as VMAttribute;
//     //         virtualaddress = (virtualaddress).wrapping_add(0x2000);
//     //     }
//     //     if n != 0 {
//     //         data = &mut *DataSpace.offset(aligned_vma) as *mut u32 as u64;
//     //         tag = &mut *TagSpace.offset(aligned_vma) as *mut Tag as u64;
//     //         if data
//     //             != mmap(
//     //                 data,
//     //                 (n).wrapping_mul(::std::mem::size_of::<[u32; 8192]>()),
//     //                 0x1 | 0x2,
//     //                 0x20 | 0x2 | 0x10,
//     //                 -(1),
//     //                 0 as __off_t,
//     //             ) as u64
//     //         {
//     //             printf(
//     //                 b"Couldn't map %d data pages at %s for VMA %016lx\0",
//     //                 n,
//     //                 data,
//     //                 aligned_vma,
//     //             );
//     //         }
//     //         if tag
//     //             != mmap(
//     //                 tag,
//     //                 (n).wrapping_mul(::std::mem::size_of::<[Tag; 8192]>()),
//     //                 0x1 | 0x2,
//     //                 0x20 | 0x2 | 0x10,
//     //                 -(1),
//     //                 0 as __off_t,
//     //             ) as u64
//     //         {
//     //             printf(
//     //                 b"Couldn't map %d tag pages at %s for VMA %016lx\0",
//     //                 n,
//     //                 tag,
//     //                 aligned_vma,
//     //             );
//     //         }
//     //         aligned_vma = (aligned_vma).wrapping_add((n * 0x2000));
//     //     }
//     //     while VMAttributeTable[(virtualaddress >> 13)] & 0o100 != 0 && pages != 0 {
//     //         pages -= 1;
//     //         virtualaddress = (virtualaddress).wrapping_add(0x2000);
//     //         aligned_vma = (aligned_vma).wrapping_add(0x2000);
//     //     }
//     // }
//     // return virtualaddress;

//     return 0;
// }
