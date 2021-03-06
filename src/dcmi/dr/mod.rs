#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Dr {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct Byte3R {
    bits: u8,
}
impl Byte3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Byte2R {
    bits: u8,
}
impl Byte2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Byte1R {
    bits: u8,
}
impl Byte1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Byte0R {
    bits: u8,
}
impl Byte0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn byte3(&self) -> Byte3R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Byte3R { bits }
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn byte2(&self) -> Byte2R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Byte2R { bits }
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn byte1(&self) -> Byte1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Byte1R { bits }
    }
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn byte0(&self) -> Byte0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Byte0R { bits }
    }
}
