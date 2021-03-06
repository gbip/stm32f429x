#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OtgHsHcsplt4 {
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
pub struct PrtaddrR {
    bits: u8,
}
impl PrtaddrR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HubaddrR {
    bits: u8,
}
impl HubaddrR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XactposR {
    bits: u8,
}
impl XactposR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ComplspltR {
    bits: u8,
}
impl ComplspltR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SplitenR {
    bits: u8,
}
impl SplitenR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PrtaddrW<'a> {
    w: &'a mut W,
}
impl<'a> _PrtaddrW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HubaddrW<'a> {
    w: &'a mut W,
}
impl<'a> _HubaddrW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XactposW<'a> {
    w: &'a mut W,
}
impl<'a> _XactposW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ComplspltW<'a> {
    w: &'a mut W,
}
impl<'a> _ComplspltW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SplitenW<'a> {
    w: &'a mut W,
}
impl<'a> _SplitenW<'a> {
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
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn prtaddr(&self) -> PrtaddrR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PrtaddrR { bits }
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    pub fn hubaddr(&self) -> HubaddrR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HubaddrR { bits }
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&self) -> XactposR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XactposR { bits }
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    pub fn complsplt(&self) -> ComplspltR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ComplspltR { bits }
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    pub fn spliten(&self) -> SplitenR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SplitenR { bits }
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
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn prtaddr(&mut self) -> _PrtaddrW {
        _PrtaddrW { w: self }
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    pub fn hubaddr(&mut self) -> _HubaddrW {
        _HubaddrW { w: self }
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&mut self) -> _XactposW {
        _XactposW { w: self }
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    pub fn complsplt(&mut self) -> _ComplspltW {
        _ComplspltW { w: self }
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    pub fn spliten(&mut self) -> _SplitenW {
        _SplitenW { w: self }
    }
}
