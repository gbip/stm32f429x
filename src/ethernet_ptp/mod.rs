use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet PTP time stamp control register"] pub ptptscr: Ptptscr,
    #[doc = "0x04 - Ethernet PTP subsecond increment register"] pub ptpssir: Ptpssir,
    #[doc = "0x08 - Ethernet PTP time stamp high register"] pub ptptshr: Ptptshr,
    #[doc = "0x0c - Ethernet PTP time stamp low register"] pub ptptslr: Ptptslr,
    #[doc = "0x10 - Ethernet PTP time stamp high update register"] pub ptptshur: Ptptshur,
    #[doc = "0x14 - Ethernet PTP time stamp low update register"] pub ptptslur: Ptptslur,
    #[doc = "0x18 - Ethernet PTP time stamp addend register"] pub ptptsar: Ptptsar,
    #[doc = "0x1c - Ethernet PTP target time high register"] pub ptptthr: Ptptthr,
    #[doc = "0x20 - Ethernet PTP target time low register"] pub ptpttlr: Ptpttlr,
    _reserved0: [u8; 4usize],
    #[doc = "0x28 - Ethernet PTP time stamp status register"] pub ptptssr: Ptptssr,
    #[doc = "0x2c - Ethernet PTP PPS control register"] pub ptpppscr: Ptpppscr,
}
#[doc = "Ethernet PTP time stamp control register"]
pub struct Ptptscr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet PTP time stamp control register"]
pub mod ptptscr;
#[doc = "Ethernet PTP subsecond increment register"]
pub struct Ptpssir {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet PTP subsecond increment register"]
pub mod ptpssir;
#[doc = "Ethernet PTP time stamp high register"]
pub struct Ptptshr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet PTP time stamp high register"]
pub mod ptptshr;
#[doc = "Ethernet PTP time stamp low register"]
pub struct Ptptslr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet PTP time stamp low register"]
pub mod ptptslr;
#[doc = "Ethernet PTP time stamp high update register"]
pub struct Ptptshur {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet PTP time stamp high update register"]
pub mod ptptshur;
#[doc = "Ethernet PTP time stamp low update register"]
pub struct Ptptslur {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet PTP time stamp low update register"]
pub mod ptptslur;
#[doc = "Ethernet PTP time stamp addend register"]
pub struct Ptptsar {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet PTP time stamp addend register"]
pub mod ptptsar;
#[doc = "Ethernet PTP target time high register"]
pub struct Ptptthr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet PTP target time high register"]
pub mod ptptthr;
#[doc = "Ethernet PTP target time low register"]
pub struct Ptpttlr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet PTP target time low register"]
pub mod ptpttlr;
#[doc = "Ethernet PTP time stamp status register"]
pub struct Ptptssr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet PTP time stamp status register"]
pub mod ptptssr;
#[doc = "Ethernet PTP PPS control register"]
pub struct Ptpppscr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet PTP PPS control register"]
pub mod ptpppscr;
