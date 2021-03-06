#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Sr {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FneR {
    bits: u8,
}
impl FneR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VsyncR {
    bits: u8,
}
impl VsyncR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HsyncR {
    bits: u8,
}
impl HsyncR {
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
    #[doc = "Bit 2 - FIFO not empty"]
    #[inline(always)]
    pub fn fne(&self) -> FneR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FneR { bits }
    }
    #[doc = "Bit 1 - VSYNC"]
    #[inline(always)]
    pub fn vsync(&self) -> VsyncR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VsyncR { bits }
    }
    #[doc = "Bit 0 - HSYNC"]
    #[inline(always)]
    pub fn hsync(&self) -> HsyncR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HsyncR { bits }
    }
}
