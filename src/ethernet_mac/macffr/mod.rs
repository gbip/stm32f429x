#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Macffr {
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
pub struct PmR {
    bits: u8,
}
impl PmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HuR {
    bits: u8,
}
impl HuR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HmR {
    bits: u8,
}
impl HmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DaifR {
    bits: u8,
}
impl DaifR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RamR {
    bits: u8,
}
impl RamR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BfdR {
    bits: u8,
}
impl BfdR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PcfR {
    bits: u8,
}
impl PcfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SaifR {
    bits: u8,
}
impl SaifR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SafR {
    bits: u8,
}
impl SafR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HpfR {
    bits: u8,
}
impl HpfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RaR {
    bits: u8,
}
impl RaR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PmW<'a> {
    w: &'a mut W,
}
impl<'a> _PmW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HuW<'a> {
    w: &'a mut W,
}
impl<'a> _HuW<'a> {
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
pub struct _HmW<'a> {
    w: &'a mut W,
}
impl<'a> _HmW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DaifW<'a> {
    w: &'a mut W,
}
impl<'a> _DaifW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RamW<'a> {
    w: &'a mut W,
}
impl<'a> _RamW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BfdW<'a> {
    w: &'a mut W,
}
impl<'a> _BfdW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PcfW<'a> {
    w: &'a mut W,
}
impl<'a> _PcfW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SaifW<'a> {
    w: &'a mut W,
}
impl<'a> _SaifW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SafW<'a> {
    w: &'a mut W,
}
impl<'a> _SafW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HpfW<'a> {
    w: &'a mut W,
}
impl<'a> _HpfW<'a> {
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
pub struct _RaW<'a> {
    w: &'a mut W,
}
impl<'a> _RaW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PmR { bits }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn hu(&self) -> HuR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HuR { bits }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn hm(&self) -> HmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HmR { bits }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn daif(&self) -> DaifR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DaifR { bits }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RamR { bits }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bfd(&self) -> BfdR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BfdR { bits }
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn pcf(&self) -> PcfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PcfR { bits }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn saif(&self) -> SaifR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SaifR { bits }
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn saf(&self) -> SafR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SafR { bits }
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn hpf(&self) -> HpfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HpfR { bits }
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RaR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn pm(&mut self) -> _PmW {
        _PmW { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn hu(&mut self) -> _HuW {
        _HuW { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn hm(&mut self) -> _HmW {
        _HmW { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn daif(&mut self) -> _DaifW {
        _DaifW { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn ram(&mut self) -> _RamW {
        _RamW { w: self }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bfd(&mut self) -> _BfdW {
        _BfdW { w: self }
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn pcf(&mut self) -> _PcfW {
        _PcfW { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn saif(&mut self) -> _SaifW {
        _SaifW { w: self }
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn saf(&mut self) -> _SafW {
        _SafW { w: self }
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn hpf(&mut self) -> _HpfW {
        _HpfW { w: self }
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn ra(&mut self) -> _RaW {
        _RaW { w: self }
    }
}
