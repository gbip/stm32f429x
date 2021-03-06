#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Pllcfgr {
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
pub struct Pllq3R {
    bits: u8,
}
impl Pllq3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Pllq2R {
    bits: u8,
}
impl Pllq2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Pllq1R {
    bits: u8,
}
impl Pllq1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Pllq0R {
    bits: u8,
}
impl Pllq0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PllsrcR {
    bits: u8,
}
impl PllsrcR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Pllp1R {
    bits: u8,
}
impl Pllp1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Pllp0R {
    bits: u8,
}
impl Pllp0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Plln8R {
    bits: u8,
}
impl Plln8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Plln7R {
    bits: u8,
}
impl Plln7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Plln6R {
    bits: u8,
}
impl Plln6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Plln5R {
    bits: u8,
}
impl Plln5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Plln4R {
    bits: u8,
}
impl Plln4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Plln3R {
    bits: u8,
}
impl Plln3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Plln2R {
    bits: u8,
}
impl Plln2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Plln1R {
    bits: u8,
}
impl Plln1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Plln0R {
    bits: u8,
}
impl Plln0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Pllm5R {
    bits: u8,
}
impl Pllm5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Pllm4R {
    bits: u8,
}
impl Pllm4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Pllm3R {
    bits: u8,
}
impl Pllm3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Pllm2R {
    bits: u8,
}
impl Pllm2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Pllm1R {
    bits: u8,
}
impl Pllm1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Pllm0R {
    bits: u8,
}
impl Pllm0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _Pllq3W<'a> {
    w: &'a mut W,
}
impl<'a> _Pllq3W<'a> {
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
#[doc = r" Proxy"]
pub struct _Pllq2W<'a> {
    w: &'a mut W,
}
impl<'a> _Pllq2W<'a> {
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
pub struct _Pllq1W<'a> {
    w: &'a mut W,
}
impl<'a> _Pllq1W<'a> {
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
pub struct _Pllq0W<'a> {
    w: &'a mut W,
}
impl<'a> _Pllq0W<'a> {
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
pub struct _PllsrcW<'a> {
    w: &'a mut W,
}
impl<'a> _PllsrcW<'a> {
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
pub struct _Pllp1W<'a> {
    w: &'a mut W,
}
impl<'a> _Pllp1W<'a> {
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
pub struct _Pllp0W<'a> {
    w: &'a mut W,
}
impl<'a> _Pllp0W<'a> {
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
pub struct _Plln8W<'a> {
    w: &'a mut W,
}
impl<'a> _Plln8W<'a> {
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
pub struct _Plln7W<'a> {
    w: &'a mut W,
}
impl<'a> _Plln7W<'a> {
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
pub struct _Plln6W<'a> {
    w: &'a mut W,
}
impl<'a> _Plln6W<'a> {
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
pub struct _Plln5W<'a> {
    w: &'a mut W,
}
impl<'a> _Plln5W<'a> {
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
pub struct _Plln4W<'a> {
    w: &'a mut W,
}
impl<'a> _Plln4W<'a> {
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
pub struct _Plln3W<'a> {
    w: &'a mut W,
}
impl<'a> _Plln3W<'a> {
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
pub struct _Plln2W<'a> {
    w: &'a mut W,
}
impl<'a> _Plln2W<'a> {
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
pub struct _Plln1W<'a> {
    w: &'a mut W,
}
impl<'a> _Plln1W<'a> {
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
pub struct _Plln0W<'a> {
    w: &'a mut W,
}
impl<'a> _Plln0W<'a> {
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
pub struct _Pllm5W<'a> {
    w: &'a mut W,
}
impl<'a> _Pllm5W<'a> {
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
pub struct _Pllm4W<'a> {
    w: &'a mut W,
}
impl<'a> _Pllm4W<'a> {
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
pub struct _Pllm3W<'a> {
    w: &'a mut W,
}
impl<'a> _Pllm3W<'a> {
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
pub struct _Pllm2W<'a> {
    w: &'a mut W,
}
impl<'a> _Pllm2W<'a> {
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
pub struct _Pllm1W<'a> {
    w: &'a mut W,
}
impl<'a> _Pllm1W<'a> {
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
pub struct _Pllm0W<'a> {
    w: &'a mut W,
}
impl<'a> _Pllm0W<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    # [ doc = "Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ] # [ inline ( always ) ]
    pub fn pllq3(&self) -> Pllq3R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Pllq3R { bits }
    }
    # [ doc = "Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ] # [ inline ( always ) ]
    pub fn pllq2(&self) -> Pllq2R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Pllq2R { bits }
    }
    # [ doc = "Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ] # [ inline ( always ) ]
    pub fn pllq1(&self) -> Pllq1R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Pllq1R { bits }
    }
    # [ doc = "Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ] # [ inline ( always ) ]
    pub fn pllq0(&self) -> Pllq0R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Pllq0R { bits }
    }
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PllsrcR { bits }
    }
    #[doc = "Bit 17 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp1(&self) -> Pllp1R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Pllp1R { bits }
    }
    #[doc = "Bit 16 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp0(&self) -> Pllp0R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Pllp0R { bits }
    }
    #[doc = "Bit 14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln8(&self) -> Plln8R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Plln8R { bits }
    }
    #[doc = "Bit 13 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln7(&self) -> Plln7R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Plln7R { bits }
    }
    #[doc = "Bit 12 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln6(&self) -> Plln6R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Plln6R { bits }
    }
    #[doc = "Bit 11 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln5(&self) -> Plln5R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Plln5R { bits }
    }
    #[doc = "Bit 10 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln4(&self) -> Plln4R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Plln4R { bits }
    }
    #[doc = "Bit 9 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln3(&self) -> Plln3R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Plln3R { bits }
    }
    #[doc = "Bit 8 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln2(&self) -> Plln2R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Plln2R { bits }
    }
    #[doc = "Bit 7 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln1(&self) -> Plln1R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Plln1R { bits }
    }
    #[doc = "Bit 6 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln0(&self) -> Plln0R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Plln0R { bits }
    }
    #[doc = "Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm5(&self) -> Pllm5R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Pllm5R { bits }
    }
    #[doc = "Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm4(&self) -> Pllm4R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Pllm4R { bits }
    }
    #[doc = "Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm3(&self) -> Pllm3R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Pllm3R { bits }
    }
    #[doc = "Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm2(&self) -> Pllm2R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Pllm2R { bits }
    }
    #[doc = "Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm1(&self) -> Pllm1R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Pllm1R { bits }
    }
    #[doc = "Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm0(&self) -> Pllm0R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Pllm0R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 603992080 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    # [ doc = "Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ] # [ inline ( always ) ]
    pub fn pllq3(&mut self) -> _Pllq3W {
        _Pllq3W { w: self }
    }
    # [ doc = "Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ] # [ inline ( always ) ]
    pub fn pllq2(&mut self) -> _Pllq2W {
        _Pllq2W { w: self }
    }
    # [ doc = "Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ] # [ inline ( always ) ]
    pub fn pllq1(&mut self) -> _Pllq1W {
        _Pllq1W { w: self }
    }
    # [ doc = "Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ] # [ inline ( always ) ]
    pub fn pllq0(&mut self) -> _Pllq0W {
        _Pllq0W { w: self }
    }
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> _PllsrcW {
        _PllsrcW { w: self }
    }
    #[doc = "Bit 17 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp1(&mut self) -> _Pllp1W {
        _Pllp1W { w: self }
    }
    #[doc = "Bit 16 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp0(&mut self) -> _Pllp0W {
        _Pllp0W { w: self }
    }
    #[doc = "Bit 14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln8(&mut self) -> _Plln8W {
        _Plln8W { w: self }
    }
    #[doc = "Bit 13 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln7(&mut self) -> _Plln7W {
        _Plln7W { w: self }
    }
    #[doc = "Bit 12 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln6(&mut self) -> _Plln6W {
        _Plln6W { w: self }
    }
    #[doc = "Bit 11 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln5(&mut self) -> _Plln5W {
        _Plln5W { w: self }
    }
    #[doc = "Bit 10 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln4(&mut self) -> _Plln4W {
        _Plln4W { w: self }
    }
    #[doc = "Bit 9 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln3(&mut self) -> _Plln3W {
        _Plln3W { w: self }
    }
    #[doc = "Bit 8 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln2(&mut self) -> _Plln2W {
        _Plln2W { w: self }
    }
    #[doc = "Bit 7 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln1(&mut self) -> _Plln1W {
        _Plln1W { w: self }
    }
    #[doc = "Bit 6 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln0(&mut self) -> _Plln0W {
        _Plln0W { w: self }
    }
    #[doc = "Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm5(&mut self) -> _Pllm5W {
        _Pllm5W { w: self }
    }
    #[doc = "Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm4(&mut self) -> _Pllm4W {
        _Pllm4W { w: self }
    }
    #[doc = "Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm3(&mut self) -> _Pllm3W {
        _Pllm3W { w: self }
    }
    #[doc = "Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm2(&mut self) -> _Pllm2W {
        _Pllm2W { w: self }
    }
    #[doc = "Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm1(&mut self) -> _Pllm1W {
        _Pllm1W { w: self }
    }
    #[doc = "Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm0(&mut self) -> _Pllm0W {
        _Pllm0W { w: self }
    }
}
