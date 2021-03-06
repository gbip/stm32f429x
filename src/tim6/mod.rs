use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"] pub cr1: Cr1,
    #[doc = "0x04 - control register 2"] pub cr2: Cr2,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - DMA/Interrupt enable register"] pub dier: Dier,
    #[doc = "0x10 - status register"] pub sr: Sr,
    #[doc = "0x14 - event generation register"] pub egr: Egr,
    _reserved1: [u8; 12usize],
    #[doc = "0x24 - counter"] pub cnt: Cnt,
    #[doc = "0x28 - prescaler"] pub psc: Psc,
    #[doc = "0x2c - auto-reload register"] pub arr: Arr,
}
#[doc = "control register 1"]
pub struct Cr1 {
    register: VolatileCell<u32>,
}
#[doc = "control register 1"]
pub mod cr1;
#[doc = "control register 2"]
pub struct Cr2 {
    register: VolatileCell<u32>,
}
#[doc = "control register 2"]
pub mod cr2;
#[doc = "DMA/Interrupt enable register"]
pub struct Dier {
    register: VolatileCell<u32>,
}
#[doc = "DMA/Interrupt enable register"]
pub mod dier;
#[doc = "status register"]
pub struct Sr {
    register: VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "event generation register"]
pub struct Egr {
    register: VolatileCell<u32>,
}
#[doc = "event generation register"]
pub mod egr;
#[doc = "counter"]
pub struct Cnt {
    register: VolatileCell<u32>,
}
#[doc = "counter"]
pub mod cnt;
#[doc = "prescaler"]
pub struct Psc {
    register: VolatileCell<u32>,
}
#[doc = "prescaler"]
pub mod psc;
#[doc = "auto-reload register"]
pub struct Arr {
    register: VolatileCell<u32>,
}
#[doc = "auto-reload register"]
pub mod arr;
