use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - status register"] pub sr: Sr,
    #[doc = "0x04 - control register 1"] pub cr1: Cr1,
    #[doc = "0x08 - control register 2"] pub cr2: Cr2,
    #[doc = "0x0c - sample time register 1"] pub smpr1: Smpr1,
    #[doc = "0x10 - sample time register 2"] pub smpr2: Smpr2,
    #[doc = "0x14 - injected channel data offset register x"] pub jofr1: Jofr1,
    #[doc = "0x18 - injected channel data offset register x"] pub jofr2: Jofr2,
    #[doc = "0x1c - injected channel data offset register x"] pub jofr3: Jofr3,
    #[doc = "0x20 - injected channel data offset register x"] pub jofr4: Jofr4,
    #[doc = "0x24 - watchdog higher threshold register"] pub htr: Htr,
    #[doc = "0x28 - watchdog lower threshold register"] pub ltr: Ltr,
    #[doc = "0x2c - regular sequence register 1"] pub sqr1: Sqr1,
    #[doc = "0x30 - regular sequence register 2"] pub sqr2: Sqr2,
    #[doc = "0x34 - regular sequence register 3"] pub sqr3: Sqr3,
    #[doc = "0x38 - injected sequence register"] pub jsqr: Jsqr,
    #[doc = "0x3c - injected data register x"] pub jdr1: Jdr1,
    #[doc = "0x40 - injected data register x"] pub jdr2: Jdr2,
    #[doc = "0x44 - injected data register x"] pub jdr3: Jdr3,
    #[doc = "0x48 - injected data register x"] pub jdr4: Jdr4,
    #[doc = "0x4c - regular data register"] pub dr: Dr,
}
#[doc = "status register"]
pub struct Sr {
    register: VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
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
#[doc = "sample time register 1"]
pub struct Smpr1 {
    register: VolatileCell<u32>,
}
#[doc = "sample time register 1"]
pub mod smpr1;
#[doc = "sample time register 2"]
pub struct Smpr2 {
    register: VolatileCell<u32>,
}
#[doc = "sample time register 2"]
pub mod smpr2;
#[doc = "injected channel data offset register x"]
pub struct Jofr1 {
    register: VolatileCell<u32>,
}
#[doc = "injected channel data offset register x"]
pub mod jofr1;
#[doc = "injected channel data offset register x"]
pub struct Jofr2 {
    register: VolatileCell<u32>,
}
#[doc = "injected channel data offset register x"]
pub mod jofr2;
#[doc = "injected channel data offset register x"]
pub struct Jofr3 {
    register: VolatileCell<u32>,
}
#[doc = "injected channel data offset register x"]
pub mod jofr3;
#[doc = "injected channel data offset register x"]
pub struct Jofr4 {
    register: VolatileCell<u32>,
}
#[doc = "injected channel data offset register x"]
pub mod jofr4;
#[doc = "watchdog higher threshold register"]
pub struct Htr {
    register: VolatileCell<u32>,
}
#[doc = "watchdog higher threshold register"]
pub mod htr;
#[doc = "watchdog lower threshold register"]
pub struct Ltr {
    register: VolatileCell<u32>,
}
#[doc = "watchdog lower threshold register"]
pub mod ltr;
#[doc = "regular sequence register 1"]
pub struct Sqr1 {
    register: VolatileCell<u32>,
}
#[doc = "regular sequence register 1"]
pub mod sqr1;
#[doc = "regular sequence register 2"]
pub struct Sqr2 {
    register: VolatileCell<u32>,
}
#[doc = "regular sequence register 2"]
pub mod sqr2;
#[doc = "regular sequence register 3"]
pub struct Sqr3 {
    register: VolatileCell<u32>,
}
#[doc = "regular sequence register 3"]
pub mod sqr3;
#[doc = "injected sequence register"]
pub struct Jsqr {
    register: VolatileCell<u32>,
}
#[doc = "injected sequence register"]
pub mod jsqr;
#[doc = "injected data register x"]
pub struct Jdr1 {
    register: VolatileCell<u32>,
}
#[doc = "injected data register x"]
pub mod jdr1;
#[doc = "injected data register x"]
pub struct Jdr2 {
    register: VolatileCell<u32>,
}
#[doc = "injected data register x"]
pub mod jdr2;
#[doc = "injected data register x"]
pub struct Jdr3 {
    register: VolatileCell<u32>,
}
#[doc = "injected data register x"]
pub mod jdr3;
#[doc = "injected data register x"]
pub struct Jdr4 {
    register: VolatileCell<u32>,
}
#[doc = "injected data register x"]
pub mod jdr4;
#[doc = "regular data register"]
pub struct Dr {
    register: VolatileCell<u32>,
}
#[doc = "regular data register"]
pub mod dr;
