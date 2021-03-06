use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - clock control register"] pub cr: Cr,
    #[doc = "0x04 - PLL configuration register"] pub pllcfgr: Pllcfgr,
    #[doc = "0x08 - clock configuration register"] pub cfgr: Cfgr,
    #[doc = "0x0c - clock interrupt register"] pub cir: Cir,
    #[doc = "0x10 - AHB1 peripheral reset register"] pub ahb1rstr: Ahb1rstr,
    #[doc = "0x14 - AHB2 peripheral reset register"] pub ahb2rstr: Ahb2rstr,
    #[doc = "0x18 - AHB3 peripheral reset register"] pub ahb3rstr: Ahb3rstr,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - APB1 peripheral reset register"] pub apb1rstr: Apb1rstr,
    #[doc = "0x24 - APB2 peripheral reset register"] pub apb2rstr: Apb2rstr,
    _reserved1: [u8; 8usize],
    #[doc = "0x30 - AHB1 peripheral clock register"] pub ahb1enr: Ahb1enr,
    #[doc = "0x34 - AHB2 peripheral clock enable register"] pub ahb2enr: Ahb2enr,
    #[doc = "0x38 - AHB3 peripheral clock enable register"] pub ahb3enr: Ahb3enr,
    _reserved2: [u8; 4usize],
    #[doc = "0x40 - APB1 peripheral clock enable register"] pub apb1enr: Apb1enr,
    #[doc = "0x44 - APB2 peripheral clock enable register"] pub apb2enr: Apb2enr,
    _reserved3: [u8; 8usize],
    #[doc = "0x50 - AHB1 peripheral clock enable in low power mode register"]
    pub ahb1lpenr: Ahb1lpenr,
    #[doc = "0x54 - AHB2 peripheral clock enable in low power mode register"]
    pub ahb2lpenr: Ahb2lpenr,
    #[doc = "0x58 - AHB3 peripheral clock enable in low power mode register"]
    pub ahb3lpenr: Ahb3lpenr,
    _reserved4: [u8; 4usize],
    #[doc = "0x60 - APB1 peripheral clock enable in low power mode register"]
    pub apb1lpenr: Apb1lpenr,
    #[doc = "0x64 - APB2 peripheral clock enabled in low power mode register"]
    pub apb2lpenr: Apb2lpenr,
    _reserved5: [u8; 8usize],
    #[doc = "0x70 - Backup domain control register"] pub bdcr: Bdcr,
    #[doc = "0x74 - clock control & status register"] pub csr: Csr,
    _reserved6: [u8; 8usize],
    #[doc = "0x80 - spread spectrum clock generation register"] pub sscgr: Sscgr,
    #[doc = "0x84 - PLLI2S configuration register"] pub plli2scfgr: Plli2scfgr,
    #[doc = "0x88 - PLLSAICFGR"] pub pllsaicfgr: Pllsaicfgr,
    #[doc = "0x8c - DCKCFGR"] pub dckcfgr: Dckcfgr,
}
#[doc = "clock control register"]
pub struct Cr {
    register: VolatileCell<u32>,
}
#[doc = "clock control register"]
pub mod cr;
#[doc = "PLL configuration register"]
pub struct Pllcfgr {
    register: VolatileCell<u32>,
}
#[doc = "PLL configuration register"]
pub mod pllcfgr;
#[doc = "clock configuration register"]
pub struct Cfgr {
    register: VolatileCell<u32>,
}
#[doc = "clock configuration register"]
pub mod cfgr;
#[doc = "clock interrupt register"]
pub struct Cir {
    register: VolatileCell<u32>,
}
#[doc = "clock interrupt register"]
pub mod cir;
#[doc = "AHB1 peripheral reset register"]
pub struct Ahb1rstr {
    register: VolatileCell<u32>,
}
#[doc = "AHB1 peripheral reset register"]
pub mod ahb1rstr;
#[doc = "AHB2 peripheral reset register"]
pub struct Ahb2rstr {
    register: VolatileCell<u32>,
}
#[doc = "AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "AHB3 peripheral reset register"]
pub struct Ahb3rstr {
    register: VolatileCell<u32>,
}
#[doc = "AHB3 peripheral reset register"]
pub mod ahb3rstr;
#[doc = "APB1 peripheral reset register"]
pub struct Apb1rstr {
    register: VolatileCell<u32>,
}
#[doc = "APB1 peripheral reset register"]
pub mod apb1rstr;
#[doc = "APB2 peripheral reset register"]
pub struct Apb2rstr {
    register: VolatileCell<u32>,
}
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "AHB1 peripheral clock register"]
pub struct Ahb1enr {
    register: VolatileCell<u32>,
}
#[doc = "AHB1 peripheral clock register"]
pub mod ahb1enr;
#[doc = "AHB2 peripheral clock enable register"]
pub struct Ahb2enr {
    register: VolatileCell<u32>,
}
#[doc = "AHB2 peripheral clock enable register"]
pub mod ahb2enr;
#[doc = "AHB3 peripheral clock enable register"]
pub struct Ahb3enr {
    register: VolatileCell<u32>,
}
#[doc = "AHB3 peripheral clock enable register"]
pub mod ahb3enr;
#[doc = "APB1 peripheral clock enable register"]
pub struct Apb1enr {
    register: VolatileCell<u32>,
}
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1enr;
#[doc = "APB2 peripheral clock enable register"]
pub struct Apb2enr {
    register: VolatileCell<u32>,
}
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "AHB1 peripheral clock enable in low power mode register"]
pub struct Ahb1lpenr {
    register: VolatileCell<u32>,
}
#[doc = "AHB1 peripheral clock enable in low power mode register"]
pub mod ahb1lpenr;
#[doc = "AHB2 peripheral clock enable in low power mode register"]
pub struct Ahb2lpenr {
    register: VolatileCell<u32>,
}
#[doc = "AHB2 peripheral clock enable in low power mode register"]
pub mod ahb2lpenr;
#[doc = "AHB3 peripheral clock enable in low power mode register"]
pub struct Ahb3lpenr {
    register: VolatileCell<u32>,
}
#[doc = "AHB3 peripheral clock enable in low power mode register"]
pub mod ahb3lpenr;
#[doc = "APB1 peripheral clock enable in low power mode register"]
pub struct Apb1lpenr {
    register: VolatileCell<u32>,
}
#[doc = "APB1 peripheral clock enable in low power mode register"]
pub mod apb1lpenr;
#[doc = "APB2 peripheral clock enabled in low power mode register"]
pub struct Apb2lpenr {
    register: VolatileCell<u32>,
}
#[doc = "APB2 peripheral clock enabled in low power mode register"]
pub mod apb2lpenr;
#[doc = "Backup domain control register"]
pub struct Bdcr {
    register: VolatileCell<u32>,
}
#[doc = "Backup domain control register"]
pub mod bdcr;
#[doc = "clock control & status register"]
pub struct Csr {
    register: VolatileCell<u32>,
}
#[doc = "clock control & status register"]
pub mod csr;
#[doc = "spread spectrum clock generation register"]
pub struct Sscgr {
    register: VolatileCell<u32>,
}
#[doc = "spread spectrum clock generation register"]
pub mod sscgr;
#[doc = "PLLI2S configuration register"]
pub struct Plli2scfgr {
    register: VolatileCell<u32>,
}
#[doc = "PLLI2S configuration register"]
pub mod plli2scfgr;
#[doc = "PLLSAICFGR"]
pub struct Pllsaicfgr {
    register: VolatileCell<u32>,
}
#[doc = "PLLSAICFGR"]
pub mod pllsaicfgr;
#[doc = "DCKCFGR"]
pub struct Dckcfgr {
    register: VolatileCell<u32>,
}
#[doc = "DCKCFGR"]
pub mod dckcfgr;
