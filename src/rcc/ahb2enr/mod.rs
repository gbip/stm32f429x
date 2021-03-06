#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Ahb2enr {
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
#[doc = "Possible values of the field `OTGFSEN`"]
pub type OtgfsenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `RNGEN`"]
pub type RngenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `DCMIEN`"]
pub type DcmienR = super::ahb1enr::OtghsulpienR;
#[doc = "Values that can be written to the field `OTGFSEN`"]
pub type OtgfsenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _OtgfsenW<'a> {
    w: &'a mut W,
}
impl<'a> _OtgfsenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OtgfsenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RNGEN`"]
pub type RngenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _RngenW<'a> {
    w: &'a mut W,
}
impl<'a> _RngenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RngenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCMIEN`"]
pub type DcmienW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _DcmienW<'a> {
    w: &'a mut W,
}
impl<'a> _DcmienW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DcmienW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
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
    #[doc = "Bit 7 - USB OTG FS clock enable"]
    #[inline(always)]
    pub fn otgfsen(&self) -> OtgfsenR {
        OtgfsenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Random number generator clock enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RngenR {
        RngenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Camera interface enable"]
    #[inline(always)]
    pub fn dcmien(&self) -> DcmienR {
        DcmienR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bit 7 - USB OTG FS clock enable"]
    #[inline(always)]
    pub fn otgfsen(&mut self) -> _OtgfsenW {
        _OtgfsenW { w: self }
    }
    #[doc = "Bit 6 - Random number generator clock enable"]
    #[inline(always)]
    pub fn rngen(&mut self) -> _RngenW {
        _RngenW { w: self }
    }
    #[doc = "Bit 0 - Camera interface enable"]
    #[inline(always)]
    pub fn dcmien(&mut self) -> _DcmienW {
        _DcmienW { w: self }
    }
}
