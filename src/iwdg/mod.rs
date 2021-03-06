use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key register"] pub kr: Kr,
    #[doc = "0x04 - Prescaler register"] pub pr: Pr,
    #[doc = "0x08 - Reload register"] pub rlr: Rlr,
    #[doc = "0x0c - Status register"] pub sr: Sr,
}
#[doc = "Key register"]
pub struct Kr {
    register: VolatileCell<u32>,
}
#[doc = "Key register"]
pub mod kr;
#[doc = "Prescaler register"]
pub struct Pr {
    register: VolatileCell<u32>,
}
#[doc = "Prescaler register"]
pub mod pr;
#[doc = "Reload register"]
pub struct Rlr {
    register: VolatileCell<u32>,
}
#[doc = "Reload register"]
pub mod rlr;
#[doc = "Status register"]
pub struct Sr {
    register: VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod sr;
