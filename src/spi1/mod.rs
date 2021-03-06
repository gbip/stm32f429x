use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"] pub cr1: Cr1,
    #[doc = "0x04 - control register 2"] pub cr2: Cr2,
    #[doc = "0x08 - status register"] pub sr: Sr,
    #[doc = "0x0c - data register"] pub dr: Dr,
    #[doc = "0x10 - CRC polynomial register"] pub crcpr: Crcpr,
    #[doc = "0x14 - RX CRC register"] pub rxcrcr: Rxcrcr,
    #[doc = "0x18 - TX CRC register"] pub txcrcr: Txcrcr,
    #[doc = "0x1c - I2S configuration register"] pub i2scfgr: I2scfgr,
    #[doc = "0x20 - I2S prescaler register"] pub i2spr: I2spr,
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
#[doc = "status register"]
pub struct Sr {
    register: VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "data register"]
pub struct Dr {
    register: VolatileCell<u32>,
}
#[doc = "data register"]
pub mod dr;
#[doc = "CRC polynomial register"]
pub struct Crcpr {
    register: VolatileCell<u32>,
}
#[doc = "CRC polynomial register"]
pub mod crcpr;
#[doc = "RX CRC register"]
pub struct Rxcrcr {
    register: VolatileCell<u32>,
}
#[doc = "RX CRC register"]
pub mod rxcrcr;
#[doc = "TX CRC register"]
pub struct Txcrcr {
    register: VolatileCell<u32>,
}
#[doc = "TX CRC register"]
pub mod txcrcr;
#[doc = "I2S configuration register"]
pub struct I2scfgr {
    register: VolatileCell<u32>,
}
#[doc = "I2S configuration register"]
pub mod i2scfgr;
#[doc = "I2S prescaler register"]
pub struct I2spr {
    register: VolatileCell<u32>,
}
#[doc = "I2S prescaler register"]
pub mod i2spr;
