#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Ospeedr {
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
#[doc = "Possible values of the field `OSPEEDR15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ospeedr15R {
    #[doc = "Low speed."] Low,
    #[doc = "Medium speed."] Medium,
    #[doc = "High speed."] High,
    #[doc = "Very high speed."] VeryHigh,
}
impl Ospeedr15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            Ospeedr15R::Low => 0,
            Ospeedr15R::Medium => 1,
            Ospeedr15R::High => 2,
            Ospeedr15R::VeryHigh => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(bits: u8) -> Ospeedr15R {
        match bits {
            0 => Ospeedr15R::Low,
            1 => Ospeedr15R::Medium,
            2 => Ospeedr15R::High,
            3 => Ospeedr15R::VeryHigh,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ospeedr15R::Low
    }
    #[doc = "Checks if the value of the field is `Medium`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == Ospeedr15R::Medium
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ospeedr15R::High
    }
    #[doc = "Checks if the value of the field is `VeryHigh`"]
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        *self == Ospeedr15R::VeryHigh
    }
}
#[doc = "Possible values of the field `OSPEEDR14`"]
pub type Ospeedr14R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR13`"]
pub type Ospeedr13R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR12`"]
pub type Ospeedr12R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR11`"]
pub type Ospeedr11R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR10`"]
pub type Ospeedr10R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR9`"]
pub type Ospeedr9R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR8`"]
pub type Ospeedr8R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR7`"]
pub type Ospeedr7R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR6`"]
pub type Ospeedr6R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR5`"]
pub type Ospeedr5R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR4`"]
pub type Ospeedr4R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR3`"]
pub type Ospeedr3R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR2`"]
pub type Ospeedr2R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR1`"]
pub type Ospeedr1R = Ospeedr15R;
#[doc = "Possible values of the field `OSPEEDR0`"]
pub type Ospeedr0R = Ospeedr15R;
#[doc = "Values that can be written to the field `OSPEEDR15`"]
pub enum Ospeedr15W {
    #[doc = "Low speed."] Low,
    #[doc = "Medium speed."] Medium,
    #[doc = "High speed."] High,
    #[doc = "Very high speed."] VeryHigh,
}
impl Ospeedr15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            Ospeedr15W::Low => 0,
            Ospeedr15W::Medium => 1,
            Ospeedr15W::High => 2,
            Ospeedr15W::VeryHigh => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _Ospeedr15W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR14`"]
pub type Ospeedr14W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr14W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr14W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR13`"]
pub type Ospeedr13W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr13W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr13W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR12`"]
pub type Ospeedr12W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr12W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr12W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR11`"]
pub type Ospeedr11W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr11W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR10`"]
pub type Ospeedr10W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr10W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR9`"]
pub type Ospeedr9W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr9W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR8`"]
pub type Ospeedr8W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr8W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR7`"]
pub type Ospeedr7W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr7W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR6`"]
pub type Ospeedr6W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr6W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR5`"]
pub type Ospeedr5W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr5W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR4`"]
pub type Ospeedr4W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr4W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR3`"]
pub type Ospeedr3W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr3W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR2`"]
pub type Ospeedr2W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr2W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR1`"]
pub type Ospeedr1W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr1W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR0`"]
pub type Ospeedr0W = Ospeedr15W;
#[doc = r" Proxy"]
pub struct _Ospeedr0W<'a> {
    w: &'a mut W,
}
impl<'a> _Ospeedr0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Ospeedr0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(Ospeedr15W::Low)
    }
    #[doc = "Medium speed."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(Ospeedr15W::Medium)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(Ospeedr15W::High)
    }
    #[doc = "Very high speed."]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(Ospeedr15W::VeryHigh)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr15(&self) -> Ospeedr15R {
        Ospeedr15R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr14(&self) -> Ospeedr14R {
        Ospeedr14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr13(&self) -> Ospeedr13R {
        Ospeedr13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr12(&self) -> Ospeedr12R {
        Ospeedr12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr11(&self) -> Ospeedr11R {
        Ospeedr11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr10(&self) -> Ospeedr10R {
        Ospeedr10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr9(&self) -> Ospeedr9R {
        Ospeedr9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr8(&self) -> Ospeedr8R {
        Ospeedr8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr7(&self) -> Ospeedr7R {
        Ospeedr7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr6(&self) -> Ospeedr6R {
        Ospeedr6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr5(&self) -> Ospeedr5R {
        Ospeedr5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr4(&self) -> Ospeedr4R {
        Ospeedr4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&self) -> Ospeedr3R {
        Ospeedr3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr2(&self) -> Ospeedr2R {
        Ospeedr2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr1(&self) -> Ospeedr1R {
        Ospeedr1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr0(&self) -> Ospeedr0R {
        Ospeedr0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 192 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr15(&mut self) -> _Ospeedr15W {
        _Ospeedr15W { w: self }
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr14(&mut self) -> _Ospeedr14W {
        _Ospeedr14W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr13(&mut self) -> _Ospeedr13W {
        _Ospeedr13W { w: self }
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr12(&mut self) -> _Ospeedr12W {
        _Ospeedr12W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr11(&mut self) -> _Ospeedr11W {
        _Ospeedr11W { w: self }
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr10(&mut self) -> _Ospeedr10W {
        _Ospeedr10W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr9(&mut self) -> _Ospeedr9W {
        _Ospeedr9W { w: self }
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr8(&mut self) -> _Ospeedr8W {
        _Ospeedr8W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr7(&mut self) -> _Ospeedr7W {
        _Ospeedr7W { w: self }
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr6(&mut self) -> _Ospeedr6W {
        _Ospeedr6W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr5(&mut self) -> _Ospeedr5W {
        _Ospeedr5W { w: self }
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr4(&mut self) -> _Ospeedr4W {
        _Ospeedr4W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&mut self) -> _Ospeedr3W {
        _Ospeedr3W { w: self }
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr2(&mut self) -> _Ospeedr2W {
        _Ospeedr2W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr1(&mut self) -> _Ospeedr1W {
        _Ospeedr1W { w: self }
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr0(&mut self) -> _Ospeedr0W {
        _Ospeedr0W { w: self }
    }
}
