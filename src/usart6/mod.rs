use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status register"] pub sr: Sr,
    #[doc = "0x04 - Data register"] pub dr: Dr,
    #[doc = "0x08 - Baud rate register"] pub brr: Brr,
    #[doc = "0x0c - Control register 1"] pub cr1: Cr1,
    #[doc = "0x10 - Control register 2"] pub cr2: Cr2,
    #[doc = "0x14 - Control register 3"] pub cr3: Cr3,
    #[doc = "0x18 - Guard time and prescaler register"] pub gtpr: Gtpr,
}
#[doc = "Status register"]
pub struct Sr {
    register: VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod sr;
#[doc = "Data register"]
pub struct Dr {
    register: VolatileCell<u32>,
}
#[doc = "Data register"]
pub mod dr;
#[doc = "Baud rate register"]
pub struct Brr {
    register: VolatileCell<u32>,
}
#[doc = "Baud rate register"]
pub mod brr;
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
#[doc = "Control register 3"]
pub struct Cr3 {
    register: VolatileCell<u32>,
}
#[doc = "Control register 3"]
pub mod cr3;
#[doc = "Guard time and prescaler register"]
pub struct Gtpr {
    register: VolatileCell<u32>,
}
#[doc = "Guard time and prescaler register"]
pub mod gtpr;
