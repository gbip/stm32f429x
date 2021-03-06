#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Moder {
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
#[doc = "Possible values of the field `MODER15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Moder15R {
    #[doc = "Input mode (reset)."] Input,
    #[doc = "General purpose output mode."] Output,
    #[doc = "Alternate function mode."] Afm,
    #[doc = "Analog mode"] Analog,
}
impl Moder15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            Moder15R::Input => 0,
            Moder15R::Output => 1,
            Moder15R::Afm => 2,
            Moder15R::Analog => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(bits: u8) -> Moder15R {
        match bits {
            0 => Moder15R::Input,
            1 => Moder15R::Output,
            2 => Moder15R::Afm,
            3 => Moder15R::Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Input`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Moder15R::Input
    }
    #[doc = "Checks if the value of the field is `Output`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Moder15R::Output
    }
    #[doc = "Checks if the value of the field is `Afm`"]
    #[inline(always)]
    pub fn is_afm(&self) -> bool {
        *self == Moder15R::Afm
    }
    #[doc = "Checks if the value of the field is `Analog`"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == Moder15R::Analog
    }
}
#[doc = "Possible values of the field `MODER14`"]
pub type Moder14R = Moder15R;
#[doc = "Possible values of the field `MODER13`"]
pub type Moder13R = Moder15R;
#[doc = "Possible values of the field `MODER12`"]
pub type Moder12R = Moder15R;
#[doc = "Possible values of the field `MODER11`"]
pub type Moder11R = Moder15R;
#[doc = "Possible values of the field `MODER10`"]
pub type Moder10R = Moder15R;
#[doc = "Possible values of the field `MODER9`"]
pub type Moder9R = Moder15R;
#[doc = "Possible values of the field `MODER8`"]
pub type Moder8R = Moder15R;
#[doc = "Possible values of the field `MODER7`"]
pub type Moder7R = Moder15R;
#[doc = "Possible values of the field `MODER6`"]
pub type Moder6R = Moder15R;
#[doc = "Possible values of the field `MODER5`"]
pub type Moder5R = Moder15R;
#[doc = "Possible values of the field `MODER4`"]
pub type Moder4R = Moder15R;
#[doc = "Possible values of the field `MODER3`"]
pub type Moder3R = Moder15R;
#[doc = "Possible values of the field `MODER2`"]
pub type Moder2R = Moder15R;
#[doc = "Possible values of the field `MODER1`"]
pub type Moder1R = Moder15R;
#[doc = "Possible values of the field `MODER0`"]
pub type Moder0R = Moder15R;
#[doc = "Values that can be written to the field `MODER15`"]
pub enum Moder15W {
    #[doc = "Input mode (reset)."] Input,
    #[doc = "General purpose output mode."] Output,
    #[doc = "Alternate function mode."] Afm,
    #[doc = "Analog mode"] Analog,
}
impl Moder15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            Moder15W::Input => 0,
            Moder15W::Output => 1,
            Moder15W::Afm => 2,
            Moder15W::Analog => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _Moder15W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER14`"]
pub type Moder14W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder14W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder14W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER13`"]
pub type Moder13W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder13W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder13W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER12`"]
pub type Moder12W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder12W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder12W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER11`"]
pub type Moder11W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder11W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER10`"]
pub type Moder10W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder10W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER9`"]
pub type Moder9W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder9W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER8`"]
pub type Moder8W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder8W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER7`"]
pub type Moder7W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder7W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER6`"]
pub type Moder6W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder6W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER5`"]
pub type Moder5W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder5W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER4`"]
pub type Moder4W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder4W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER3`"]
pub type Moder3W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder3W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER2`"]
pub type Moder2W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder2W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER1`"]
pub type Moder1W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder1W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
#[doc = "Values that can be written to the field `MODER0`"]
pub type Moder0W = Moder15W;
#[doc = r" Proxy"]
pub struct _Moder0W<'a> {
    w: &'a mut W,
}
impl<'a> _Moder0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Moder0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input mode (reset)."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(Moder15W::Input)
    }
    #[doc = "General purpose output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(Moder15W::Output)
    }
    #[doc = "Alternate function mode."]
    #[inline(always)]
    pub fn afm(self) -> &'a mut W {
        self.variant(Moder15W::Afm)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(Moder15W::Analog)
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
    pub fn moder15(&self) -> Moder15R {
        Moder15R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder14(&self) -> Moder14R {
        Moder14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder13(&self) -> Moder13R {
        Moder13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder12(&self) -> Moder12R {
        Moder12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder11(&self) -> Moder11R {
        Moder11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder10(&self) -> Moder10R {
        Moder10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder9(&self) -> Moder9R {
        Moder9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder8(&self) -> Moder8R {
        Moder8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder7(&self) -> Moder7R {
        Moder7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder6(&self) -> Moder6R {
        Moder6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder5(&self) -> Moder5R {
        Moder5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder4(&self) -> Moder4R {
        Moder4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder3(&self) -> Moder3R {
        Moder3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder2(&self) -> Moder2R {
        Moder2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder1(&self) -> Moder1R {
        Moder1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder0(&self) -> Moder0R {
        Moder0R::_from({
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
        W { bits: 640 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder15(&mut self) -> _Moder15W {
        _Moder15W { w: self }
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder14(&mut self) -> _Moder14W {
        _Moder14W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder13(&mut self) -> _Moder13W {
        _Moder13W { w: self }
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder12(&mut self) -> _Moder12W {
        _Moder12W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder11(&mut self) -> _Moder11W {
        _Moder11W { w: self }
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder10(&mut self) -> _Moder10W {
        _Moder10W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder9(&mut self) -> _Moder9W {
        _Moder9W { w: self }
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder8(&mut self) -> _Moder8W {
        _Moder8W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder7(&mut self) -> _Moder7W {
        _Moder7W { w: self }
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder6(&mut self) -> _Moder6W {
        _Moder6W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder5(&mut self) -> _Moder5W {
        _Moder5W { w: self }
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder4(&mut self) -> _Moder4W {
        _Moder4W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder3(&mut self) -> _Moder3W {
        _Moder3W { w: self }
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder2(&mut self) -> _Moder2W {
        _Moder2W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder1(&mut self) -> _Moder1W {
        _Moder1W { w: self }
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder0(&mut self) -> _Moder0W {
        _Moder0W { w: self }
    }
}
