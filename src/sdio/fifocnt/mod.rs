#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Fifocnt {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FifocountR {
    bits: u32,
}
impl FifocountR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:23 - Remaining number of words to be written to or read from the FIFO."]
    #[inline(always)]
    pub fn fifocount(&self) -> FifocountR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        FifocountR { bits }
    }
}
