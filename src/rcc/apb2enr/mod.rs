#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Apb2enr {
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
#[doc = "Possible values of the field `TIM1EN`"]
pub type Tim1enR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `TIM8EN`"]
pub type Tim8enR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `USART1EN`"]
pub type Usart1enR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `USART6EN`"]
pub type Usart6enR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `ADC1EN`"]
pub type Adc1enR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `ADC2EN`"]
pub type Adc2enR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `ADC3EN`"]
pub type Adc3enR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `SDIOEN`"]
pub type SdioenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `SPI1EN`"]
pub type Spi1enR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `SPI4ENR`"]
pub type Spi4enrR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `SYSCFGEN`"]
pub type SyscfgenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `TIM9EN`"]
pub type Tim9enR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `TIM10EN`"]
pub type Tim10enR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `TIM11EN`"]
pub type Tim11enR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `SPI5ENR`"]
pub type Spi5enrR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `SPI6ENR`"]
pub type Spi6enrR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `SAI1EN`"]
pub type Sai1enR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `LTDCEN`"]
pub type LtdcenR = super::ahb1enr::OtghsulpienR;
#[doc = "Values that can be written to the field `TIM1EN`"]
pub type Tim1enW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim1enW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim1enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim1enW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `TIM8EN`"]
pub type Tim8enW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim8enW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim8enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim8enW) -> &'a mut W {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USART1EN`"]
pub type Usart1enW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Usart1enW<'a> {
    w: &'a mut W,
}
impl<'a> _Usart1enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Usart1enW) -> &'a mut W {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USART6EN`"]
pub type Usart6enW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Usart6enW<'a> {
    w: &'a mut W,
}
impl<'a> _Usart6enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Usart6enW) -> &'a mut W {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC1EN`"]
pub type Adc1enW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Adc1enW<'a> {
    w: &'a mut W,
}
impl<'a> _Adc1enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Adc1enW) -> &'a mut W {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC2EN`"]
pub type Adc2enW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Adc2enW<'a> {
    w: &'a mut W,
}
impl<'a> _Adc2enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Adc2enW) -> &'a mut W {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC3EN`"]
pub type Adc3enW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Adc3enW<'a> {
    w: &'a mut W,
}
impl<'a> _Adc3enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Adc3enW) -> &'a mut W {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDIOEN`"]
pub type SdioenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _SdioenW<'a> {
    w: &'a mut W,
}
impl<'a> _SdioenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SdioenW) -> &'a mut W {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI1EN`"]
pub type Spi1enW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Spi1enW<'a> {
    w: &'a mut W,
}
impl<'a> _Spi1enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Spi1enW) -> &'a mut W {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI4ENR`"]
pub type Spi4enrW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Spi4enrW<'a> {
    w: &'a mut W,
}
impl<'a> _Spi4enrW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Spi4enrW) -> &'a mut W {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYSCFGEN`"]
pub type SyscfgenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _SyscfgenW<'a> {
    w: &'a mut W,
}
impl<'a> _SyscfgenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SyscfgenW) -> &'a mut W {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM9EN`"]
pub type Tim9enW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim9enW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim9enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim9enW) -> &'a mut W {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM10EN`"]
pub type Tim10enW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim10enW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim10enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim10enW) -> &'a mut W {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM11EN`"]
pub type Tim11enW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim11enW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim11enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim11enW) -> &'a mut W {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI5ENR`"]
pub type Spi5enrW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Spi5enrW<'a> {
    w: &'a mut W,
}
impl<'a> _Spi5enrW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Spi5enrW) -> &'a mut W {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI6ENR`"]
pub type Spi6enrW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Spi6enrW<'a> {
    w: &'a mut W,
}
impl<'a> _Spi6enrW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Spi6enrW) -> &'a mut W {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAI1EN`"]
pub type Sai1enW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Sai1enW<'a> {
    w: &'a mut W,
}
impl<'a> _Sai1enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Sai1enW) -> &'a mut W {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LTDCEN`"]
pub type LtdcenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _LtdcenW<'a> {
    w: &'a mut W,
}
impl<'a> _LtdcenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LtdcenW) -> &'a mut W {
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
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> Tim1enR {
        Tim1enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 1 - TIM8 clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> Tim8enR {
        Tim8enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6en(&self) -> Usart6enR {
        Usart6enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> Adc1enR {
        Adc1enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&self) -> Adc2enR {
        Adc2enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    pub fn adc3en(&self) -> Adc3enR {
        Adc3enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&self) -> SdioenR {
        SdioenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4enr(&self) -> Spi4enrR {
        Spi4enrR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SyscfgenR {
        SyscfgenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline(always)]
    pub fn tim9en(&self) -> Tim9enR {
        Tim9enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 17 - TIM10 clock enable"]
    #[inline(always)]
    pub fn tim10en(&self) -> Tim10enR {
        Tim10enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline(always)]
    pub fn tim11en(&self) -> Tim11enR {
        Tim11enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - SPI5 clock enable"]
    #[inline(always)]
    pub fn spi5enr(&self) -> Spi5enrR {
        Spi5enrR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - SPI6 clock enable"]
    #[inline(always)]
    pub fn spi6enr(&self) -> Spi6enrR {
        Spi6enrR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&self) -> Sai1enR {
        Sai1enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - LTDC clock enable"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LtdcenR {
        LtdcenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> _Tim1enW {
        _Tim1enW { w: self }
    }
    #[doc = "Bit 1 - TIM8 clock enable"]
    #[inline(always)]
    pub fn tim8en(&mut self) -> _Tim8enW {
        _Tim8enW { w: self }
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> _Usart1enW {
        _Usart1enW { w: self }
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6en(&mut self) -> _Usart6enW {
        _Usart6enW { w: self }
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&mut self) -> _Adc1enW {
        _Adc1enW { w: self }
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&mut self) -> _Adc2enW {
        _Adc2enW { w: self }
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    pub fn adc3en(&mut self) -> _Adc3enW {
        _Adc3enW { w: self }
    }
    #[doc = "Bit 11 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&mut self) -> _SdioenW {
        _SdioenW { w: self }
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> _Spi1enW {
        _Spi1enW { w: self }
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4enr(&mut self) -> _Spi4enrW {
        _Spi4enrW { w: self }
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> _SyscfgenW {
        _SyscfgenW { w: self }
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline(always)]
    pub fn tim9en(&mut self) -> _Tim9enW {
        _Tim9enW { w: self }
    }
    #[doc = "Bit 17 - TIM10 clock enable"]
    #[inline(always)]
    pub fn tim10en(&mut self) -> _Tim10enW {
        _Tim10enW { w: self }
    }
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline(always)]
    pub fn tim11en(&mut self) -> _Tim11enW {
        _Tim11enW { w: self }
    }
    #[doc = "Bit 20 - SPI5 clock enable"]
    #[inline(always)]
    pub fn spi5enr(&mut self) -> _Spi5enrW {
        _Spi5enrW { w: self }
    }
    #[doc = "Bit 21 - SPI6 clock enable"]
    #[inline(always)]
    pub fn spi6enr(&mut self) -> _Spi6enrW {
        _Spi6enrW { w: self }
    }
    #[doc = "Bit 22 - SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&mut self) -> _Sai1enW {
        _Sai1enW { w: self }
    }
    #[doc = "Bit 26 - LTDC clock enable"]
    #[inline(always)]
    pub fn ltdcen(&mut self) -> _LtdcenW {
        _LtdcenW { w: self }
    }
}
