//////////////////////////////////////////////////////////////////////////////////////////////////
/// Instruction formats
/// IMAS p 80. +
///
use c2rust_bitfields::BitfieldStruct;


#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct FullWord {
    #[bitfield(name = "address", ty = "u64", bits = "0..=31")]
    #[bitfield(name = "N", ty = "u8", bits = "32..=34")]
    #[bitfield(name = "S", ty = "u8", bits = "35..=37")]
    #[bitfield(name = "CDR", ty = "u8", bits = "38..=39")]
    pub fullword:  [u8; 8],
}

#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Entry {
    #[bitfield(name = "address", ty = "u64", bits = "0..=31")]
    #[bitfield(name = "N", ty = "u32", bits = "32..=34")]
    #[bitfield(name = "S", ty = "u32", bits = "35..=37")]
    #[bitfield(name = "CDR", ty = "u64", bits = "38..=39")]
    pub entry:  [u8; 8],
}

#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct PackedHalf {
    #[bitfield(name = "instr_even", ty = "u32", bits = "0..=17")]
    #[bitfield(name = "instr_odd", ty = "u32", bits = "18..=35")]
    #[bitfield(name = "N", ty = "u32", bits = "32..=34")]
    #[bitfield(name = "S", ty = "u32", bits = "35..=37")]
    #[bitfield(name = "CDR", ty = "u64", bits = "38..=39")]
    pub packed_half: [u8; 8],
}

// Packed instructions formats
#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Immediate10Bits {
    #[bitfield(name = "operand", ty = "u32", bits = "0..=9")]
    #[bitfield(name = "tbd", ty = "u32", bits = "10..=17")]
    pub immediate_10_bits: [u8; 4],
}

#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Branch {
    #[bitfield(name = "offset", ty = "u8", bits = "0..=9")]
    #[bitfield(name = "tbd", ty = "u32", bits = "10..=17")]
    pub branch: [u8; 4],
}

#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct FieldExtraction {
    #[bitfield(name = "bottom", ty = "u8", bits = "0..=4")] //location of the bottom bit, i.e.rotate count (>> N)
    #[bitfield(name = "size", ty = "u8", bits = "5..=9")]  // field size - 1
    #[bitfield(name = "tbd", ty = "u32", bits = "10..=17")]
    pub extraction: [u8; 4],
}
