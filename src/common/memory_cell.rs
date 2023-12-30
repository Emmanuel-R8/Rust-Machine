use std::ops::{ Add, AddAssign, Neg, Sub, SubAssign };

use super::{
    constants::{ QTag, ADDRESS_NIL, ADDRESS_T, CDR, QTAG_FIXNUM, QTAG_SINGLEFLOAT },
    types::Address,
};

#[derive(Debug)]
pub struct MemoryCell {
    cdr_tag: u8, // 3 bits for cdr and 5 bits for tag
    half_word1: u16, // 16 bits
    half_word2: u16, // 16 bits
}

impl MemoryCell {
    pub fn new(cdr: u8, tag: u8, half_word1: u16, half_word2: u16) -> Self {
        assert!(
            cdr <= 0b0000_0111,
            "cdr must be 3 bits (called with cdr: {cdr:#05o} / {cdr:#010b}, tag: {tag:#05o} / {tag:#010b}, half_word1: {half_word1:#010x}, half_word2: {half_word2:#010x})"
        ); // make sure cdr is within 3 bits
        assert!(
            tag <= 0b0011_1111,
            "tag must be 6 bits (called with cdr: {cdr:#05o} / {cdr:#010b}, tag: {tag:#05o} / {tag:#010b}, half_word1: {half_word1:#010x}, half_word2: {half_word2:#010x})"
        ); // make sure tag is within 6 bits

        // pack cdr and tag into one byte
        let cdr_tag = (cdr << 6) | tag;
        return MemoryCell {
            cdr_tag,
            half_word1,
            half_word2,
        };
    }

    pub fn new_cdr_tag_u(cdr: CDR, tag: QTag, u: u32) -> Self {
        let cdr = cdr as u8;
        let tag = tag as u8;
        let half_word1 = ((u | 0xffff_0000) >> 16) as u16;
        let half_word2 = (u & 0x0000_ffff) as u16;
        return MemoryCell::new(cdr, tag, half_word1, half_word2);
    }

    pub fn new_cdr_tag_i(cdr: CDR, tag: QTag, i: i32) -> Self {
        let u = unsafe { std::mem::transmute::<i32, u32>(i) };
        return MemoryCell::new_cdr_tag_u(cdr, tag, u);
    }

    pub fn new_cdr_tag_f(cdr: CDR, tag: QTag, f: f32) -> Self {
        let u = f32::to_bits(f);
        MemoryCell::new_cdr_tag_u(cdr, tag, u)
    }

    pub fn new_cdr_tag_a(cdr: CDR, tag: QTag, a: u32) -> Self {
        MemoryCell::new_cdr_tag_u(cdr, tag, a)
    }

    pub fn cdr(&self) -> u8 {
        self.cdr_tag >> 6 // retrieve cdr
    }

    pub fn set_cdr(&mut self, cdr: CDR) {
        self.cdr_tag = ((cdr as u8) << 6) | (self.cdr_tag & 0b0011_1111);
    }

    pub fn tag(&self) -> u8 {
        self.cdr_tag & 0b0011_1111
    }

    pub fn set_tag(&mut self, tag: QTag) -> &Self {
        self.cdr_tag = (self.cdr() << 6) | ((tag as u8) & 0b0011_1111);
        return self;
    }

    // Form a i32 from the 2 half-words
    pub fn as_i32(&self) -> Option<i32> {
        if self.tag() == (QTag::Fixnum as u8) {
            let mut bits = self.half_word1 as u32;
            bits |= (self.half_word2 as u32) << 16;
            return Some(unsafe { std::mem::transmute::<u32, i32>(bits) });
        } else {
            return None;
        }
    }

    pub fn set_i32(&mut self, val: i32) -> &Self {
        let u = unsafe { std::mem::transmute::<i32, u32>(val) };
        let h1 = ((u & 0xffff_0000) >> 16) as u16;
        let h2 = (u & 0x0000_ffff) as u16;
        self.half_word1 = h1;
        self.half_word2 = h2;

        return self;
    }

    // Form a f32 from the 2 half-words
    pub fn as_f32(&self) -> Option<f32> {
        if self.tag() == (QTag::SingleFloat as u8) {
            let mut bits = (self.half_word1 as u32) << 16;
            bits |= self.half_word2 as u32;
            return Some(f32::from_bits(bits));
        } else {
            return None;
        }
    }

    pub fn set_f32(&mut self, val: f32) -> &Self {
        let u = f32::to_bits(val);
        let h1 = ((u & 0xffff_0000) >> 16) as u16;
        let h2 = (u & 0x0000_ffff) as u16;
        self.half_word1 = h1;
        self.half_word2 = h2;

        return self;
    }

    pub fn as_raw(&self) -> u32 {
        let mut bits = (self.half_word1 as u32) << 16;
        bits |= self.half_word2 as u32;
        return bits;
    }

    pub fn set_raw(&mut self, val: u32) -> &Self {
        let h1 = ((val & 0xffff_0000) >> 16) as u16;
        let h2 = (val & 0x0000_ffff) as u16;
        self.half_word1 = h1;
        self.half_word2 = h2;

        return self;
    }

    pub fn as_address(&self) -> Address {
        let mut bits = (self.half_word1 as u32) << 16;
        bits |= self.half_word2 as u32;
        return bits as Address;
    }

    pub fn set_address(&mut self, val: Address) -> &Self {
        let h1 = ((val & 0xffff_0000) >> 16) as u16;
        let h2 = (val & 0x0000_ffff) as u16;
        self.half_word1 = h1;
        self.half_word2 = h2;

        return self;
    }

    // Check constants
    pub fn is_t(&self) -> bool {
        self.as_raw() == ADDRESS_T
    }

    pub fn is_nil(&self) -> bool {
        self.as_raw() == ADDRESS_NIL
    }

    //
    // Basic arithmetic
    //

    // Non mutating
    pub fn inc(self) -> Self {
        match self.tag() {
            QTAG_FIXNUM => {
                let mut m = self.clone();
                let i = self.as_i32().unwrap() + 1;
                m.set_i32(i);
                return m;
            }
            QTAG_SINGLEFLOAT => {
                let mut m = self.clone();
                let f = self.as_f32().unwrap() + 1.0;
                m.set_f32(f);
                return m;
            }
            _ => todo!(),
        }
    }

    // Non mutating
    pub fn dec(self) -> Self {
        match self.tag() {
            QTAG_FIXNUM => {
                let mut m = self.clone();
                let i = self.as_i32().unwrap() - 1;
                m.set_i32(i);
                return m;
            }
            QTAG_SINGLEFLOAT => {
                let mut m = self.clone();
                let f = self.as_f32().unwrap() - 1.0;
                m.set_f32(f);
                return m;
            }
            _ => todo!(),
        }
    }

    // Mutating
    pub fn inc_mut(&mut self) -> &Self {
        match self.tag() {
            QTAG_FIXNUM => {
                let i = self.as_i32().unwrap() + 1;
                self.set_i32(i);
            }
            QTAG_SINGLEFLOAT => {
                let f = self.as_f32().unwrap() + 1.0;
                self.set_f32(f);
            }
            _ => todo!(),
        }

        return self;
    }

    // Non mutating
    pub fn dec_mut(&mut self) -> &Self {
        match self.tag() {
            QTAG_FIXNUM => {
                let i = self.as_i32().unwrap() - 1;
                self.set_i32(i);
            }
            QTAG_SINGLEFLOAT => {
                let f = self.as_f32().unwrap() - 1.0;
                self.set_f32(f);
            }
            _ => todo!(),
        }

        return self;
    }
}

impl Default for MemoryCell {
    fn default() -> Self {
        MemoryCell::new_cdr_tag_u(CDR::Normal, QTag::Fixnum, 0)
    }
}

impl Clone for MemoryCell {
    fn clone(&self) -> Self {
        let r = self.as_raw();
        let h1 = ((r & 0xffff_0000) >> 16) as u16;
        let h2 = (r & 0x0000_ffff) as u16;
        MemoryCell::new(self.cdr(), self.tag(), h1, h2)
    }
}

impl Copy for MemoryCell {}

// Implement PartialEq and`eq`for MemoryCell
// todo: tHIS PROBABLY ONLY WORKS FOR PRIMITIVE TYPES.
impl PartialEq for MemoryCell {
    fn eq(&self, other: &Self) -> bool {
        self.tag() == other.tag() &&
            self.half_word1 == other.half_word1 &&
            self.half_word2 == other.half_word2
    }
}

impl Eq for MemoryCell {}

impl Neg for MemoryCell {
    type Output = Option<Self>;

    fn neg(self) -> Self::Output {
        if self.tag() == (QTag::Fixnum as u8) {
            let val_i32 = -self.as_i32().unwrap();
            let val_u32 = unsafe { std::mem::transmute::<i32, u32>(val_i32) };

            return Some(
                MemoryCell::new(
                    self.cdr(),
                    self.tag(),
                    ((val_u32 & 0xffff_0000) >> 16) as u16,
                    (val_u32 & 0x0000_ffff) as u16
                )
            );
        }

        return None;
    }
}

// Implements add for 2 MemoryCell where both contain the same type
// The content is u32 if the TAG is equal to QTag::Fixnum
// The content is u32 if the TAG is equal to QTag::Fixnum
// The content is u32 if the TAG is equal to QTag::Fixnum
impl Add for MemoryCell {
    type Output = Option<Self>;
    // TODO: Check big or low endian representation
    fn add(self, rhs: Self) -> Option<Self> {
        if self.tag() == (QTag::Fixnum as u8) && rhs.tag() == (QTag::Fixnum as u8) {
            let val_i32 = self.as_i32().unwrap() + rhs.as_i32().unwrap();
            let val_u32 = unsafe { std::mem::transmute::<i32, u32>(val_i32) };

            return Some(
                MemoryCell::new(
                    self.cdr(),
                    self.tag(),
                    ((val_u32 & 0xffff_0000) >> 16) as u16,
                    (val_u32 & 0x0000_ffff) as u16
                )
            );
        }

        if self.tag() == (QTag::SingleFloat as u8) && rhs.tag() == (QTag::SingleFloat as u8) {
            let val_f32 = self.as_f32().unwrap() + rhs.as_f32().unwrap();
            let val_u32 = f32::to_bits(val_f32);
            return Some(
                MemoryCell::new(
                    self.cdr(),
                    self.tag(),
                    ((val_u32 & 0xffff_0000) >> 16) as u16,
                    (val_u32 & 0x0000_ffff) as u16
                )
            );
        }

        return None;
    }
}

// Implements sub assign for 2 MemoryCell where both contain the same type
// The content is i32 if the TAG is equal to QTag::Fixnum
// The content is f32 if the TAG is equal to QTag::Singlefloat
impl Sub for MemoryCell {
    type Output = Option<Self>;
    // TODO: Check big or low endian representation
    fn sub(self, rhs: Self) -> Option<Self> {
        if self.tag() == (QTag::Fixnum as u8) && rhs.tag() == (QTag::Fixnum as u8) {
            let val_i32 = self.as_i32().unwrap() - rhs.as_i32().unwrap();
            let val_u32 = unsafe { std::mem::transmute::<i32, u32>(val_i32) };
            return Some(
                MemoryCell::new(
                    self.cdr(),
                    self.tag(),
                    ((val_u32 & 0xffff_0000) >> 16) as u16,
                    (val_u32 & 0x0000_ffff) as u16
                )
            );
        }

        if self.tag() == (QTag::SingleFloat as u8) && rhs.tag() == (QTag::SingleFloat as u8) {
            let val = self.as_f32().unwrap() - rhs.as_f32().unwrap();
            let val_u32 = f32::to_bits(val);
            return Some(
                MemoryCell::new(
                    self.cdr(),
                    self.tag(),
                    ((val_u32 & 0xffff_0000) >> 16) as u16,
                    (val_u32 & 0x0000_ffff) as u16
                )
            );
        }

        return None;
    }
}

// Implements add assign for 2 MemoryCell where both contain the same type
// The content is u32 if the TAG is equal to QTag::Fixnum
// The content is u32 if the TAG is equal to QTag::Fixnum
// The content is u32 if the TAG is equal to QTag::Fixnum
impl AddAssign for MemoryCell {
    // TODO: Check big or low endian representation
    fn add_assign(&mut self, rhs: Self) {
        if self.tag() == (QTag::Fixnum as u8) && rhs.tag() == (QTag::Fixnum as u8) {
            let val_i32 = self.as_i32().unwrap() + rhs.as_i32().unwrap();
            let val_u32 = unsafe { std::mem::transmute::<i32, u32>(val_i32) };
            *self = MemoryCell::new(
                self.cdr(),
                self.tag(),
                ((val_u32 & 0xffff_0000) >> 16) as u16,
                (val_u32 & 0x0000_ffff) as u16
            );
            return;
        }

        if self.tag() == (QTag::SingleFloat as u8) && rhs.tag() == (QTag::SingleFloat as u8) {
            let val_f32 = self.as_f32().unwrap() + rhs.as_f32().unwrap();
            let val_u32 = f32::to_bits(val_f32);
            *self = MemoryCell::new(
                self.cdr(),
                self.tag(),
                ((val_u32 & 0xffff_0000) >> 16) as u16,
                (val_u32 & 0x0000_ffff) as u16
            );
            return;
        }
    }
}

// Implements sub assign for 2 MemoryCell where both contain the same type
// The content is i32 if the TAG is equal to QTag::Fixnum
// The content is f32 if the TAG is equal to QTag::Singlefloat
impl SubAssign for MemoryCell {
    // TODO: Check big or low endian representation
    fn sub_assign(&mut self, rhs: Self) {
        if self.tag() == (QTag::Fixnum as u8) && rhs.tag() == (QTag::Fixnum as u8) {
            let val_i32 = self.as_i32().unwrap() - rhs.as_i32().unwrap();
            let val_u32 = unsafe { std::mem::transmute::<i32, u32>(val_i32) };
            *self = MemoryCell::new(
                self.cdr(),
                self.tag(),
                ((val_u32 & 0xffff_0000) >> 16) as u16,
                (val_u32 & 0x0000_ffff) as u16
            );
            return;
        }

        if self.tag() == (QTag::SingleFloat as u8) && rhs.tag() == (QTag::SingleFloat as u8) {
            let val = self.as_f32().unwrap() - rhs.as_f32().unwrap();
            let val_u32 = f32::to_bits(val);
            *self = MemoryCell::new(
                self.cdr(),
                self.tag(),
                ((val_u32 & 0xffff_0000) >> 16) as u16,
                (val_u32 & 0x0000_ffff) as u16
            );
            return;
        }
    }
}
