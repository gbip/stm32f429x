#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Dmaomr {
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
pub struct SrR {
    bits: u8,
}
impl SrR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OsfR {
    bits: u8,
}
impl OsfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RtcR {
    bits: u8,
}
impl RtcR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FugfR {
    bits: u8,
}
impl FugfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FefR {
    bits: u8,
}
impl FefR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct StR {
    bits: u8,
}
impl StR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TtcR {
    bits: u8,
}
impl TtcR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FtfR {
    bits: u8,
}
impl FtfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TsfR {
    bits: u8,
}
impl TsfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DfrfR {
    bits: u8,
}
impl DfrfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RsfR {
    bits: u8,
}
impl RsfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DtcefdR {
    bits: u8,
}
impl DtcefdR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SrW<'a> {
    w: &'a mut W,
}
impl<'a> _SrW<'a> {
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
pub struct _OsfW<'a> {
    w: &'a mut W,
}
impl<'a> _OsfW<'a> {
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
pub struct _RtcW<'a> {
    w: &'a mut W,
}
impl<'a> _RtcW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FugfW<'a> {
    w: &'a mut W,
}
impl<'a> _FugfW<'a> {
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
pub struct _FefW<'a> {
    w: &'a mut W,
}
impl<'a> _FefW<'a> {
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
pub struct _StW<'a> {
    w: &'a mut W,
}
impl<'a> _StW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TtcW<'a> {
    w: &'a mut W,
}
impl<'a> _TtcW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FtfW<'a> {
    w: &'a mut W,
}
impl<'a> _FtfW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TsfW<'a> {
    w: &'a mut W,
}
impl<'a> _TsfW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DfrfW<'a> {
    w: &'a mut W,
}
impl<'a> _DfrfW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RsfW<'a> {
    w: &'a mut W,
}
impl<'a> _RsfW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DtcefdW<'a> {
    w: &'a mut W,
}
impl<'a> _DtcefdW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 1 - SR"]
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SrR { bits }
    }
    #[doc = "Bit 2 - OSF"]
    #[inline(always)]
    pub fn osf(&self) -> OsfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OsfR { bits }
    }
    #[doc = "Bits 3:4 - RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RtcR { bits }
    }
    #[doc = "Bit 6 - FUGF"]
    #[inline(always)]
    pub fn fugf(&self) -> FugfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FugfR { bits }
    }
    #[doc = "Bit 7 - FEF"]
    #[inline(always)]
    pub fn fef(&self) -> FefR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FefR { bits }
    }
    #[doc = "Bit 13 - ST"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        StR { bits }
    }
    #[doc = "Bits 14:16 - TTC"]
    #[inline(always)]
    pub fn ttc(&self) -> TtcR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TtcR { bits }
    }
    #[doc = "Bit 20 - FTF"]
    #[inline(always)]
    pub fn ftf(&self) -> FtfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FtfR { bits }
    }
    #[doc = "Bit 21 - TSF"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TsfR { bits }
    }
    #[doc = "Bit 24 - DFRF"]
    #[inline(always)]
    pub fn dfrf(&self) -> DfrfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DfrfR { bits }
    }
    #[doc = "Bit 25 - RSF"]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RsfR { bits }
    }
    #[doc = "Bit 26 - DTCEFD"]
    #[inline(always)]
    pub fn dtcefd(&self) -> DtcefdR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DtcefdR { bits }
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
    #[doc = "Bit 1 - SR"]
    #[inline(always)]
    pub fn sr(&mut self) -> _SrW {
        _SrW { w: self }
    }
    #[doc = "Bit 2 - OSF"]
    #[inline(always)]
    pub fn osf(&mut self) -> _OsfW {
        _OsfW { w: self }
    }
    #[doc = "Bits 3:4 - RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> _RtcW {
        _RtcW { w: self }
    }
    #[doc = "Bit 6 - FUGF"]
    #[inline(always)]
    pub fn fugf(&mut self) -> _FugfW {
        _FugfW { w: self }
    }
    #[doc = "Bit 7 - FEF"]
    #[inline(always)]
    pub fn fef(&mut self) -> _FefW {
        _FefW { w: self }
    }
    #[doc = "Bit 13 - ST"]
    #[inline(always)]
    pub fn st(&mut self) -> _StW {
        _StW { w: self }
    }
    #[doc = "Bits 14:16 - TTC"]
    #[inline(always)]
    pub fn ttc(&mut self) -> _TtcW {
        _TtcW { w: self }
    }
    #[doc = "Bit 20 - FTF"]
    #[inline(always)]
    pub fn ftf(&mut self) -> _FtfW {
        _FtfW { w: self }
    }
    #[doc = "Bit 21 - TSF"]
    #[inline(always)]
    pub fn tsf(&mut self) -> _TsfW {
        _TsfW { w: self }
    }
    #[doc = "Bit 24 - DFRF"]
    #[inline(always)]
    pub fn dfrf(&mut self) -> _DfrfW {
        _DfrfW { w: self }
    }
    #[doc = "Bit 25 - RSF"]
    #[inline(always)]
    pub fn rsf(&mut self) -> _RsfW {
        _RsfW { w: self }
    }
    #[doc = "Bit 26 - DTCEFD"]
    #[inline(always)]
    pub fn dtcefd(&mut self) -> _DtcefdW {
        _DtcefdW { w: self }
    }
}
