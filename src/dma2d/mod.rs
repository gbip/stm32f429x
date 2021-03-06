use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"] pub cr: Cr,
    #[doc = "0x04 - Interrupt Status Register"] pub isr: Isr,
    #[doc = "0x08 - interrupt flag clear register"] pub ifcr: Ifcr,
    #[doc = "0x0c - foreground memory address register"] pub fgmar: Fgmar,
    #[doc = "0x10 - foreground offset register"] pub fgor: Fgor,
    #[doc = "0x14 - background memory address register"] pub bgmar: Bgmar,
    #[doc = "0x18 - background offset register"] pub bgor: Bgor,
    #[doc = "0x1c - foreground PFC control register"] pub fgpfccr: Fgpfccr,
    #[doc = "0x20 - foreground color register"] pub fgcolr: Fgcolr,
    #[doc = "0x24 - background PFC control register"] pub bgpfccr: Bgpfccr,
    #[doc = "0x28 - background color register"] pub bgcolr: Bgcolr,
    #[doc = "0x2c - foreground CLUT memory address register"] pub fgcmar: Fgcmar,
    #[doc = "0x30 - background CLUT memory address register"] pub bgcmar: Bgcmar,
    #[doc = "0x34 - output PFC control register"] pub opfccr: Opfccr,
    #[doc = "0x38 - output color register"] pub ocolr: Ocolr,
    #[doc = "0x3c - output memory address register"] pub omar: Omar,
    #[doc = "0x40 - output offset register"] pub oor: Oor,
    #[doc = "0x44 - number of line register"] pub nlr: Nlr,
    #[doc = "0x48 - line watermark register"] pub lwr: Lwr,
    #[doc = "0x4c - AHB master timer configuration register"] pub amtcr: Amtcr,
    _reserved0: [u8; 944usize],
    #[doc = "0x400 - FGCLUT"] pub fgclut: Fgclut,
    _reserved1: [u8; 1020usize],
    #[doc = "0x800 - BGCLUT"] pub bgclut: Bgclut,
}
#[doc = "control register"]
pub struct Cr {
    register: VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "Interrupt Status Register"]
pub struct Isr {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "interrupt flag clear register"]
pub struct Ifcr {
    register: VolatileCell<u32>,
}
#[doc = "interrupt flag clear register"]
pub mod ifcr;
#[doc = "foreground memory address register"]
pub struct Fgmar {
    register: VolatileCell<u32>,
}
#[doc = "foreground memory address register"]
pub mod fgmar;
#[doc = "foreground offset register"]
pub struct Fgor {
    register: VolatileCell<u32>,
}
#[doc = "foreground offset register"]
pub mod fgor;
#[doc = "background memory address register"]
pub struct Bgmar {
    register: VolatileCell<u32>,
}
#[doc = "background memory address register"]
pub mod bgmar;
#[doc = "background offset register"]
pub struct Bgor {
    register: VolatileCell<u32>,
}
#[doc = "background offset register"]
pub mod bgor;
#[doc = "foreground PFC control register"]
pub struct Fgpfccr {
    register: VolatileCell<u32>,
}
#[doc = "foreground PFC control register"]
pub mod fgpfccr;
#[doc = "foreground color register"]
pub struct Fgcolr {
    register: VolatileCell<u32>,
}
#[doc = "foreground color register"]
pub mod fgcolr;
#[doc = "background PFC control register"]
pub struct Bgpfccr {
    register: VolatileCell<u32>,
}
#[doc = "background PFC control register"]
pub mod bgpfccr;
#[doc = "background color register"]
pub struct Bgcolr {
    register: VolatileCell<u32>,
}
#[doc = "background color register"]
pub mod bgcolr;
#[doc = "foreground CLUT memory address register"]
pub struct Fgcmar {
    register: VolatileCell<u32>,
}
#[doc = "foreground CLUT memory address register"]
pub mod fgcmar;
#[doc = "background CLUT memory address register"]
pub struct Bgcmar {
    register: VolatileCell<u32>,
}
#[doc = "background CLUT memory address register"]
pub mod bgcmar;
#[doc = "output PFC control register"]
pub struct Opfccr {
    register: VolatileCell<u32>,
}
#[doc = "output PFC control register"]
pub mod opfccr;
#[doc = "output color register"]
pub struct Ocolr {
    register: VolatileCell<u32>,
}
#[doc = "output color register"]
pub mod ocolr;
#[doc = "output memory address register"]
pub struct Omar {
    register: VolatileCell<u32>,
}
#[doc = "output memory address register"]
pub mod omar;
#[doc = "output offset register"]
pub struct Oor {
    register: VolatileCell<u32>,
}
#[doc = "output offset register"]
pub mod oor;
#[doc = "number of line register"]
pub struct Nlr {
    register: VolatileCell<u32>,
}
#[doc = "number of line register"]
pub mod nlr;
#[doc = "line watermark register"]
pub struct Lwr {
    register: VolatileCell<u32>,
}
#[doc = "line watermark register"]
pub mod lwr;
#[doc = "AHB master timer configuration register"]
pub struct Amtcr {
    register: VolatileCell<u32>,
}
#[doc = "AHB master timer configuration register"]
pub mod amtcr;
#[doc = "FGCLUT"]
pub struct Fgclut {
    register: VolatileCell<u32>,
}
#[doc = "FGCLUT"]
pub mod fgclut;
#[doc = "BGCLUT"]
pub struct Bgclut {
    register: VolatileCell<u32>,
}
#[doc = "BGCLUT"]
pub mod bgclut;
