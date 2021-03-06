use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MMC control register"] pub mmccr: Mmccr,
    #[doc = "0x04 - Ethernet MMC receive interrupt register"] pub mmcrir: Mmcrir,
    #[doc = "0x08 - Ethernet MMC transmit interrupt register"] pub mmctir: Mmctir,
    #[doc = "0x0c - Ethernet MMC receive interrupt mask register"] pub mmcrimr: Mmcrimr,
    #[doc = "0x10 - Ethernet MMC transmit interrupt mask register"] pub mmctimr: Mmctimr,
    _reserved0: [u8; 56usize],
    #[doc = "0x4c - Ethernet MMC transmitted good frames after a single collision counter"]
    pub mmctgfsccr: Mmctgfsccr,
    #[doc = "0x50 - Ethernet MMC transmitted good frames after more than a single collision"]
    pub mmctgfmsccr: Mmctgfmsccr,
    _reserved1: [u8; 20usize],
    #[doc = "0x68 - Ethernet MMC transmitted good frames counter register"] pub mmctgfcr: Mmctgfcr,
    _reserved2: [u8; 40usize],
    #[doc = "0x94 - Ethernet MMC received frames with CRC error counter register"]
    pub mmcrfcecr: Mmcrfcecr,
    #[doc = "0x98 - Ethernet MMC received frames with alignment error counter register"]
    pub mmcrfaecr: Mmcrfaecr,
    _reserved3: [u8; 40usize],
    #[doc = "0xc4 - MMC received good unicast frames counter register"] pub mmcrgufcr: Mmcrgufcr,
}
#[doc = "Ethernet MMC control register"]
pub struct Mmccr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MMC control register"]
pub mod mmccr;
#[doc = "Ethernet MMC receive interrupt register"]
pub struct Mmcrir {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MMC receive interrupt register"]
pub mod mmcrir;
#[doc = "Ethernet MMC transmit interrupt register"]
pub struct Mmctir {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MMC transmit interrupt register"]
pub mod mmctir;
#[doc = "Ethernet MMC receive interrupt mask register"]
pub struct Mmcrimr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MMC receive interrupt mask register"]
pub mod mmcrimr;
#[doc = "Ethernet MMC transmit interrupt mask register"]
pub struct Mmctimr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MMC transmit interrupt mask register"]
pub mod mmctimr;
#[doc = "Ethernet MMC transmitted good frames after a single collision counter"]
pub struct Mmctgfsccr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MMC transmitted good frames after a single collision counter"]
pub mod mmctgfsccr;
#[doc = "Ethernet MMC transmitted good frames after more than a single collision"]
pub struct Mmctgfmsccr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MMC transmitted good frames after more than a single collision"]
pub mod mmctgfmsccr;
#[doc = "Ethernet MMC transmitted good frames counter register"]
pub struct Mmctgfcr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MMC transmitted good frames counter register"]
pub mod mmctgfcr;
#[doc = "Ethernet MMC received frames with CRC error counter register"]
pub struct Mmcrfcecr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MMC received frames with CRC error counter register"]
pub mod mmcrfcecr;
#[doc = "Ethernet MMC received frames with alignment error counter register"]
pub struct Mmcrfaecr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MMC received frames with alignment error counter register"]
pub mod mmcrfaecr;
#[doc = "MMC received good unicast frames counter register"]
pub struct Mmcrgufcr {
    register: VolatileCell<u32>,
}
#[doc = "MMC received good unicast frames counter register"]
pub mod mmcrgufcr;
