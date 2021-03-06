#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Fs1r {
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
pub struct Fsc0R {
    bits: u8,
}
impl Fsc0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc1R {
    bits: u8,
}
impl Fsc1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc2R {
    bits: u8,
}
impl Fsc2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc3R {
    bits: u8,
}
impl Fsc3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc4R {
    bits: u8,
}
impl Fsc4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc5R {
    bits: u8,
}
impl Fsc5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc6R {
    bits: u8,
}
impl Fsc6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc7R {
    bits: u8,
}
impl Fsc7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc8R {
    bits: u8,
}
impl Fsc8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc9R {
    bits: u8,
}
impl Fsc9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc10R {
    bits: u8,
}
impl Fsc10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc11R {
    bits: u8,
}
impl Fsc11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc12R {
    bits: u8,
}
impl Fsc12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc13R {
    bits: u8,
}
impl Fsc13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc14R {
    bits: u8,
}
impl Fsc14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc15R {
    bits: u8,
}
impl Fsc15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc16R {
    bits: u8,
}
impl Fsc16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc17R {
    bits: u8,
}
impl Fsc17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc18R {
    bits: u8,
}
impl Fsc18R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc19R {
    bits: u8,
}
impl Fsc19R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc20R {
    bits: u8,
}
impl Fsc20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc21R {
    bits: u8,
}
impl Fsc21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc22R {
    bits: u8,
}
impl Fsc22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc23R {
    bits: u8,
}
impl Fsc23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc24R {
    bits: u8,
}
impl Fsc24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc25R {
    bits: u8,
}
impl Fsc25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc26R {
    bits: u8,
}
impl Fsc26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fsc27R {
    bits: u8,
}
impl Fsc27R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _Fsc0W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc1W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc2W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc3W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc4W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc5W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc6W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc7W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc8W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc9W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc10W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc11W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc12W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc13W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc13W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc14W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc15W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc15W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc16W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc16W<'a> {
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
pub struct _Fsc17W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc17W<'a> {
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
pub struct _Fsc18W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc18W<'a> {
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
#[doc = r" Proxy"]
pub struct _Fsc19W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc19W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc20W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc20W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc21W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc21W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc22W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc22W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc23W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc23W<'a> {
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
pub struct _Fsc24W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc24W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc25W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc25W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc26W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc26W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Fsc27W<'a> {
    w: &'a mut W,
}
impl<'a> _Fsc27W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 27;
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
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc0(&self) -> Fsc0R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc0R { bits }
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc1(&self) -> Fsc1R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc1R { bits }
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc2(&self) -> Fsc2R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc2R { bits }
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc3(&self) -> Fsc3R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc3R { bits }
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc4(&self) -> Fsc4R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc4R { bits }
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc5(&self) -> Fsc5R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc5R { bits }
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc6(&self) -> Fsc6R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc6R { bits }
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc7(&self) -> Fsc7R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc7R { bits }
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc8(&self) -> Fsc8R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc8R { bits }
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc9(&self) -> Fsc9R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc9R { bits }
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc10(&self) -> Fsc10R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc10R { bits }
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc11(&self) -> Fsc11R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc11R { bits }
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc12(&self) -> Fsc12R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc12R { bits }
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc13(&self) -> Fsc13R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc13R { bits }
    }
    #[doc = "Bit 14 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc14(&self) -> Fsc14R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc14R { bits }
    }
    #[doc = "Bit 15 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc15(&self) -> Fsc15R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc15R { bits }
    }
    #[doc = "Bit 16 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc16(&self) -> Fsc16R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc16R { bits }
    }
    #[doc = "Bit 17 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc17(&self) -> Fsc17R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc17R { bits }
    }
    #[doc = "Bit 18 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc18(&self) -> Fsc18R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc18R { bits }
    }
    #[doc = "Bit 19 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc19(&self) -> Fsc19R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc19R { bits }
    }
    #[doc = "Bit 20 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc20(&self) -> Fsc20R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc20R { bits }
    }
    #[doc = "Bit 21 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc21(&self) -> Fsc21R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc21R { bits }
    }
    #[doc = "Bit 22 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc22(&self) -> Fsc22R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc22R { bits }
    }
    #[doc = "Bit 23 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc23(&self) -> Fsc23R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc23R { bits }
    }
    #[doc = "Bit 24 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc24(&self) -> Fsc24R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc24R { bits }
    }
    #[doc = "Bit 25 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc25(&self) -> Fsc25R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc25R { bits }
    }
    #[doc = "Bit 26 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc26(&self) -> Fsc26R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc26R { bits }
    }
    #[doc = "Bit 27 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc27(&self) -> Fsc27R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fsc27R { bits }
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
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc0(&mut self) -> _Fsc0W {
        _Fsc0W { w: self }
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc1(&mut self) -> _Fsc1W {
        _Fsc1W { w: self }
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc2(&mut self) -> _Fsc2W {
        _Fsc2W { w: self }
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc3(&mut self) -> _Fsc3W {
        _Fsc3W { w: self }
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc4(&mut self) -> _Fsc4W {
        _Fsc4W { w: self }
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc5(&mut self) -> _Fsc5W {
        _Fsc5W { w: self }
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc6(&mut self) -> _Fsc6W {
        _Fsc6W { w: self }
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc7(&mut self) -> _Fsc7W {
        _Fsc7W { w: self }
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc8(&mut self) -> _Fsc8W {
        _Fsc8W { w: self }
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc9(&mut self) -> _Fsc9W {
        _Fsc9W { w: self }
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc10(&mut self) -> _Fsc10W {
        _Fsc10W { w: self }
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc11(&mut self) -> _Fsc11W {
        _Fsc11W { w: self }
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc12(&mut self) -> _Fsc12W {
        _Fsc12W { w: self }
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc13(&mut self) -> _Fsc13W {
        _Fsc13W { w: self }
    }
    #[doc = "Bit 14 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc14(&mut self) -> _Fsc14W {
        _Fsc14W { w: self }
    }
    #[doc = "Bit 15 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc15(&mut self) -> _Fsc15W {
        _Fsc15W { w: self }
    }
    #[doc = "Bit 16 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc16(&mut self) -> _Fsc16W {
        _Fsc16W { w: self }
    }
    #[doc = "Bit 17 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc17(&mut self) -> _Fsc17W {
        _Fsc17W { w: self }
    }
    #[doc = "Bit 18 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc18(&mut self) -> _Fsc18W {
        _Fsc18W { w: self }
    }
    #[doc = "Bit 19 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc19(&mut self) -> _Fsc19W {
        _Fsc19W { w: self }
    }
    #[doc = "Bit 20 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc20(&mut self) -> _Fsc20W {
        _Fsc20W { w: self }
    }
    #[doc = "Bit 21 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc21(&mut self) -> _Fsc21W {
        _Fsc21W { w: self }
    }
    #[doc = "Bit 22 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc22(&mut self) -> _Fsc22W {
        _Fsc22W { w: self }
    }
    #[doc = "Bit 23 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc23(&mut self) -> _Fsc23W {
        _Fsc23W { w: self }
    }
    #[doc = "Bit 24 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc24(&mut self) -> _Fsc24W {
        _Fsc24W { w: self }
    }
    #[doc = "Bit 25 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc25(&mut self) -> _Fsc25W {
        _Fsc25W { w: self }
    }
    #[doc = "Bit 26 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc26(&mut self) -> _Fsc26W {
        _Fsc26W { w: self }
    }
    #[doc = "Bit 27 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc27(&mut self) -> _Fsc27W {
        _Fsc27W { w: self }
    }
}
