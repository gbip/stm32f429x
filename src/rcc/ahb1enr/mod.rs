#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Ahb1enr {
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
#[doc = "Possible values of the field `OTGHSULPIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OtghsulpienR {
    #[doc = "Enabled."] Enabled,
    #[doc = "Disabled."] Disabled,
}
impl OtghsulpienR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            OtghsulpienR::Enabled => 1,
            OtghsulpienR::Disabled => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(bits: u8) -> OtghsulpienR {
        match bits {
            1 => OtghsulpienR::Enabled,
            0 => OtghsulpienR::Disabled,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OtghsulpienR::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OtghsulpienR::Disabled
    }
}
#[doc = "Possible values of the field `OTGHSEN`"]
pub type OtghsenR = OtghsulpienR;
#[doc = "Possible values of the field `ETHMACPTPEN`"]
pub type EthmacptpenR = OtghsulpienR;
#[doc = "Possible values of the field `ETHMACRXEN`"]
pub type EthmacrxenR = OtghsulpienR;
#[doc = "Possible values of the field `ETHMACTXEN`"]
pub type EthmactxenR = OtghsulpienR;
#[doc = "Possible values of the field `ETHMACEN`"]
pub type EthmacenR = OtghsulpienR;
#[doc = "Possible values of the field `DMA2DEN`"]
pub type Dma2denR = OtghsulpienR;
#[doc = "Possible values of the field `DMA2EN`"]
pub type Dma2enR = OtghsulpienR;
#[doc = "Possible values of the field `DMA1EN`"]
pub type Dma1enR = OtghsulpienR;
#[doc = "Possible values of the field `CCMDATARAMEN`"]
pub type CcmdataramenR = OtghsulpienR;
#[doc = "Possible values of the field `BKPSRAMEN`"]
pub type BkpsramenR = OtghsulpienR;
#[doc = "Possible values of the field `CRCEN`"]
pub type CrcenR = OtghsulpienR;
#[doc = "Possible values of the field `GPIOKEN`"]
pub type GpiokenR = OtghsulpienR;
#[doc = "Possible values of the field `GPIOJEN`"]
pub type GpiojenR = OtghsulpienR;
#[doc = "Possible values of the field `GPIOIEN`"]
pub type GpioienR = OtghsulpienR;
#[doc = "Possible values of the field `GPIOHEN`"]
pub type GpiohenR = OtghsulpienR;
#[doc = "Possible values of the field `GPIOGEN`"]
pub type GpiogenR = OtghsulpienR;
#[doc = "Possible values of the field `GPIOFEN`"]
pub type GpiofenR = OtghsulpienR;
#[doc = "Possible values of the field `GPIOEEN`"]
pub type GpioeenR = OtghsulpienR;
#[doc = "Possible values of the field `GPIODEN`"]
pub type GpiodenR = OtghsulpienR;
#[doc = "Possible values of the field `GPIOCEN`"]
pub type GpiocenR = OtghsulpienR;
#[doc = "Possible values of the field `GPIOBEN`"]
pub type GpiobenR = OtghsulpienR;
#[doc = "Possible values of the field `GPIOAEN`"]
pub type GpioaenR = OtghsulpienR;
#[doc = "Values that can be written to the field `OTGHSULPIEN`"]
pub enum OtghsulpienW {
    #[doc = "Enabled."] Enabled,
    #[doc = "Disabled."] Disabled,
}
impl OtghsulpienW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            OtghsulpienW::Enabled => 1,
            OtghsulpienW::Disabled => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OtghsulpienW<'a> {
    w: &'a mut W,
}
impl<'a> _OtghsulpienW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OtghsulpienW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OTGHSEN`"]
pub type OtghsenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _OtghsenW<'a> {
    w: &'a mut W,
}
impl<'a> _OtghsenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OtghsenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ETHMACPTPEN`"]
pub type EthmacptpenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _EthmacptpenW<'a> {
    w: &'a mut W,
}
impl<'a> _EthmacptpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EthmacptpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ETHMACRXEN`"]
pub type EthmacrxenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _EthmacrxenW<'a> {
    w: &'a mut W,
}
impl<'a> _EthmacrxenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EthmacrxenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ETHMACTXEN`"]
pub type EthmactxenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _EthmactxenW<'a> {
    w: &'a mut W,
}
impl<'a> _EthmactxenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EthmactxenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `ETHMACEN`"]
pub type EthmacenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _EthmacenW<'a> {
    w: &'a mut W,
}
impl<'a> _EthmacenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EthmacenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA2DEN`"]
pub type Dma2denW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Dma2denW<'a> {
    w: &'a mut W,
}
impl<'a> _Dma2denW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Dma2denW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA2EN`"]
pub type Dma2enW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Dma2enW<'a> {
    w: &'a mut W,
}
impl<'a> _Dma2enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Dma2enW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `DMA1EN`"]
pub type Dma1enW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Dma1enW<'a> {
    w: &'a mut W,
}
impl<'a> _Dma1enW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Dma1enW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `CCMDATARAMEN`"]
pub type CcmdataramenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _CcmdataramenW<'a> {
    w: &'a mut W,
}
impl<'a> _CcmdataramenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CcmdataramenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `BKPSRAMEN`"]
pub type BkpsramenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _BkpsramenW<'a> {
    w: &'a mut W,
}
impl<'a> _BkpsramenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BkpsramenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `CRCEN`"]
pub type CrcenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _CrcenW<'a> {
    w: &'a mut W,
}
impl<'a> _CrcenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CrcenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `GPIOKEN`"]
pub type GpiokenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpiokenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpiokenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpiokenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `GPIOJEN`"]
pub type GpiojenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpiojenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpiojenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpiojenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `GPIOIEN`"]
pub type GpioienW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpioienW<'a> {
    w: &'a mut W,
}
impl<'a> _GpioienW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpioienW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `GPIOHEN`"]
pub type GpiohenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpiohenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpiohenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpiohenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `GPIOGEN`"]
pub type GpiogenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpiogenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpiogenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpiogenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `GPIOFEN`"]
pub type GpiofenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpiofenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpiofenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpiofenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `GPIOEEN`"]
pub type GpioeenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpioeenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpioeenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpioeenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `GPIODEN`"]
pub type GpiodenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpiodenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpiodenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpiodenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOCEN`"]
pub type GpiocenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpiocenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpiocenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpiocenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOBEN`"]
pub type GpiobenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpiobenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpiobenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpiobenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
#[doc = "Values that can be written to the field `GPIOAEN`"]
pub type GpioaenW = OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpioaenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpioaenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpioaenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OtghsulpienW::Disabled)
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
    #[doc = "Bit 30 - USB OTG HSULPI clock enable"]
    #[inline(always)]
    pub fn otghsulpien(&self) -> OtghsulpienR {
        OtghsulpienR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - USB OTG HS clock enable"]
    #[inline(always)]
    pub fn otghsen(&self) -> OtghsenR {
        OtghsenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable"]
    #[inline(always)]
    pub fn ethmacptpen(&self) -> EthmacptpenR {
        EthmacptpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - Ethernet Reception clock enable"]
    #[inline(always)]
    pub fn ethmacrxen(&self) -> EthmacrxenR {
        EthmacrxenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Ethernet Transmission clock enable"]
    #[inline(always)]
    pub fn ethmactxen(&self) -> EthmactxenR {
        EthmactxenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 25 - Ethernet MAC clock enable"]
    #[inline(always)]
    pub fn ethmacen(&self) -> EthmacenR {
        EthmacenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - DMA2D clock enable"]
    #[inline(always)]
    pub fn dma2den(&self) -> Dma2denR {
        Dma2denR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> Dma2enR {
        Dma2enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - CCM data RAM clock enable"]
    #[inline(always)]
    pub fn ccmdataramen(&self) -> CcmdataramenR {
        CcmdataramenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable"]
    #[inline(always)]
    pub fn bkpsramen(&self) -> BkpsramenR {
        BkpsramenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - IO port K clock enable"]
    #[inline(always)]
    pub fn gpioken(&self) -> GpiokenR {
        GpiokenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 9 - IO port J clock enable"]
    #[inline(always)]
    pub fn gpiojen(&self) -> GpiojenR {
        GpiojenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline(always)]
    pub fn gpioien(&self) -> GpioienR {
        GpioienR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GpiohenR {
        GpiohenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GpiogenR {
        GpiogenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GpiofenR {
        GpiofenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GpioeenR {
        GpioeenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GpiodenR {
        GpiodenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GpiocenR {
        GpiocenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GpiobenR {
        GpiobenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GpioaenR {
        GpioaenR::_from({
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
        W { bits: 1048576 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 30 - USB OTG HSULPI clock enable"]
    #[inline(always)]
    pub fn otghsulpien(&mut self) -> _OtghsulpienW {
        _OtghsulpienW { w: self }
    }
    #[doc = "Bit 29 - USB OTG HS clock enable"]
    #[inline(always)]
    pub fn otghsen(&mut self) -> _OtghsenW {
        _OtghsenW { w: self }
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable"]
    #[inline(always)]
    pub fn ethmacptpen(&mut self) -> _EthmacptpenW {
        _EthmacptpenW { w: self }
    }
    #[doc = "Bit 27 - Ethernet Reception clock enable"]
    #[inline(always)]
    pub fn ethmacrxen(&mut self) -> _EthmacrxenW {
        _EthmacrxenW { w: self }
    }
    #[doc = "Bit 26 - Ethernet Transmission clock enable"]
    #[inline(always)]
    pub fn ethmactxen(&mut self) -> _EthmactxenW {
        _EthmactxenW { w: self }
    }
    #[doc = "Bit 25 - Ethernet MAC clock enable"]
    #[inline(always)]
    pub fn ethmacen(&mut self) -> _EthmacenW {
        _EthmacenW { w: self }
    }
    #[doc = "Bit 23 - DMA2D clock enable"]
    #[inline(always)]
    pub fn dma2den(&mut self) -> _Dma2denW {
        _Dma2denW { w: self }
    }
    #[doc = "Bit 22 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> _Dma2enW {
        _Dma2enW { w: self }
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> _Dma1enW {
        _Dma1enW { w: self }
    }
    #[doc = "Bit 20 - CCM data RAM clock enable"]
    #[inline(always)]
    pub fn ccmdataramen(&mut self) -> _CcmdataramenW {
        _CcmdataramenW { w: self }
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable"]
    #[inline(always)]
    pub fn bkpsramen(&mut self) -> _BkpsramenW {
        _BkpsramenW { w: self }
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> _CrcenW {
        _CrcenW { w: self }
    }
    #[doc = "Bit 10 - IO port K clock enable"]
    #[inline(always)]
    pub fn gpioken(&mut self) -> _GpiokenW {
        _GpiokenW { w: self }
    }
    #[doc = "Bit 9 - IO port J clock enable"]
    #[inline(always)]
    pub fn gpiojen(&mut self) -> _GpiojenW {
        _GpiojenW { w: self }
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline(always)]
    pub fn gpioien(&mut self) -> _GpioienW {
        _GpioienW { w: self }
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&mut self) -> _GpiohenW {
        _GpiohenW { w: self }
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&mut self) -> _GpiogenW {
        _GpiogenW { w: self }
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&mut self) -> _GpiofenW {
        _GpiofenW { w: self }
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&mut self) -> _GpioeenW {
        _GpioeenW { w: self }
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&mut self) -> _GpiodenW {
        _GpiodenW { w: self }
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&mut self) -> _GpiocenW {
        _GpiocenW { w: self }
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&mut self) -> _GpiobenW {
        _GpiobenW { w: self }
    }
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> _GpioaenW {
        _GpioaenW { w: self }
    }
}
