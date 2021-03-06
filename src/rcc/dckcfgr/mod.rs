#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Dckcfgr {
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
pub struct Plli2sdivqR {
    bits: u8,
}
impl Plli2sdivqR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PllsaidivqR {
    bits: u8,
}
impl PllsaidivqR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PllsaidivrR {
    bits: u8,
}
impl PllsaidivrR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Sai1asrcR {
    bits: u8,
}
impl Sai1asrcR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Sai1bsrcR {
    bits: u8,
}
impl Sai1bsrcR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TimpreR {
    bits: u8,
}
impl TimpreR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _Plli2sdivqW<'a> {
    w: &'a mut W,
}
impl<'a> _Plli2sdivqW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PllsaidivqW<'a> {
    w: &'a mut W,
}
impl<'a> _PllsaidivqW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PllsaidivrW<'a> {
    w: &'a mut W,
}
impl<'a> _PllsaidivrW<'a> {
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
pub struct _Sai1asrcW<'a> {
    w: &'a mut W,
}
impl<'a> _Sai1asrcW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Sai1bsrcW<'a> {
    w: &'a mut W,
}
impl<'a> _Sai1bsrcW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TimpreW<'a> {
    w: &'a mut W,
}
impl<'a> _TimpreW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - PLLI2SDIVQ"]
    #[inline(always)]
    pub fn plli2sdivq(&self) -> Plli2sdivqR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Plli2sdivqR { bits }
    }
    #[doc = "Bits 8:12 - PLLSAIDIVQ"]
    #[inline(always)]
    pub fn pllsaidivq(&self) -> PllsaidivqR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PllsaidivqR { bits }
    }
    #[doc = "Bits 16:17 - PLLSAIDIVR"]
    #[inline(always)]
    pub fn pllsaidivr(&self) -> PllsaidivrR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PllsaidivrR { bits }
    }
    #[doc = "Bits 20:21 - SAI1ASRC"]
    #[inline(always)]
    pub fn sai1asrc(&self) -> Sai1asrcR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Sai1asrcR { bits }
    }
    #[doc = "Bits 22:23 - SAI1BSRC"]
    #[inline(always)]
    pub fn sai1bsrc(&self) -> Sai1bsrcR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Sai1bsrcR { bits }
    }
    #[doc = "Bit 24 - TIMPRE"]
    #[inline(always)]
    pub fn timpre(&self) -> TimpreR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TimpreR { bits }
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
    #[doc = "Bits 0:4 - PLLI2SDIVQ"]
    #[inline(always)]
    pub fn plli2sdivq(&mut self) -> _Plli2sdivqW {
        _Plli2sdivqW { w: self }
    }
    #[doc = "Bits 8:12 - PLLSAIDIVQ"]
    #[inline(always)]
    pub fn pllsaidivq(&mut self) -> _PllsaidivqW {
        _PllsaidivqW { w: self }
    }
    #[doc = "Bits 16:17 - PLLSAIDIVR"]
    #[inline(always)]
    pub fn pllsaidivr(&mut self) -> _PllsaidivrW {
        _PllsaidivrW { w: self }
    }
    #[doc = "Bits 20:21 - SAI1ASRC"]
    #[inline(always)]
    pub fn sai1asrc(&mut self) -> _Sai1asrcW {
        _Sai1asrcW { w: self }
    }
    #[doc = "Bits 22:23 - SAI1BSRC"]
    #[inline(always)]
    pub fn sai1bsrc(&mut self) -> _Sai1bsrcW {
        _Sai1bsrcW { w: self }
    }
    #[doc = "Bit 24 - TIMPRE"]
    #[inline(always)]
    pub fn timpre(&mut self) -> _TimpreW {
        _TimpreW { w: self }
    }
}
