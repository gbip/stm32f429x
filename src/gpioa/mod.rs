use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"] pub moder: Moder,
    #[doc = "0x04 - GPIO port output type register"] pub otyper: Otyper,
    #[doc = "0x08 - GPIO port output speed register"] pub ospeedr: Ospeedr,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"] pub pupdr: Pupdr,
    #[doc = "0x10 - GPIO port input data register"] pub idr: Idr,
    #[doc = "0x14 - GPIO port output data register"] pub odr: Odr,
    #[doc = "0x18 - GPIO port bit set/reset register"] pub bsrr: Bsrr,
    #[doc = "0x1c - GPIO port configuration lock register"] pub lckr: Lckr,
    #[doc = "0x20 - GPIO alternate function low register"] pub afrl: Afrl,
    #[doc = "0x24 - GPIO alternate function high register"] pub afrh: Afrh,
}
#[doc = "GPIO port mode register"]
pub struct Moder {
    register: VolatileCell<u32>,
}
#[doc = "GPIO port mode register"]
pub mod moder;
#[doc = "GPIO port output type register"]
pub struct Otyper {
    register: VolatileCell<u32>,
}
#[doc = "GPIO port output type register"]
pub mod otyper;
#[doc = "GPIO port output speed register"]
pub struct Ospeedr {
    register: VolatileCell<u32>,
}
#[doc = "GPIO port output speed register"]
pub mod ospeedr;
#[doc = "GPIO port pull-up/pull-down register"]
pub struct Pupdr {
    register: VolatileCell<u32>,
}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod pupdr;
#[doc = "GPIO port input data register"]
pub struct Idr {
    register: VolatileCell<u32>,
}
#[doc = "GPIO port input data register"]
pub mod idr;
#[doc = "GPIO port output data register"]
pub struct Odr {
    register: VolatileCell<u32>,
}
#[doc = "GPIO port output data register"]
pub mod odr;
#[doc = "GPIO port bit set/reset register"]
pub struct Bsrr {
    register: VolatileCell<u32>,
}
#[doc = "GPIO port bit set/reset register"]
pub mod bsrr;
#[doc = "GPIO port configuration lock register"]
pub struct Lckr {
    register: VolatileCell<u32>,
}
#[doc = "GPIO port configuration lock register"]
pub mod lckr;
#[doc = "GPIO alternate function low register"]
pub struct Afrl {
    register: VolatileCell<u32>,
}
#[doc = "GPIO alternate function low register"]
pub mod afrl;
#[doc = "GPIO alternate function high register"]
pub struct Afrh {
    register: VolatileCell<u32>,
}
#[doc = "GPIO alternate function high register"]
pub mod afrh;
