#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Ptptscr {
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
pub struct TseR {
    bits: u8,
}
impl TseR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TsfcuR {
    bits: u8,
}
impl TsfcuR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Tsptppsv2eR {
    bits: u8,
}
impl Tsptppsv2eR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TssptpoefeR {
    bits: u8,
}
impl TssptpoefeR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Tssipv6feR {
    bits: u8,
}
impl Tssipv6feR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Tssipv4feR {
    bits: u8,
}
impl Tssipv4feR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TssemeR {
    bits: u8,
}
impl TssemeR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TssmrmeR {
    bits: u8,
}
impl TssmrmeR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TscntR {
    bits: u8,
}
impl TscntR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TspffmaeR {
    bits: u8,
}
impl TspffmaeR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TsstiR {
    bits: u8,
}
impl TsstiR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TsstuR {
    bits: u8,
}
impl TsstuR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TsiteR {
    bits: u8,
}
impl TsiteR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TtsaruR {
    bits: u8,
}
impl TtsaruR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TssarfeR {
    bits: u8,
}
impl TssarfeR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TsssrR {
    bits: u8,
}
impl TsssrR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TseW<'a> {
    w: &'a mut W,
}
impl<'a> _TseW<'a> {
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
pub struct _TsfcuW<'a> {
    w: &'a mut W,
}
impl<'a> _TsfcuW<'a> {
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
pub struct _Tsptppsv2eW<'a> {
    w: &'a mut W,
}
impl<'a> _Tsptppsv2eW<'a> {
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
pub struct _TssptpoefeW<'a> {
    w: &'a mut W,
}
impl<'a> _TssptpoefeW<'a> {
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
#[doc = r" Proxy"]
pub struct _Tssipv6feW<'a> {
    w: &'a mut W,
}
impl<'a> _Tssipv6feW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Tssipv4feW<'a> {
    w: &'a mut W,
}
impl<'a> _Tssipv4feW<'a> {
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
pub struct _TssemeW<'a> {
    w: &'a mut W,
}
impl<'a> _TssemeW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TssmrmeW<'a> {
    w: &'a mut W,
}
impl<'a> _TssmrmeW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TscntW<'a> {
    w: &'a mut W,
}
impl<'a> _TscntW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TspffmaeW<'a> {
    w: &'a mut W,
}
impl<'a> _TspffmaeW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TsstiW<'a> {
    w: &'a mut W,
}
impl<'a> _TsstiW<'a> {
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
pub struct _TsstuW<'a> {
    w: &'a mut W,
}
impl<'a> _TsstuW<'a> {
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
pub struct _TsiteW<'a> {
    w: &'a mut W,
}
impl<'a> _TsiteW<'a> {
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
pub struct _TtsaruW<'a> {
    w: &'a mut W,
}
impl<'a> _TtsaruW<'a> {
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
pub struct _TssarfeW<'a> {
    w: &'a mut W,
}
impl<'a> _TssarfeW<'a> {
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
pub struct _TsssrW<'a> {
    w: &'a mut W,
}
impl<'a> _TsssrW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TseR { bits }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn tsfcu(&self) -> TsfcuR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TsfcuR { bits }
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn tsptppsv2e(&self) -> Tsptppsv2eR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tsptppsv2eR { bits }
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn tssptpoefe(&self) -> TssptpoefeR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TssptpoefeR { bits }
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn tssipv6fe(&self) -> Tssipv6feR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tssipv6feR { bits }
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn tssipv4fe(&self) -> Tssipv4feR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tssipv4feR { bits }
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn tsseme(&self) -> TssemeR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TssemeR { bits }
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn tssmrme(&self) -> TssmrmeR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TssmrmeR { bits }
    }
    #[doc = "Bits 16:17 - no description available"]
    #[inline(always)]
    pub fn tscnt(&self) -> TscntR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TscntR { bits }
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn tspffmae(&self) -> TspffmaeR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TspffmaeR { bits }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn tssti(&self) -> TsstiR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TsstiR { bits }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn tsstu(&self) -> TsstuR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TsstuR { bits }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn tsite(&self) -> TsiteR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TsiteR { bits }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn ttsaru(&self) -> TtsaruR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TtsaruR { bits }
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn tssarfe(&self) -> TssarfeR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TssarfeR { bits }
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn tsssr(&self) -> TsssrR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TsssrR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 8192 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn tse(&mut self) -> _TseW {
        _TseW { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn tsfcu(&mut self) -> _TsfcuW {
        _TsfcuW { w: self }
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn tsptppsv2e(&mut self) -> _Tsptppsv2eW {
        _Tsptppsv2eW { w: self }
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn tssptpoefe(&mut self) -> _TssptpoefeW {
        _TssptpoefeW { w: self }
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn tssipv6fe(&mut self) -> _Tssipv6feW {
        _Tssipv6feW { w: self }
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn tssipv4fe(&mut self) -> _Tssipv4feW {
        _Tssipv4feW { w: self }
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn tsseme(&mut self) -> _TssemeW {
        _TssemeW { w: self }
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn tssmrme(&mut self) -> _TssmrmeW {
        _TssmrmeW { w: self }
    }
    #[doc = "Bits 16:17 - no description available"]
    #[inline(always)]
    pub fn tscnt(&mut self) -> _TscntW {
        _TscntW { w: self }
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn tspffmae(&mut self) -> _TspffmaeW {
        _TspffmaeW { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn tssti(&mut self) -> _TsstiW {
        _TsstiW { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn tsstu(&mut self) -> _TsstuW {
        _TsstuW { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn tsite(&mut self) -> _TsiteW {
        _TsiteW { w: self }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn ttsaru(&mut self) -> _TtsaruW {
        _TtsaruW { w: self }
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn tssarfe(&mut self) -> _TssarfeW {
        _TssarfeW { w: self }
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn tsssr(&mut self) -> _TsssrW {
        _TsssrW { w: self }
    }
}
