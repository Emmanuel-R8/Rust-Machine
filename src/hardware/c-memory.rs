#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn TakeMemoryTrap(vector: u32, vma: isize);
    fn OldspaceAddressP(vma: isize) -> Boole;
    static mut MemoryActionTable: [[Byte; 64]; 12];
    fn MemoryReadInternal(vma: isize, object: *mut LispObj, row: *mut Byte) -> isize;
    static mut EnableIDS: Boole;
    fn vpunt(section: &str, format: &str, _: ...);
    static mut memory_vma: isize;
}

#[no_mangle]
pub fn EnsureVirtualAddress(mut vma: isize) -> isize {
    let mut data: caddr_t = "";
    let mut tag: caddr_t = "";
    let mut aligned_vma: isize = vma.wrapping_sub(vma & 0x2000);
    if VMAttributeTable[vma >> 13] & 0o100 != 0 {
        return vma;
    }
    data = &mut *DataSpace.offset(aligned_vma as isize) as *mut isize as caddr_t;
    tag = &mut *TagSpace.offset(aligned_vma as isize) as *mut Tag as caddr_t;
    if data
        != mmap(
            data as *mut libc::c_void,
            ::std::mem::size_of::<[isize; 8192]>() as libc::c_ulong,
            0x1 as usize | 0x2,
            0x20 as usize | 0x2 as usize | 0x10,
            -(1),
            0 as usize as __off_t,
        ) as caddr_t
    {
        printf(
            b"Couldn't map data page at %s for VMA %016lx\0" as *const u8 as *const libc::c_char,
            data,
            vma,
        );
    }
    if tag
        != mmap(
            tag as *mut libc::c_void,
            ::std::mem::size_of::<[Tag; 8192]>() as libc::c_ulong,
            0x1 as usize | 0x2,
            0x20 as usize | 0x2 as usize | 0x10,
            -(1),
            0 as usize as __off_t,
        ) as caddr_t
    {
        printf(
            b"Couldn't map tag page at %s for VMA %016lx\0" as *const u8 as *const libc::c_char,
            tag,
            vma,
        );
    }
    VMAttributeTable[(vma >> 13) as usize] = (0o1 as usize | 0o4 as usize | 0o100) as VMAttribute;
    return vma;
}

#[no_mangle]
pub fn EnsureVirtualAddressRange(
    mut virtualaddress: isize,
    mut count: u32,
    mut faultp: Boole,
) -> isize {
    let mut pages: usize = (count + (0x2000 as usize - 1)) / 0x2000;
    let mut data: caddr_t = "";
    let mut tag: caddr_t = "";
    let mut aligned_vma: isize =
        virtualaddress.wrapping_sub(virtualaddress & (0x2000 as usize - 1) as libc::c_ulong);
    let mut n: usize = 0;
    while pages != 0 {
        n = 0;
        while VMAttributeTable[(virtualaddress >> 13) as usize] as usize & 0o100 as usize == 0
            && pages != 0
        {
            n += 1;
            pages -= 1;
            VMAttributeTable[(virtualaddress >> 13) as usize] =
                (0o1 as usize | 0o4 | 0o100) as VMAttribute;
            virtualaddress = (virtualaddress as libc::c_ulong)
                .wrapping_add(0x2000 as usize as libc::c_ulong)
                as isize as isize;
        }
        if n != 0 {
            data = &mut *DataSpace.offset(aligned_vma as isize) as *mut isize as caddr_t;
            tag = &mut *TagSpace.offset(aligned_vma as isize) as *mut Tag as caddr_t;
            if data
                != mmap(
                    data as *mut libc::c_void,
                    (n as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<[isize; 8192]>() as libc::c_ulong),
                    0x1 as usize | 0x2,
                    0x20 as usize | 0x2 as usize | 0x10,
                    -(1),
                    0 as usize as __off_t,
                ) as caddr_t
            {
                printf(
                    b"Couldn't map %d data pages at %s for VMA %016lx\0" as *const u8
                        as *const libc::c_char,
                    n,
                    data,
                    aligned_vma,
                );
            }
            if tag
                != mmap(
                    tag as *mut libc::c_void,
                    (n as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<[Tag; 8192]>() as libc::c_ulong),
                    0x1 as usize | 0x2,
                    0x20 as usize | 0x2 as usize | 0x10,
                    -(1),
                    0 as usize as __off_t,
                ) as caddr_t
            {
                printf(
                    b"Couldn't map %d tag pages at %s for VMA %016lx\0" as *const u8
                        as *const libc::c_char,
                    n,
                    tag,
                    aligned_vma,
                );
            }
            aligned_vma = (aligned_vma as libc::c_ulong).wrapping_add((n * 0x2000) as libc::c_ulong)
                as isize as isize;
        }
        while VMAttributeTable[(virtualaddress >> 13) as usize] as usize & 0o100 as usize != 0
            && pages != 0
        {
            pages -= 1;
            virtualaddress = (virtualaddress as libc::c_ulong)
                .wrapping_add(0x2000 as usize as libc::c_ulong)
                as isize as isize;
            aligned_vma = (aligned_vma as libc::c_ulong)
                .wrapping_add(0x2000 as usize as libc::c_ulong) as isize
                as isize;
        }
    }
    return virtualaddress;
}

#[no_mangle]
pub fn DestroyVirtualAddress(mut vma: isize) -> isize {
    let mut data: caddr_t = "";
    let mut tag: caddr_t = "";
    let mut aligned_vma: isize = vma.wrapping_sub(vma & (0x2000 as usize - 1) as libc::c_ulong);
    if VMAttributeTable[(vma >> 13) as usize] & 0o100 as usize == 0 {
        return vma;
    }
    data = &mut *DataSpace.offset(aligned_vma as isize) as *mut isize as caddr_t;
    tag = &mut *TagSpace.offset(aligned_vma as isize) as *mut Tag as caddr_t;
    if munmap(
        data as *mut libc::c_void,
        ::std::mem::size_of::<[isize; 8192]>() as libc::c_ulong,
    ) != 0
    {
        printf(
            b"Couldn't unmap data page at %s for VMA %016lx\0" as *const u8 as *const libc::c_char,
            data,
            vma,
        );
    }
    if munmap(
        tag as *mut libc::c_void,
        ::std::mem::size_of::<[Tag; 8192]>() as libc::c_ulong,
    ) != 0
    {
        printf(
            b"Couldn't unmap tag page at %s for VMA %016lx\0" as *const u8 as *const libc::c_char,
            tag,
            vma,
        );
    }
    VMAttributeTable[(vma >> 13) as usize] = 0 as usize as VMAttribute;
    return vma;
}

#[no_mangle]
pub fn DestroyVirtualAddressRange(mut vma: isize, mut count: u32) -> isize {
    let mut pages: usize = (count + (0x2000 as usize - 1)) / 0x2000;
    loop {
        let fresh0 = pages;
        pages = pages - 1;
        if !(fresh0 != 0) {
            break;
        }
        DestroyVirtualAddress(vma);
        vma =
            (vma as libc::c_ulong).wrapping_add(0x2000 as usize as libc::c_ulong) as isize as isize;
    }
    return vma;
}

#[no_mangle]
pub fn MapVirtualAddressData(mut vma: isize) -> *mut isize {
    return &mut *DataSpace.offset(vma as isize) as *mut isize;
}

#[no_mangle]
pub fn MapVirtualAddressTag(mut vma: isize) -> *mut Tag {
    return &mut *TagSpace.offset(vma as isize) as *mut Tag;
}

#[no_mangle]
pub fn VirtualMemoryRead(mut vma: isize, mut object: *mut LispObj) -> usize {
    memory_vma = vma;
    (*object).parts.data.u = *DataSpace.offset(vma as isize) as ui32;
    (*object).parts.tag = *TagSpace.offset(vma as isize) as ui32;
    return 0;
}

#[no_mangle]
pub fn VirtualMemoryWrite(mut vma: isize, mut object: *mut LispObj) -> usize {
    memory_vma = vma;
    *DataSpace.offset(vma as isize) = (*object).parts.data.u as isize;
    *TagSpace.offset(vma as isize) = (*object).parts.tag as Tag;
    return 0;
}

#[no_mangle]
pub fn VirtualMemoryReadBlock(mut vma: isize, mut object: *mut LispObj, mut count: u32) -> usize {
    let mut data: *mut isize = &mut *DataSpace.offset(vma as isize) as *mut isize;
    let mut tag: *mut Tag = &mut *TagSpace.offset(vma as isize) as *mut Tag;
    let mut edata: *mut isize =
        &mut *DataSpace.offset(vma.wrapping_add(count as libc::c_ulong) as isize) as *mut isize;
    memory_vma = vma;
    while data < edata {
        let fresh1 = data;
        data = data.offset(1);
        (*object).parts.data.u = *fresh1 as ui32;
        let fresh2 = tag;
        tag = tag.offset(1);
        (*object).parts.tag = *fresh2 as ui32;
        object = object.offset(1);
        memory_vma = memory_vma.wrapping_add(1);
    }
    return 0;
}

#[no_mangle]
pub fn VirtualMemoryWriteBlock(mut vma: isize, mut object: *mut LispObj, mut count: u32) -> usize {
    let mut data: *mut isize = &mut *DataSpace.offset(vma as isize) as *mut isize;
    let mut tag: *mut Tag = &mut *TagSpace.offset(vma as isize) as *mut Tag;
    let mut edata: *mut isize =
        &mut *DataSpace.offset(vma.wrapping_add(count as libc::c_ulong) as isize) as *mut isize;
    memory_vma = vma;
    while data < edata {
        let fresh3 = data;
        data = data.offset(1);
        *fresh3 = (*object).parts.data.u as isize;
        let fresh4 = tag;
        tag = tag.offset(1);
        *fresh4 = (*object).parts.tag as Tag;
        object = object.offset(1);
        memory_vma = memory_vma.wrapping_add(1);
    }
    return 0;
}

#[no_mangle]
pub fn VirtualMemoryWriteBlockConstant(
    mut vma: isize,
    mut object: *mut LispObj,
    mut count: u32,
    mut increment: u32,
) -> usize {
    let mut data: *mut isize = &mut *DataSpace.offset(vma as isize) as *mut isize;
    let mut tag: *mut Tag = &mut *TagSpace.offset(vma as isize) as *mut Tag;
    let mut ctag: Tag = (*object).parts.tag as Tag;
    let mut cdata: isize = (*object).parts.data.u as isize;
    let mut edata: *mut isize =
        &mut *DataSpace.offset(vma.wrapping_add(count as libc::c_ulong) as isize) as *mut isize;
    memory_vma = vma;
    memset(
        tag as *mut libc::c_uchar as *mut libc::c_void,
        ctag,
        (count as libc::c_ulong).wrapping_mul(::std::mem::size_of::<Tag>() as libc::c_ulong),
    );
    match increment {
        0 => {
            if cdata == 0 {
                memset(
                    data as *mut libc::c_uchar as *mut libc::c_void,
                    0 as usize as libc::c_uchar,
                    (count as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<isize>() as libc::c_ulong),
                );
            } else {
                while data < edata {
                    let fresh5 = data.offset(1);
                    *fresh5 = cdata;
                    memory_vma = memory_vma.wrapping_add(1);
                }
            }
        }
        1 => {
            while data < edata {
                *data.offset(1) = cdata.wrapping_add(1);
                memory_vma = memory_vma.wrapping_add(1);
            }
        }
        _ => {
            while data < edata {
                let fresh8 = data;
                data = data.offset(1);
                *fresh8 = cdata;
                cdata = (cdata as libc::c_ulong).wrapping_add(increment as libc::c_ulong) as isize
                    as isize;
                memory_vma = memory_vma.wrapping_add(1);
            }
        }
    }
    return 0;
}

#[no_mangle]
pub fn VirtualMemorySearch(mut vma: *mut isize, mut object: *mut LispObj, mut count: u32) -> Boole {
    let mut tag: *mut Tag = &mut *TagSpace.offset(*vma as isize) as *mut Tag;
    let mut etag: *mut Tag =
        &mut *TagSpace.offset((*vma).wrapping_add(count as libc::c_ulong) as isize) as *mut Tag;
    let mut ctag: Tag = (*object).parts.tag as Tag;
    let mut cdata: isize = (*object).parts.data.u as isize;
    while tag < etag {
        tag = memchr(
            tag as *mut libc::c_uchar as *const libc::c_void,
            ctag,
            etag.offset_from(tag).wrapping_mul(::std::mem::size_of::<Tag>()),
        ) as *mut Tag;
        if tag.is_null() {
            return 0 as usize as Boole;
        }
        memory_vma = tag.offset_from(TagSpace) as libc::c_long as isize;
        if *DataSpace.offset(memory_vma as isize) == cdata {
            *vma = memory_vma;
            return 1 as usize as Boole;
        }
        tag = tag.offset(1);
    }
    return 0 as usize as Boole;
}

#[no_mangle]
pub fn VirtualMemoryCopy(
    mut from: isize,
    mut to: isize,
    mut count: u32,
    mut row: *mut Byte,
) -> usize {
    let mut fromdata: *mut isize = &mut *DataSpace.offset(from as isize) as *mut isize;
    let mut fromtag: *mut Tag = &mut *TagSpace.offset(from as isize) as *mut Tag;
    let mut etag: *mut Tag =
        &mut *TagSpace.offset(from.wrapping_add(count as libc::c_ulong) as isize) as *mut Tag;
    let mut todata: *mut isize = &mut *DataSpace.offset(to as isize) as *mut isize;
    let mut totag: *mut Tag = &mut *TagSpace.offset(to as isize) as *mut Tag;
    let mut obj: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    };
    let mut tag: Tag = 0;
    let mut action: usize = 0;
    memory_vma = from;
    if row == (MemoryActionTable[CycleRaw as usize as usize]).as_mut_ptr() {
        memmove(
            totag as *mut libc::c_uchar as *mut libc::c_void,
            fromtag as *mut libc::c_uchar as *const libc::c_void,
            (count as libc::c_ulong).wrapping_mul(::std::mem::size_of::<Tag>() as libc::c_ulong),
        );
        memmove(
            todata as *mut libc::c_uchar as *mut libc::c_void,
            fromdata as *mut libc::c_uchar as *const libc::c_void,
            (count as libc::c_ulong).wrapping_mul(::std::mem::size_of::<isize>() as libc::c_ulong),
        );
        return 0;
    }
    while fromtag < etag {
        tag = *fromtag;
        action = *row.offset(tag as isize);
        if action & (0o4 as usize | 0o10) == 0o4 as usize {
            if OldspaceAddressP(*fromdata) != 0 {
                TakeMemoryTrap(TransportTrapVector, *fromdata);
            }
        }
        if action != 0 {
            MemoryReadInternal(
                fromtag.offset_from(TagSpace) as libc::c_long as isize,
                &mut obj,
                row,
            );
            let fresh9 = totag;
            totag = totag.offset(1);
            *fresh9 = obj.parts.tag as Tag;
            let fresh10 = todata;
            todata = todata.offset(1);
            *fresh10 = obj.parts.data.u as isize;
            fromtag = fromtag.offset(1);
            fromdata = fromdata.offset(1);
        } else {
            let fresh11 = totag;
            totag = totag.offset(1);
            *fresh11 = tag;
            fromtag = fromtag.offset(1);
            let fresh12 = fromdata;
            fromdata = fromdata.offset(1);
            let fresh13 = todata;
            todata = todata.offset(1);
            *fresh13 = *fresh12;
        }
        memory_vma = memory_vma.wrapping_add(1);
    }
    return 0;
}

#[no_mangle]
pub fn VirtualMemoryScan(mut vma: *mut isize, mut count: u32) -> Boole {
    let mut attr: *mut VMAttribute =
        &mut *VMAttributeTable.as_mut_ptr().offset((*vma >> 13) as isize) as *mut VMAttribute;
    while count > 0 as usize {
        if *attr as usize & 0o4 as usize != 0 {
            let mut scanvma: isize =
                ((attr.offset_from(VMAttributeTable.as_mut_ptr()) as libc::c_long) << 13) as isize;
            let mut tag: *mut Tag = &mut *TagSpace.offset(scanvma as isize) as *mut Tag;
            let mut etag: *mut Tag = &mut *TagSpace.offset(
                scanvma
                    .wrapping_add((if (0x2000) < count { 0x2000 } else { count }) as libc::c_ulong)
                    as isize,
            ) as *mut Tag;
            while tag < etag {
                if 0xfff4fffff8f7 as libc::c_long & ((1) << *tag) as libc::c_long == 0
                    && OldspaceAddressP(
                        *DataSpace.offset(tag.offset_from(TagSpace) as libc::c_long as isize),
                    ) as usize
                        != 0
                {
                    *vma = tag.offset_from(TagSpace) as libc::c_long as isize;
                    return 1 as usize as Boole;
                }
                tag = tag.offset(1);
            }
        }
        attr = attr.offset(1);
        count -= 0x2000;
    }
    return 0 as usize as Boole;
}

#[no_mangle]
pub fn VirtualMemoryEnable(mut vma: isize, mut count: u32) {
    let mut attr: *mut VMAttribute =
        &mut *VMAttributeTable.as_mut_ptr().offset((vma >> 13) as isize) as *mut VMAttribute;
    let mut eattr: *mut VMAttribute = &mut *VMAttributeTable
        .as_mut_ptr()
        .offset((vma.wrapping_add(count as libc::c_ulong) >> 13) as isize)
        as *mut VMAttribute;
    while attr < eattr {
        let mut a: VMAttribute = *attr;
        if a as usize & 0o100 as usize != 0 && a as usize & 0o10 as usize == 0 {
            a = (a as usize | 0o4) as VMAttribute;
            *attr = a;
        }
        attr = attr.offset(1);
    }
}

#[no_mangle]
pub static mut VM: VMState = VMState {
    CommandRegister: 0,
    AddressRegister: 0,
    ExtentRegister: 0,
    AttributesRegister: 0,
    DestinationRegister: 0,
    DataRegister: LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    },
};

#[no_mangle]
pub fn VMCommand(mut command: u32) -> usize {
    let mut vm: *mut VMState = &mut VM;
    match (command >> 19 & ((1) << 13) - 1) as VMOpcode as libc::c_uint {
        0 => {
            let mut vpn: usize = ((*vm).AddressRegister >> 13);
            return ((if VMAttributeTable[vpn as usize] & 0o100 as usize != 0 {
                VMResultSuccess
            } else {
                VMResultFailure
            }) & ((1) << 13) - 1)
                << 19
                | vpn & !((((1) << 13) - 1) << 19);
        }
        1 => {
            EnsureVirtualAddressRange(
                (*vm).AddressRegister,
                (*vm).ExtentRegister,
                0 as usize as Boole,
            );
            return ((if 1 as usize != 0 {
                VMResultSuccess
            } else {
                VMResultFailure
            }) & ((1) << 13) - 1)
                << 19
                | 0 & !((((1) << 13) - 1) << 19);
        }
        2 => {
            DestroyVirtualAddressRange((*vm).AddressRegister, (*vm).ExtentRegister);
            return ((if 1 as usize != 0 {
                VMResultSuccess
            } else {
                VMResultFailure
            }) & ((1) << 13) - 1)
                << 19
                | 0 & !((((1) << 13) - 1) << 19);
        }
        3 => {
            let mut attr: VMAttribute = VMAttributeTable[(command >> 0 & ((1) << 19) - 1) as usize];
            if attr as usize & 0o100 as usize != 0 {
                (*vm).AttributesRegister =
                    VMAttributeTable[(command >> 0 & ((1) << 19) - 1) as usize] as isize;
                return ((if 1 as usize != 0 {
                    VMResultSuccess
                } else {
                    VMResultFailure
                }) & ((1) << 13) - 1)
                    << 19
                    | command & !((((1) << 13) - 1) << 19);
            } else {
                return ((if 0 as usize != 0 {
                    VMResultSuccess
                } else {
                    VMResultFailure
                }) & ((1) << 13) - 1)
                    << 19
                    | command & !((((1) << 13) - 1) << 19);
            }
        }
        4 => {
            let mut attr_0: VMAttribute =
                VMAttributeTable[(command >> 0 & ((1) << 19) - 1) as usize];
            if attr_0 as usize & 0o100 as usize != 0 {
                let ref mut fresh14 = (*vm).AttributesRegister;
                *fresh14 |= 0o100 as usize as libc::c_ulong;
                VMAttributeTable[(command >> 0 & ((1) << 19) - 1) as usize] =
                    *fresh14 as VMAttribute;
                return ((if 1 as usize != 0 {
                    VMResultSuccess
                } else {
                    VMResultFailure
                }) & ((1) << 13) - 1)
                    << 19
                    | command & !((((1) << 13) - 1) << 19);
            } else {
                return ((if 0 as usize != 0 {
                    VMResultSuccess
                } else {
                    VMResultFailure
                }) & ((1) << 13) - 1)
                    << 19
                    | command & !((((1) << 13) - 1) << 19);
            }
        }
        5 => {
            VirtualMemoryWriteBlockConstant(
                (*vm).AddressRegister,
                &mut (*vm).DataRegister,
                (*vm).ExtentRegister,
                command >> 0 & ((1) << 19) - 1,
            );
            return ((if 1 as usize != 0 {
                VMResultSuccess
            } else {
                VMResultFailure
            }) & ((1) << 13) - 1)
                << 19
                | 0 & !((((1) << 13) - 1) << 19);
        }
        6 => {
            let mut result: Boole = VirtualMemorySearch(
                &mut (*vm).AddressRegister,
                &mut (*vm).DataRegister,
                (*vm).ExtentRegister,
            );
            return ((if result as usize != 0 {
                VMResultSuccess
            } else {
                VMResultFailure
            }) & ((1) << 13) - 1)
                << 19
                | 0 & !((((1) << 13) - 1) << 19);
        }
        7 => {
            let mut result_0: Boole = VirtualMemoryCopy(
                (*vm).AddressRegister,
                (*vm).DestinationRegister,
                (*vm).ExtentRegister,
                (MemoryActionTable[(command >> 0 & ((1) << 19) - 1) as usize]).as_mut_ptr(),
            ) as Boole;
            return ((if result_0 as usize != 0 {
                VMResultSuccess
            } else {
                VMResultFailure
            }) & ((1) << 13) - 1)
                << 19
                | 0 & !((((1) << 13) - 1) << 19);
        }
        8 => {
            let mut result_1: Boole =
                VirtualMemoryScan(&mut (*vm).AddressRegister, (*vm).ExtentRegister);
            return ((if result_1 as usize != 0 {
                VMResultSuccess
            } else {
                VMResultFailure
            }) & ((1) << 13) - 1)
                << 19
                | 0 & !((((1) << 13) - 1) << 19);
        }
        9 => {
            VirtualMemoryEnable((*vm).AddressRegister, (*vm).ExtentRegister);
            return ((if 1 as usize != 0 {
                VMResultSuccess
            } else {
                VMResultFailure
            }) & ((1) << 13) - 1)
                << 19
                | 0 & !((((1) << 13) - 1) << 19);
        }
        _ => {}
    }
    panic!("Reached end of non-void function without returning");
}

fn ComputeProtection(mut attr: VMAttribute) -> usize {
    if attr as usize & 0o10 as usize != 0 {
        attr = (attr as usize & !(0o4)) as VMAttribute;
    }
    if attr & (0o100 as usize | 0o4 as usize | 0o1) != 0o100 {
        return 0;
    }
    if attr & (0o40 as usize | 0o20 as usize | 0o2) != 0o40 as usize | 0o20 {
        return 0x1 as usize | 0x4;
    }
    return 0x1 as usize | 0x2 as usize | 0x4;
}

#[no_mangle]
pub fn MapWorldLoad(
    mut vma: isize,
    mut length: u32,
    mut worldfile: u32,
    mut dataoffset: off_t,
    mut tagoffset: off_t,
) -> isize {
    let mut data: caddr_t = "";
    let mut tag: caddr_t = "";
    let mut attr: VMAttribute = (0o100 as usize
        | 0o20
        | (if 0 as usize != 0 {
            0o1 as usize
        } else {
            0 as usize
        })
        | (if EnableIDS as usize != 0 && 1 as usize != 0 {
            0
        } else {
            0o40
        })) as VMAttribute;
    let mut prot: usize = ComputeProtection(attr);
    let mut dataCount: size_t = 0;
    let mut tagCount: size_t = 0;
    let mut words: usize = 0;
    while length > 0 as usize {
        while length > 0
            && (vma & (((1) << 16) - 1) as libc::c_ulong != 0
                || VMAttributeTable[(vma >> 13) as usize] & 0o100 as usize != 0
                || length < (1) << 16)
        {
            words = (0o2000 as usize as libc::c_ulong)
                .wrapping_sub(vma & (0x2000 as usize - 1) as libc::c_ulong);
            if words > length {
                words = length;
            }
            EnsureVirtualAddress(vma);
            dataCount = (::std::mem::size_of::<isize>() as libc::c_ulong)
                .wrapping_mul(words as libc::c_ulong);
            if dataoffset != lseek(worldfile, dataoffset, 0) {
                vpunt(
                    "",
                    b"Unable to seek to data offset %d in world file\0" as *const u8
                        as *const libc::c_char as &str,
                    dataoffset,
                );
            }
            if dataCount
                != read(
                    worldfile,
                    MapVirtualAddressData(vma) as *mut libc::c_void,
                    dataCount,
                ) as libc::c_ulong
            {
                vpunt(
                    "",
                    b"Unable to read data page %d from world file\0" as *const u8
                        as *const libc::c_char as &str,
                    vma >> 13,
                );
            }
            tagCount = (::std::mem::size_of::<Tag>() as libc::c_ulong)
                .wrapping_mul(words as libc::c_ulong);
            if tagoffset != lseek(worldfile, tagoffset, 0) {
                vpunt(
                    "",
                    b"Unable to seek to tag offset %d in world file\0" as *const u8
                        as *const libc::c_char as &str,
                    tagoffset,
                );
            }
            if tagCount
                != read(
                    worldfile,
                    MapVirtualAddressTag(vma) as *mut libc::c_void,
                    tagCount,
                ) as libc::c_ulong
            {
                vpunt(
                    "",
                    b"Unable to read tag page %d from world file\0" as *const u8
                        as *const libc::c_char as &str,
                    vma >> 13,
                );
            }
            VMAttributeTable[(vma >> 13) as usize] = (0o1 as usize | 0o4 | 0o100) as VMAttribute;
            vma = (vma as libc::c_ulong).wrapping_add(words as libc::c_ulong) as isize as isize;
            dataoffset = (dataoffset as libc::c_ulong).wrapping_add(dataCount) as off_t as off_t;
            tagoffset = (tagoffset as libc::c_ulong).wrapping_add(tagCount) as off_t as off_t;
            length -= words;
            unmapped_world_words += words;
        }
        swap_map_entries += 1;
        if length > 0 as usize {
            let mut limit: usize = length - (length & ((1) << 16) - 1);
            words = 0;
            while words < limit
                && *(VMAttributeTable.as_mut_ptr() as *mut int64_t)
                    .offset((vma.wrapping_add(words as libc::c_ulong) >> 16) as isize)
                    & 0x4040404040404040 as libc::c_long
                    == 0
            {
                let mut wadlimit: usize = words + ((1) << 16);
                let mut pattr: *mut VMAttribute = &mut *VMAttributeTable
                    .as_mut_ptr()
                    .offset((vma.wrapping_add(words as libc::c_ulong) >> 13) as isize)
                    as *mut VMAttribute;
                while words < wadlimit {
                    *pattr = attr;
                    words += 0o2000;
                    pattr = pattr.offset(1);
                }
            }
            data = &mut *DataSpace.offset(vma as isize) as *mut isize as caddr_t;
            tag = &mut *TagSpace.offset(vma as isize) as *mut Tag as caddr_t;
            dataCount = (::std::mem::size_of::<isize>() as libc::c_ulong)
                .wrapping_mul(words as libc::c_ulong);
            if data
                != mmap(
                    data as *mut libc::c_void,
                    dataCount,
                    0x1 as usize | 0x2 as usize | 0x4,
                    0 as usize | 0x2 as usize | 0x10,
                    worldfile,
                    dataoffset,
                ) as caddr_t
            {
                vpunt(
                    "",
                    b"Couldn't map %d world data pages at %lx for VMA %x\0" as *const u8
                        as *const libc::c_char as &str,
                    words >> 13,
                    data,
                    vma,
                );
            }
            tagCount = (::std::mem::size_of::<Tag>() as libc::c_ulong)
                .wrapping_mul(words as libc::c_ulong);
            if tag
                != mmap(
                    tag as *mut libc::c_void,
                    tagCount,
                    prot,
                    0 as usize | 0x2 as usize | 0x10,
                    worldfile,
                    tagoffset,
                ) as caddr_t
            {
                vpunt(
                    "",
                    b"Couldn't map %d world tag pages at %lx for VMA %x\0" as *const u8
                        as *const libc::c_char as &str,
                    words >> 13,
                    tag,
                    vma,
                );
            }
            vma = (vma as libc::c_ulong).wrapping_add(words as libc::c_ulong) as isize as isize;
            dataoffset = (dataoffset as libc::c_ulong).wrapping_add(dataCount) as off_t as off_t;
            tagoffset = (tagoffset as libc::c_ulong).wrapping_add(tagCount) as off_t as off_t;
            length -= words;
            mapped_world_words += words;
            file_map_entries += 2;
        }
    }
    return vma;
}
