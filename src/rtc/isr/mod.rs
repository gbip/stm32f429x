#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Isr {
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
pub struct AlrawfR {
    bits: u8,
}
impl AlrawfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AlrbwfR {
    bits: u8,
}
impl AlrbwfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WutwfR {
    bits: u8,
}
impl WutwfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ShpfR {
    bits: u8,
}
impl ShpfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct InitsR {
    bits: u8,
}
impl InitsR {
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
pub struct InitfR {
    bits: u8,
}
impl InitfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct InitR {
    bits: u8,
}
impl InitR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AlrafR {
    bits: u8,
}
impl AlrafR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AlrbfR {
    bits: u8,
}
impl AlrbfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WutfR {
    bits: u8,
}
impl WutfR {
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
pub struct TsovfR {
    bits: u8,
}
impl TsovfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Tamp1fR {
    bits: u8,
}
impl Tamp1fR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Tamp2fR {
    bits: u8,
}
impl Tamp2fR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RecalpfR {
    bits: u8,
}
impl RecalpfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ShpfW<'a> {
    w: &'a mut W,
}
impl<'a> _ShpfW<'a> {
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
pub struct _RsfW<'a> {
    w: &'a mut W,
}
impl<'a> _RsfW<'a> {
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
pub struct _InitW<'a> {
    w: &'a mut W,
}
impl<'a> _InitW<'a> {
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
pub struct _AlrafW<'a> {
    w: &'a mut W,
}
impl<'a> _AlrafW<'a> {
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
pub struct _AlrbfW<'a> {
    w: &'a mut W,
}
impl<'a> _AlrbfW<'a> {
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
pub struct _WutfW<'a> {
    w: &'a mut W,
}
impl<'a> _WutfW<'a> {
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
pub struct _TsfW<'a> {
    w: &'a mut W,
}
impl<'a> _TsfW<'a> {
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
pub struct _TsovfW<'a> {
    w: &'a mut W,
}
impl<'a> _TsovfW<'a> {
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
pub struct _Tamp1fW<'a> {
    w: &'a mut W,
}
impl<'a> _Tamp1fW<'a> {
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
pub struct _Tamp2fW<'a> {
    w: &'a mut W,
}
impl<'a> _Tamp2fW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Alarm A write flag"]
    #[inline(always)]
    pub fn alrawf(&self) -> AlrawfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AlrawfR { bits }
    }
    #[doc = "Bit 1 - Alarm B write flag"]
    #[inline(always)]
    pub fn alrbwf(&self) -> AlrbwfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AlrbwfR { bits }
    }
    #[doc = "Bit 2 - Wakeup timer write flag"]
    #[inline(always)]
    pub fn wutwf(&self) -> WutwfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WutwfR { bits }
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&self) -> ShpfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ShpfR { bits }
    }
    #[doc = "Bit 4 - Initialization status flag"]
    #[inline(always)]
    pub fn inits(&self) -> InitsR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        InitsR { bits }
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RsfR { bits }
    }
    #[doc = "Bit 6 - Initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> InitfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        InitfR { bits }
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        InitR { bits }
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&self) -> AlrafR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AlrafR { bits }
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&self) -> AlrbfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AlrbfR { bits }
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&self) -> WutfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WutfR { bits }
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TsfR { bits }
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&self) -> TsovfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TsovfR { bits }
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    pub fn tamp1f(&self) -> Tamp1fR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tamp1fR { bits }
    }
    #[doc = "Bit 14 - TAMPER2 detection flag"]
    #[inline(always)]
    pub fn tamp2f(&self) -> Tamp2fR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tamp2fR { bits }
    }
    #[doc = "Bit 16 - Recalibration pending Flag"]
    #[inline(always)]
    pub fn recalpf(&self) -> RecalpfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RecalpfR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 7 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&mut self) -> _ShpfW {
        _ShpfW { w: self }
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&mut self) -> _RsfW {
        _RsfW { w: self }
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&mut self) -> _InitW {
        _InitW { w: self }
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&mut self) -> _AlrafW {
        _AlrafW { w: self }
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&mut self) -> _AlrbfW {
        _AlrbfW { w: self }
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&mut self) -> _WutfW {
        _WutfW { w: self }
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&mut self) -> _TsfW {
        _TsfW { w: self }
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&mut self) -> _TsovfW {
        _TsovfW { w: self }
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    pub fn tamp1f(&mut self) -> _Tamp1fW {
        _Tamp1fW { w: self }
    }
    #[doc = "Bit 14 - TAMPER2 detection flag"]
    #[inline(always)]
    pub fn tamp2f(&mut self) -> _Tamp2fW {
        _Tamp2fW { w: self }
    }
}
