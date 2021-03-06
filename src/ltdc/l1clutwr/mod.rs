#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::L1clutwr {
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
pub struct _ClutaddW<'a> {
    w: &'a mut W,
}
impl<'a> _ClutaddW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RedW<'a> {
    w: &'a mut W,
}
impl<'a> _RedW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GreenW<'a> {
    w: &'a mut W,
}
impl<'a> _GreenW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BlueW<'a> {
    w: &'a mut W,
}
impl<'a> _BlueW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 24:31 - CLUT Address"]
    #[inline(always)]
    pub fn clutadd(&mut self) -> _ClutaddW {
        _ClutaddW { w: self }
    }
    #[doc = "Bits 16:23 - Red value"]
    #[inline(always)]
    pub fn red(&mut self) -> _RedW {
        _RedW { w: self }
    }
    #[doc = "Bits 8:15 - Green value"]
    #[inline(always)]
    pub fn green(&mut self) -> _GreenW {
        _GreenW { w: self }
    }
    #[doc = "Bits 0:7 - Blue value"]
    #[inline(always)]
    pub fn blue(&mut self) -> _BlueW {
        _BlueW { w: self }
    }
}
