use crate::common::constants::{QTag, ADDRESS_NIL, ADDRESS_T, MEMORY_ADDRESS_PAGE_SHIFT, MEMORY_PAGE_MASK, VMAttribute, VMResultCode};
use crate::common::types::{LispQ, QData, QTagdata};
use crate::utils::{dpb, ldb};

// Constants
const OBJECT_T: LispQ = LispQ {
    parts: QTagdata {
        tag: QTag::Symbol,
        data: QData { u: ADDRESS_T },
    },
};
const OBJECT_NIL: LispQ = LispQ {
    parts: QTagdata {
        tag: QTag::NIL,
        data: QData { u: ADDRESS_NIL },
    },
};
const OBJECT_CDR_MASK: LispQ = LispQ {
    parts: QTagdata {
        tag: QTag::TagCdrMask,
        data: QData { u: 0 },
    },
};


pub fn memory_page_number(vma: u32) -> u32 {
    return vma >> MEMORY_ADDRESS_PAGE_SHIFT
}
pub fn memory_page_offset(vma: u32) -> u32 {
    return vma & MEMORY_PAGE_MASK
}
pub fn page_number_memory(vpn: u32) -> u32 {
    return vpn << MEMORY_ADDRESS_PAGE_SHIFT
}

pub fn access_fault(vma: &VMAttribute) -> bool {
    return vma & VMAttribute::AccessFault != 0
}
pub fn write_fault(vma: &VMAttribute) -> bool {
   return  vma & VMAttribute::WriteFault != 0
}
pub fn transport_fault(vma: &VMAttribute) -> bool {
   return  vma & VMAttribute::TransportFault != 0
}
pub fn transport_disable(vma: &VMAttribute) -> bool {
 return    vma & VMAttribute::TransportDisable != 0
}
pub fn ephemeral(vma: &VMAttribute) -> bool {
   return  vma & VMAttribute::Ephemeral > 0
}
pub fn modified(vma: &VMAttribute) -> bool {
 return    vma & VMAttribute::Modified != 0
}
pub fn exists(vma: &VMAttribute) -> bool {
   return  vma & VMAttribute::Exists != 0
}

pub fn set_vmaccess_fault(mut vma: VMAttribute) {
   return  vma = vma | VMAttribute::AccessFault
}
pub fn set_vmwrite_fault(mut vma: VMAttribute) {
 return    vma = vma | VMAttribute::WriteFault
}
pub fn set_vmtransport_fault(mut vma: VMAttribute) {
  return   vma = vma | VMAttribute::TransportFault
}
pub fn set_vmtransport_disable(mut vma: VMAttribute) {
  return   vma = vma | VMAttribute::TransportDisable
}
pub fn set_vmephemeral(mut vma: VMAttribute) {
 return    vma = vma | VMAttribute::Ephemeral
}
pub fn set_vmmodified(mut vma: VMAttribute) {
  return   vma = vma | VMAttribute::Modified
}
pub fn set_vmexists(mut vma: VMAttribute) {
  return   vma = vma | VMAttribute::Exists
}

pub fn clear_vmaccess_fault(mut vma: VMAttribute) {
 return    vma = vma & (VMAttribute::AccessFault ^ 0b1111_1111)
}
pub fn clear_vmwrite_fault(mut vma: VMAttribute) {
 return    vma = vma & (VMAttribute::WriteFault ^ 0b1111_1111)
}
pub fn clear_vmtransport_faultlt(mut vma: VMAttribute) {
 return    vma = vma & (VMAttribute::TransportFault ^ 0b1111_1111)
}
pub fn clear_vmtransport_disable(mut vma: VMAttribute) {
 return    vma = vma & (VMAttribute::TransportDisable ^ 0b1111_1111)
}
pub fn clear_vmephemeral(mut vma: VMAttribute) {
 return    vma = vma & (VMAttribute::Ephemeral ^ 0b1111_1111)
}
pub fn clear_vmmodified(mut vma: VMAttribute) {
 return    vma = vma & (VMAttribute::Modified ^ 0b1111_1111)
}
pub fn clear_vmexists(mut vma: VMAttribute) {
 return    vma = vma & (VMAttribute::Exists ^ 0b1111_1111)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VMMemory {
    VMAttribute::table: [Vec<VMAttribute>; 1 << (32 - MEMORY_ADDRESS_PAGE_SHIFT)],
}

static ENABLE_IDS: bool = false;

pub fn default_attributes(faultp: bool, worldp: bool) -> u8 {
    let mut accessfaultp: u8 = 0;
    if faultp {
        accessfaultp = VMAttribute::AccessFault;
    } else {
        accessfaultp = 0;
    }

    let mut modifiedp: u8 = 0;
    if ENABLE_IDS && worldp {
        modifiedp = 0;
    } else {
        modifiedp = VMAttribute::Modified;
    }

   return  VMAttribute::Exists | VMAttribute::Modified | accessfaultp | modifiedp
}

pub fn vmcommand_opcode(command: u32) -> u32 {
 return    ldb(13, 19, command)
}

pub fn vmcommand_operand(command: u32) -> u32 {
 return    ldb(19, 0, command)
}

pub fn set_vmreply_result(reply: u32, result: u32) -> u32 {
    let r: u32;

    if result != 0 {
        r = VMResultCode::Success as u32;
    } else {
        r = VMResultCode::Failure as u32;
    }

   return  dpb(r, 13, 19, reply)
}
