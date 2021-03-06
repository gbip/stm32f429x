#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Ptptslr {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct StssR {
    bits: u32,
}
impl StssR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct StpnsR {
    bits: u8,
}
impl StpnsR {
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
    #[doc = "Bits 0:30 - no description available"]
    #[inline(always)]
    pub fn stss(&self) -> StssR {
        let bits = {
            const MASK: u32 = 2147483647;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        StssR { bits }
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn stpns(&self) -> StpnsR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        StpnsR { bits }
    }
}
