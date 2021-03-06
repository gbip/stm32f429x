#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Pmc {
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
pub struct MiiRmiiSelR {
    bits: u8,
}
impl MiiRmiiSelR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Adc1dc2R {
    bits: u8,
}
impl Adc1dc2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Adc2dc2R {
    bits: u8,
}
impl Adc2dc2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Adc3dc2R {
    bits: u8,
}
impl Adc3dc2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _MiiRmiiSelW<'a> {
    w: &'a mut W,
}
impl<'a> _MiiRmiiSelW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Adc1dc2W<'a> {
    w: &'a mut W,
}
impl<'a> _Adc1dc2W<'a> {
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
pub struct _Adc2dc2W<'a> {
    w: &'a mut W,
}
impl<'a> _Adc2dc2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Adc3dc2W<'a> {
    w: &'a mut W,
}
impl<'a> _Adc3dc2W<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 23 - Ethernet PHY interface selection"]
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MiiRmiiSelR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MiiRmiiSelR { bits }
    }
    #[doc = "Bit 16 - ADC1DC2"]
    #[inline(always)]
    pub fn adc1dc2(&self) -> Adc1dc2R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Adc1dc2R { bits }
    }
    #[doc = "Bit 17 - ADC2DC2"]
    #[inline(always)]
    pub fn adc2dc2(&self) -> Adc2dc2R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Adc2dc2R { bits }
    }
    #[doc = "Bit 18 - ADC3DC2"]
    #[inline(always)]
    pub fn adc3dc2(&self) -> Adc3dc2R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Adc3dc2R { bits }
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
    #[doc = "Bit 23 - Ethernet PHY interface selection"]
    #[inline(always)]
    pub fn mii_rmii_sel(&mut self) -> _MiiRmiiSelW {
        _MiiRmiiSelW { w: self }
    }
    #[doc = "Bit 16 - ADC1DC2"]
    #[inline(always)]
    pub fn adc1dc2(&mut self) -> _Adc1dc2W {
        _Adc1dc2W { w: self }
    }
    #[doc = "Bit 17 - ADC2DC2"]
    #[inline(always)]
    pub fn adc2dc2(&mut self) -> _Adc2dc2W {
        _Adc2dc2W { w: self }
    }
    #[doc = "Bit 18 - ADC3DC2"]
    #[inline(always)]
    pub fn adc3dc2(&mut self) -> _Adc3dc2W {
        _Adc3dc2W { w: self }
    }
}
