#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Rdt1r {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TimeR {
    bits: u16,
}
impl TimeR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FmiR {
    bits: u8,
}
impl FmiR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DlcR {
    bits: u8,
}
impl DlcR {
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
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&self) -> TimeR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TimeR { bits }
    }
    #[doc = "Bits 8:15 - FMI"]
    #[inline(always)]
    pub fn fmi(&self) -> FmiR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FmiR { bits }
    }
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DlcR { bits }
    }
}
