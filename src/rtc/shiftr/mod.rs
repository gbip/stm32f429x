#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Shiftr {
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
#[doc = r" Proxy"]
pub struct _Add1sW<'a> {
    w: &'a mut W,
}
impl<'a> _Add1sW<'a> {
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
#[doc = r" Proxy"]
pub struct _SubfsW<'a> {
    w: &'a mut W,
}
impl<'a> _SubfsW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u16) -> &'a mut W {
        const MASK: u16 = 32767;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 31 - Add one second"]
    #[inline(always)]
    pub fn add1s(&mut self) -> _Add1sW {
        _Add1sW { w: self }
    }
    #[doc = "Bits 0:14 - Subtract a fraction of a second"]
    #[inline(always)]
    pub fn subfs(&mut self) -> _SubfsW {
        _SubfsW { w: self }
    }
}
