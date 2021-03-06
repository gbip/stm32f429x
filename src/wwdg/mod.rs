use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"] pub cr: Cr,
    #[doc = "0x04 - Configuration register"] pub cfr: Cfr,
    #[doc = "0x08 - Status register"] pub sr: Sr,
}
#[doc = "Control register"]
pub struct Cr {
    register: VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod cr;
#[doc = "Configuration register"]
pub struct Cfr {
    register: VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod cfr;
#[doc = "Status register"]
pub struct Sr {
    register: VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod sr;
