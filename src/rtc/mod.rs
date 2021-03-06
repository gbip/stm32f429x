use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - time register"] pub tr: Tr,
    #[doc = "0x04 - date register"] pub dr: Dr,
    #[doc = "0x08 - control register"] pub cr: Cr,
    #[doc = "0x0c - initialization and status register"] pub isr: Isr,
    #[doc = "0x10 - prescaler register"] pub prer: Prer,
    #[doc = "0x14 - wakeup timer register"] pub wutr: Wutr,
    #[doc = "0x18 - calibration register"] pub calibr: Calibr,
    #[doc = "0x1c - alarm A register"] pub alrmar: Alrmar,
    #[doc = "0x20 - alarm B register"] pub alrmbr: Alrmbr,
    #[doc = "0x24 - write protection register"] pub wpr: Wpr,
    #[doc = "0x28 - sub second register"] pub ssr: Ssr,
    #[doc = "0x2c - shift control register"] pub shiftr: Shiftr,
    #[doc = "0x30 - time stamp time register"] pub tstr: Tstr,
    #[doc = "0x34 - time stamp date register"] pub tsdr: Tsdr,
    #[doc = "0x38 - timestamp sub second register"] pub tsssr: Tsssr,
    #[doc = "0x3c - calibration register"] pub calr: Calr,
    #[doc = "0x40 - tamper and alternate function configuration register"] pub tafcr: Tafcr,
    #[doc = "0x44 - alarm A sub second register"] pub alrmassr: Alrmassr,
    #[doc = "0x48 - alarm B sub second register"] pub alrmbssr: Alrmbssr,
    _reserved0: [u8; 4usize],
    #[doc = "0x50 - backup register"] pub bkp0r: Bkp0r,
    #[doc = "0x54 - backup register"] pub bkp1r: Bkp1r,
    #[doc = "0x58 - backup register"] pub bkp2r: Bkp2r,
    #[doc = "0x5c - backup register"] pub bkp3r: Bkp3r,
    #[doc = "0x60 - backup register"] pub bkp4r: Bkp4r,
    #[doc = "0x64 - backup register"] pub bkp5r: Bkp5r,
    #[doc = "0x68 - backup register"] pub bkp6r: Bkp6r,
    #[doc = "0x6c - backup register"] pub bkp7r: Bkp7r,
    #[doc = "0x70 - backup register"] pub bkp8r: Bkp8r,
    #[doc = "0x74 - backup register"] pub bkp9r: Bkp9r,
    #[doc = "0x78 - backup register"] pub bkp10r: Bkp10r,
    #[doc = "0x7c - backup register"] pub bkp11r: Bkp11r,
    #[doc = "0x80 - backup register"] pub bkp12r: Bkp12r,
    #[doc = "0x84 - backup register"] pub bkp13r: Bkp13r,
    #[doc = "0x88 - backup register"] pub bkp14r: Bkp14r,
    #[doc = "0x8c - backup register"] pub bkp15r: Bkp15r,
    #[doc = "0x90 - backup register"] pub bkp16r: Bkp16r,
    #[doc = "0x94 - backup register"] pub bkp17r: Bkp17r,
    #[doc = "0x98 - backup register"] pub bkp18r: Bkp18r,
    #[doc = "0x9c - backup register"] pub bkp19r: Bkp19r,
}
#[doc = "time register"]
pub struct Tr {
    register: VolatileCell<u32>,
}
#[doc = "time register"]
pub mod tr;
#[doc = "date register"]
pub struct Dr {
    register: VolatileCell<u32>,
}
#[doc = "date register"]
pub mod dr;
#[doc = "control register"]
pub struct Cr {
    register: VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "initialization and status register"]
pub struct Isr {
    register: VolatileCell<u32>,
}
#[doc = "initialization and status register"]
pub mod isr;
#[doc = "prescaler register"]
pub struct Prer {
    register: VolatileCell<u32>,
}
#[doc = "prescaler register"]
pub mod prer;
#[doc = "wakeup timer register"]
pub struct Wutr {
    register: VolatileCell<u32>,
}
#[doc = "wakeup timer register"]
pub mod wutr;
#[doc = "calibration register"]
pub struct Calibr {
    register: VolatileCell<u32>,
}
#[doc = "calibration register"]
pub mod calibr;
#[doc = "alarm A register"]
pub struct Alrmar {
    register: VolatileCell<u32>,
}
#[doc = "alarm A register"]
pub mod alrmar;
#[doc = "alarm B register"]
pub struct Alrmbr {
    register: VolatileCell<u32>,
}
#[doc = "alarm B register"]
pub mod alrmbr;
#[doc = "write protection register"]
pub struct Wpr {
    register: VolatileCell<u32>,
}
#[doc = "write protection register"]
pub mod wpr;
#[doc = "sub second register"]
pub struct Ssr {
    register: VolatileCell<u32>,
}
#[doc = "sub second register"]
pub mod ssr;
#[doc = "shift control register"]
pub struct Shiftr {
    register: VolatileCell<u32>,
}
#[doc = "shift control register"]
pub mod shiftr;
#[doc = "time stamp time register"]
pub struct Tstr {
    register: VolatileCell<u32>,
}
#[doc = "time stamp time register"]
pub mod tstr;
#[doc = "time stamp date register"]
pub struct Tsdr {
    register: VolatileCell<u32>,
}
#[doc = "time stamp date register"]
pub mod tsdr;
#[doc = "timestamp sub second register"]
pub struct Tsssr {
    register: VolatileCell<u32>,
}
#[doc = "timestamp sub second register"]
pub mod tsssr;
#[doc = "calibration register"]
pub struct Calr {
    register: VolatileCell<u32>,
}
#[doc = "calibration register"]
pub mod calr;
#[doc = "tamper and alternate function configuration register"]
pub struct Tafcr {
    register: VolatileCell<u32>,
}
#[doc = "tamper and alternate function configuration register"]
pub mod tafcr;
#[doc = "alarm A sub second register"]
pub struct Alrmassr {
    register: VolatileCell<u32>,
}
#[doc = "alarm A sub second register"]
pub mod alrmassr;
#[doc = "alarm B sub second register"]
pub struct Alrmbssr {
    register: VolatileCell<u32>,
}
#[doc = "alarm B sub second register"]
pub mod alrmbssr;
#[doc = "backup register"]
pub struct Bkp0r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp0r;
#[doc = "backup register"]
pub struct Bkp1r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp1r;
#[doc = "backup register"]
pub struct Bkp2r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp2r;
#[doc = "backup register"]
pub struct Bkp3r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp3r;
#[doc = "backup register"]
pub struct Bkp4r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp4r;
#[doc = "backup register"]
pub struct Bkp5r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp5r;
#[doc = "backup register"]
pub struct Bkp6r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp6r;
#[doc = "backup register"]
pub struct Bkp7r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp7r;
#[doc = "backup register"]
pub struct Bkp8r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp8r;
#[doc = "backup register"]
pub struct Bkp9r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp9r;
#[doc = "backup register"]
pub struct Bkp10r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp10r;
#[doc = "backup register"]
pub struct Bkp11r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp11r;
#[doc = "backup register"]
pub struct Bkp12r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp12r;
#[doc = "backup register"]
pub struct Bkp13r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp13r;
#[doc = "backup register"]
pub struct Bkp14r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp14r;
#[doc = "backup register"]
pub struct Bkp15r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp15r;
#[doc = "backup register"]
pub struct Bkp16r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp16r;
#[doc = "backup register"]
pub struct Bkp17r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp17r;
#[doc = "backup register"]
pub struct Bkp18r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp18r;
#[doc = "backup register"]
pub struct Bkp19r {
    register: VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp19r;
