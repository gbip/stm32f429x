use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash access control register"] pub acr: Acr,
    #[doc = "0x04 - Flash key register"] pub keyr: Keyr,
    #[doc = "0x08 - Flash option key register"] pub optkeyr: Optkeyr,
    #[doc = "0x0c - Status register"] pub sr: Sr,
    #[doc = "0x10 - Control register"] pub cr: Cr,
    #[doc = "0x14 - Flash option control register"] pub optcr: Optcr,
    #[doc = "0x18 - Flash option control register 1"] pub optcr1: Optcr1,
}
#[doc = "Flash access control register"]
pub struct Acr {
    register: VolatileCell<u32>,
}
#[doc = "Flash access control register"]
pub mod acr;
#[doc = "Flash key register"]
pub struct Keyr {
    register: VolatileCell<u32>,
}
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "Flash option key register"]
pub struct Optkeyr {
    register: VolatileCell<u32>,
}
#[doc = "Flash option key register"]
pub mod optkeyr;
#[doc = "Status register"]
pub struct Sr {
    register: VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod sr;
#[doc = "Control register"]
pub struct Cr {
    register: VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod cr;
#[doc = "Flash option control register"]
pub struct Optcr {
    register: VolatileCell<u32>,
}
#[doc = "Flash option control register"]
pub mod optcr;
#[doc = "Flash option control register 1"]
pub struct Optcr1 {
    register: VolatileCell<u32>,
}
#[doc = "Flash option control register 1"]
pub mod optcr1;
