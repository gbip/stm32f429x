#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Hisr {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct Tcif7R {
    bits: u8,
}
impl Tcif7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Htif7R {
    bits: u8,
}
impl Htif7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Teif7R {
    bits: u8,
}
impl Teif7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Dmeif7R {
    bits: u8,
}
impl Dmeif7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Feif7R {
    bits: u8,
}
impl Feif7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Tcif6R {
    bits: u8,
}
impl Tcif6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Htif6R {
    bits: u8,
}
impl Htif6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Teif6R {
    bits: u8,
}
impl Teif6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Dmeif6R {
    bits: u8,
}
impl Dmeif6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Feif6R {
    bits: u8,
}
impl Feif6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Tcif5R {
    bits: u8,
}
impl Tcif5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Htif5R {
    bits: u8,
}
impl Htif5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Teif5R {
    bits: u8,
}
impl Teif5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Dmeif5R {
    bits: u8,
}
impl Dmeif5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Feif5R {
    bits: u8,
}
impl Feif5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Tcif4R {
    bits: u8,
}
impl Tcif4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Htif4R {
    bits: u8,
}
impl Htif4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Teif4R {
    bits: u8,
}
impl Teif4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Dmeif4R {
    bits: u8,
}
impl Dmeif4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Feif4R {
    bits: u8,
}
impl Feif4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 27 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif7(&self) -> Tcif7R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tcif7R { bits }
    }
    #[doc = "Bit 26 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif7(&self) -> Htif7R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Htif7R { bits }
    }
    #[doc = "Bit 25 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif7(&self) -> Teif7R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Teif7R { bits }
    }
    #[doc = "Bit 24 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif7(&self) -> Dmeif7R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Dmeif7R { bits }
    }
    #[doc = "Bit 22 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif7(&self) -> Feif7R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Feif7R { bits }
    }
    #[doc = "Bit 21 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif6(&self) -> Tcif6R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tcif6R { bits }
    }
    #[doc = "Bit 20 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif6(&self) -> Htif6R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Htif6R { bits }
    }
    #[doc = "Bit 19 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif6(&self) -> Teif6R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Teif6R { bits }
    }
    #[doc = "Bit 18 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif6(&self) -> Dmeif6R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Dmeif6R { bits }
    }
    #[doc = "Bit 16 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif6(&self) -> Feif6R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Feif6R { bits }
    }
    #[doc = "Bit 11 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif5(&self) -> Tcif5R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tcif5R { bits }
    }
    #[doc = "Bit 10 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif5(&self) -> Htif5R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Htif5R { bits }
    }
    #[doc = "Bit 9 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif5(&self) -> Teif5R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Teif5R { bits }
    }
    #[doc = "Bit 8 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif5(&self) -> Dmeif5R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Dmeif5R { bits }
    }
    #[doc = "Bit 6 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif5(&self) -> Feif5R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Feif5R { bits }
    }
    #[doc = "Bit 5 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif4(&self) -> Tcif4R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tcif4R { bits }
    }
    #[doc = "Bit 4 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif4(&self) -> Htif4R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Htif4R { bits }
    }
    #[doc = "Bit 3 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif4(&self) -> Teif4R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Teif4R { bits }
    }
    #[doc = "Bit 2 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif4(&self) -> Dmeif4R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Dmeif4R { bits }
    }
    #[doc = "Bit 0 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif4(&self) -> Feif4R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Feif4R { bits }
    }
}
