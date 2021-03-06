#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Sqr2 {
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
pub struct Sq12R {
    bits: u8,
}
impl Sq12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Sq11R {
    bits: u8,
}
impl Sq11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Sq10R {
    bits: u8,
}
impl Sq10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Sq9R {
    bits: u8,
}
impl Sq9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Sq8R {
    bits: u8,
}
impl Sq8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Sq7R {
    bits: u8,
}
impl Sq7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _Sq12W<'a> {
    w: &'a mut W,
}
impl<'a> _Sq12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Sq11W<'a> {
    w: &'a mut W,
}
impl<'a> _Sq11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Sq10W<'a> {
    w: &'a mut W,
}
impl<'a> _Sq10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Sq9W<'a> {
    w: &'a mut W,
}
impl<'a> _Sq9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Sq8W<'a> {
    w: &'a mut W,
}
impl<'a> _Sq8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Sq7W<'a> {
    w: &'a mut W,
}
impl<'a> _Sq7W<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq12(&self) -> Sq12R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Sq12R { bits }
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq11(&self) -> Sq11R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Sq11R { bits }
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq10(&self) -> Sq10R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Sq10R { bits }
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq9(&self) -> Sq9R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Sq9R { bits }
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq8(&self) -> Sq8R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Sq8R { bits }
    }
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq7(&self) -> Sq7R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Sq7R { bits }
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
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq12(&mut self) -> _Sq12W {
        _Sq12W { w: self }
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq11(&mut self) -> _Sq11W {
        _Sq11W { w: self }
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq10(&mut self) -> _Sq10W {
        _Sq10W { w: self }
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq9(&mut self) -> _Sq9W {
        _Sq9W { w: self }
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq8(&mut self) -> _Sq8W {
        _Sq8W { w: self }
    }
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq7(&mut self) -> _Sq7W {
        _Sq7W { w: self }
    }
}
