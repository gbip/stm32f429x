#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OtgHsGotgctl {
    #[doc = r" Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = r" Value of the field"]
pub struct SrqscsR {
    bits: u8,
}
impl SrqscsR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SrqR {
    bits: u8,
}
impl SrqR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HngscsR {
    bits: u8,
}
impl HngscsR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HnprqR {
    bits: u8,
}
impl HnprqR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HshnpenR {
    bits: u8,
}
impl HshnpenR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DhnpenR {
    bits: u8,
}
impl DhnpenR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CidstsR {
    bits: u8,
}
impl CidstsR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DbctR {
    bits: u8,
}
impl DbctR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AsvldR {
    bits: u8,
}
impl AsvldR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BsvldR {
    bits: u8,
}
impl BsvldR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SrqW<'a> {
    w: &'a mut W,
}
impl<'a> _SrqW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HnprqW<'a> {
    w: &'a mut W,
}
impl<'a> _HnprqW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HshnpenW<'a> {
    w: &'a mut W,
}
impl<'a> _HshnpenW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DhnpenW<'a> {
    w: &'a mut W,
}
impl<'a> _DhnpenW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Session request success"]
    #[inline(always)]
    pub fn srqscs(&self) -> SrqscsR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SrqscsR { bits }
    }
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&self) -> SrqR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SrqR { bits }
    }
    #[doc = "Bit 8 - Host negotiation success"]
    #[inline(always)]
    pub fn hngscs(&self) -> HngscsR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HngscsR { bits }
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&self) -> HnprqR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HnprqR { bits }
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&self) -> HshnpenR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HshnpenR { bits }
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DhnpenR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DhnpenR { bits }
    }
    #[doc = "Bit 16 - Connector ID status"]
    #[inline(always)]
    pub fn cidsts(&self) -> CidstsR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CidstsR { bits }
    }
    #[doc = "Bit 17 - Long/short debounce time"]
    #[inline(always)]
    pub fn dbct(&self) -> DbctR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DbctR { bits }
    }
    #[doc = "Bit 18 - A-session valid"]
    #[inline(always)]
    pub fn asvld(&self) -> AsvldR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AsvldR { bits }
    }
    #[doc = "Bit 19 - B-session valid"]
    #[inline(always)]
    pub fn bsvld(&self) -> BsvldR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BsvldR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 2048 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&mut self) -> _SrqW {
        _SrqW { w: self }
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&mut self) -> _HnprqW {
        _HnprqW { w: self }
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&mut self) -> _HshnpenW {
        _HshnpenW { w: self }
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&mut self) -> _DhnpenW {
        _DhnpenW { w: self }
    }
}
