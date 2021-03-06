#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Cdr {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct Data2R {
    bits: u16,
}
impl Data2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Data1R {
    bits: u16,
}
impl Data1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 16:31 - 2nd data item of a pair of regular conversions"]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        Data2R { bits }
    }
    #[doc = "Bits 0:15 - 1st data item of a pair of regular conversions"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        Data1R { bits }
    }
}
