use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"] pub cr: Cr,
    #[doc = "0x04 - status register"] pub sr: Sr,
    #[doc = "0x08 - raw interrupt status register"] pub ris: Ris,
    #[doc = "0x0c - interrupt enable register"] pub ier: Ier,
    #[doc = "0x10 - masked interrupt status register"] pub mis: Mis,
    #[doc = "0x14 - interrupt clear register"] pub icr: Icr,
    #[doc = "0x18 - embedded synchronization code register"] pub escr: Escr,
    #[doc = "0x1c - embedded synchronization unmask register"] pub esur: Esur,
    #[doc = "0x20 - crop window start"] pub cwstrt: Cwstrt,
    #[doc = "0x24 - crop window size"] pub cwsize: Cwsize,
    #[doc = "0x28 - data register"] pub dr: Dr,
}
#[doc = "control register 1"]
pub struct Cr {
    register: VolatileCell<u32>,
}
#[doc = "control register 1"]
pub mod cr;
#[doc = "status register"]
pub struct Sr {
    register: VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "raw interrupt status register"]
pub struct Ris {
    register: VolatileCell<u32>,
}
#[doc = "raw interrupt status register"]
pub mod ris;
#[doc = "interrupt enable register"]
pub struct Ier {
    register: VolatileCell<u32>,
}
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "masked interrupt status register"]
pub struct Mis {
    register: VolatileCell<u32>,
}
#[doc = "masked interrupt status register"]
pub mod mis;
#[doc = "interrupt clear register"]
pub struct Icr {
    register: VolatileCell<u32>,
}
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "embedded synchronization code register"]
pub struct Escr {
    register: VolatileCell<u32>,
}
#[doc = "embedded synchronization code register"]
pub mod escr;
#[doc = "embedded synchronization unmask register"]
pub struct Esur {
    register: VolatileCell<u32>,
}
#[doc = "embedded synchronization unmask register"]
pub mod esur;
#[doc = "crop window start"]
pub struct Cwstrt {
    register: VolatileCell<u32>,
}
#[doc = "crop window start"]
pub mod cwstrt;
#[doc = "crop window size"]
pub struct Cwsize {
    register: VolatileCell<u32>,
}
#[doc = "crop window size"]
pub mod cwsize;
#[doc = "data register"]
pub struct Dr {
    register: VolatileCell<u32>,
}
#[doc = "data register"]
pub mod dr;
