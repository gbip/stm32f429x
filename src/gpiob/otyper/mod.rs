#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Otyper {
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
#[doc = "Possible values of the field `OT15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ot15R {
    #[doc = "Output push-pull (reset)."] Pushpull,
    #[doc = "Output open-drain."] Opendrain,
}
impl Ot15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            Ot15R::Pushpull => 0,
            Ot15R::Opendrain => 1,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(bits: u8) -> Ot15R {
        match bits {
            0 => Ot15R::Pushpull,
            1 => Ot15R::Opendrain,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pushpull`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Ot15R::Pushpull
    }
    #[doc = "Checks if the value of the field is `Opendrain`"]
    #[inline(always)]
    pub fn is_opendrain(&self) -> bool {
        *self == Ot15R::Opendrain
    }
}
#[doc = "Possible values of the field `OT14`"]
pub type Ot14R = Ot15R;
#[doc = "Possible values of the field `OT13`"]
pub type Ot13R = Ot15R;
#[doc = "Possible values of the field `OT12`"]
pub type Ot12R = Ot15R;
#[doc = "Possible values of the field `OT11`"]
pub type Ot11R = Ot15R;
#[doc = "Possible values of the field `OT10`"]
pub type Ot10R = Ot15R;
#[doc = "Possible values of the field `OT9`"]
pub type Ot9R = Ot15R;
#[doc = "Possible values of the field `OT8`"]
pub type Ot8R = Ot15R;
#[doc = "Possible values of the field `OT7`"]
pub type Ot7R = Ot15R;
#[doc = "Possible values of the field `OT6`"]
pub type Ot6R = Ot15R;
#[doc = "Possible values of the field `OT5`"]
pub type Ot5R = Ot15R;
#[doc = "Possible values of the field `OT4`"]
pub type Ot4R = Ot15R;
#[doc = "Possible values of the field `OT3`"]
pub type Ot3R = Ot15R;
#[doc = "Possible values of the field `OT2`"]
pub type Ot2R = Ot15R;
#[doc = "Possible values of the field `OT1`"]
pub type Ot1R = Ot15R;
#[doc = "Possible values of the field `OT0`"]
pub type Ot0R = Ot15R;
#[doc = "Values that can be written to the field `OT15`"]
pub enum Ot15W {
    #[doc = "Output push-pull (reset)."] Pushpull,
    #[doc = "Output open-drain."] Opendrain,
}
impl Ot15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            Ot15W::Pushpull => 0,
            Ot15W::Opendrain => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _Ot15W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT14`"]
pub type Ot14W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot14W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot14W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT13`"]
pub type Ot13W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot13W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot13W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT12`"]
pub type Ot12W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot12W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot12W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT11`"]
pub type Ot11W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot11W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT10`"]
pub type Ot10W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot10W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT9`"]
pub type Ot9W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot9W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT8`"]
pub type Ot8W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot8W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT7`"]
pub type Ot7W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot7W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT6`"]
pub type Ot6W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot6W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT5`"]
pub type Ot5W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot5W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT4`"]
pub type Ot4W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot4W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT3`"]
pub type Ot3W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot3W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT2`"]
pub type Ot2W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot2W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT1`"]
pub type Ot1W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot1W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
#[doc = "Values that can be written to the field `OT0`"]
pub type Ot0W = Ot15W;
#[doc = r" Proxy"]
pub struct _Ot0W<'a> {
    w: &'a mut W,
}
impl<'a> _Ot0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ot0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output push-pull (reset)."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(Ot15W::Pushpull)
    }
    #[doc = "Output open-drain."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(Ot15W::Opendrain)
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
    #[doc = "Bit 15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot15(&self) -> Ot15R {
        Ot15R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot14(&self) -> Ot14R {
        Ot14R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot13(&self) -> Ot13R {
        Ot13R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot12(&self) -> Ot12R {
        Ot12R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot11(&self) -> Ot11R {
        Ot11R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot10(&self) -> Ot10R {
        Ot10R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot9(&self) -> Ot9R {
        Ot9R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot8(&self) -> Ot8R {
        Ot8R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot7(&self) -> Ot7R {
        Ot7R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot6(&self) -> Ot6R {
        Ot6R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot5(&self) -> Ot5R {
        Ot5R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot4(&self) -> Ot4R {
        Ot4R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot3(&self) -> Ot3R {
        Ot3R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot2(&self) -> Ot2R {
        Ot2R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot1(&self) -> Ot1R {
        Ot1R::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot0(&self) -> Ot0R {
        Ot0R::_from({
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
    #[doc = "Bit 15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot15(&mut self) -> _Ot15W {
        _Ot15W { w: self }
    }
    #[doc = "Bit 14 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot14(&mut self) -> _Ot14W {
        _Ot14W { w: self }
    }
    #[doc = "Bit 13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot13(&mut self) -> _Ot13W {
        _Ot13W { w: self }
    }
    #[doc = "Bit 12 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot12(&mut self) -> _Ot12W {
        _Ot12W { w: self }
    }
    #[doc = "Bit 11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot11(&mut self) -> _Ot11W {
        _Ot11W { w: self }
    }
    #[doc = "Bit 10 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot10(&mut self) -> _Ot10W {
        _Ot10W { w: self }
    }
    #[doc = "Bit 9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot9(&mut self) -> _Ot9W {
        _Ot9W { w: self }
    }
    #[doc = "Bit 8 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot8(&mut self) -> _Ot8W {
        _Ot8W { w: self }
    }
    #[doc = "Bit 7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot7(&mut self) -> _Ot7W {
        _Ot7W { w: self }
    }
    #[doc = "Bit 6 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot6(&mut self) -> _Ot6W {
        _Ot6W { w: self }
    }
    #[doc = "Bit 5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot5(&mut self) -> _Ot5W {
        _Ot5W { w: self }
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot4(&mut self) -> _Ot4W {
        _Ot4W { w: self }
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot3(&mut self) -> _Ot3W {
        _Ot3W { w: self }
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot2(&mut self) -> _Ot2W {
        _Ot2W { w: self }
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot1(&mut self) -> _Ot1W {
        _Ot1W { w: self }
    }
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot0(&mut self) -> _Ot0W {
        _Ot0W { w: self }
    }
}
