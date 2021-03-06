use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"] pub cr1: Cr1,
    #[doc = "0x04 - Control register 2"] pub cr2: Cr2,
    #[doc = "0x08 - Own address register 1"] pub oar1: Oar1,
    #[doc = "0x0c - Own address register 2"] pub oar2: Oar2,
    #[doc = "0x10 - Data register"] pub dr: Dr,
    #[doc = "0x14 - Status register 1"] pub sr1: Sr1,
    #[doc = "0x18 - Status register 2"] pub sr2: Sr2,
    #[doc = "0x1c - Clock control register"] pub ccr: Ccr,
    #[doc = "0x20 - TRISE register"] pub trise: Trise,
    #[doc = "0x24 - I2C FLTR register"] pub fltr: Fltr,
}
#[doc = "Control register 1"]
pub struct Cr1 {
    register: VolatileCell<u32>,
}
#[doc = "Control register 1"]
pub mod cr1;
#[doc = "Control register 2"]
pub struct Cr2 {
    register: VolatileCell<u32>,
}
#[doc = "Control register 2"]
pub mod cr2;
#[doc = "Own address register 1"]
pub struct Oar1 {
    register: VolatileCell<u32>,
}
#[doc = "Own address register 1"]
pub mod oar1;
#[doc = "Own address register 2"]
pub struct Oar2 {
    register: VolatileCell<u32>,
}
#[doc = "Own address register 2"]
pub mod oar2;
#[doc = "Data register"]
pub struct Dr {
    register: VolatileCell<u32>,
}
#[doc = "Data register"]
pub mod dr;
#[doc = "Status register 1"]
pub struct Sr1 {
    register: VolatileCell<u32>,
}
#[doc = "Status register 1"]
pub mod sr1;
#[doc = "Status register 2"]
pub struct Sr2 {
    register: VolatileCell<u32>,
}
#[doc = "Status register 2"]
pub mod sr2;
#[doc = "Clock control register"]
pub struct Ccr {
    register: VolatileCell<u32>,
}
#[doc = "Clock control register"]
pub mod ccr;
#[doc = "TRISE register"]
pub struct Trise {
    register: VolatileCell<u32>,
}
#[doc = "TRISE register"]
pub mod trise;
#[doc = "I2C FLTR register"]
pub struct Fltr {
    register: VolatileCell<u32>,
}
#[doc = "I2C FLTR register"]
pub mod fltr;
