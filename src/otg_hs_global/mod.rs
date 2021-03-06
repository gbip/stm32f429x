use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS control and status register"] pub otg_hs_gotgctl: OtgHsGotgctl,
    #[doc = "0x04 - OTG_HS interrupt register"] pub otg_hs_gotgint: OtgHsGotgint,
    #[doc = "0x08 - OTG_HS AHB configuration register"] pub otg_hs_gahbcfg: OtgHsGahbcfg,
    #[doc = "0x0c - OTG_HS USB configuration register"] pub otg_hs_gusbcfg: OtgHsGusbcfg,
    #[doc = "0x10 - OTG_HS reset register"] pub otg_hs_grstctl: OtgHsGrstctl,
    #[doc = "0x14 - OTG_HS core interrupt register"] pub otg_hs_gintsts: OtgHsGintsts,
    #[doc = "0x18 - OTG_HS interrupt mask register"] pub otg_hs_gintmsk: OtgHsGintmsk,
    #[doc = "0x1c - OTG_HS Receive status debug read register (host mode)"]
    pub otg_hs_grxstsr_host: OtgHsGrxstsrHost,
    #[doc = "0x20 - OTG_HS status read and pop register (host mode)"]
    pub otg_hs_grxstsp_host: OtgHsGrxstspHost,
    #[doc = "0x24 - OTG_HS Receive FIFO size register"] pub otg_hs_grxfsiz: OtgHsGrxfsiz,
    #[doc = "0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)"]
    pub otg_hs_gnptxfsiz_host: OtgHsGnptxfsizHost,
    #[doc = "0x2c - OTG_HS nonperiodic transmit FIFO/queue status register"]
    pub otg_hs_gnptxsts: OtgHsGnptxsts,
    _reserved0: [u8; 8usize],
    #[doc = "0x38 - OTG_HS general core configuration register"] pub otg_hs_gccfg: OtgHsGccfg,
    #[doc = "0x3c - OTG_HS core ID register"] pub otg_hs_cid: OtgHsCid,
    _reserved1: [u8; 192usize],
    #[doc = "0x100 - OTG_HS Host periodic transmit FIFO size register"]
    pub otg_hs_hptxfsiz: OtgHsHptxfsiz,
    #[doc = "0x104 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf1: OtgHsDieptxf1,
    #[doc = "0x108 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf2: OtgHsDieptxf2,
    _reserved2: [u8; 16usize],
    #[doc = "0x11c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf3: OtgHsDieptxf3,
    #[doc = "0x120 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf4: OtgHsDieptxf4,
    #[doc = "0x124 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf5: OtgHsDieptxf5,
    #[doc = "0x128 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf6: OtgHsDieptxf6,
    #[doc = "0x12c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf7: OtgHsDieptxf7,
}
#[doc = "OTG_HS control and status register"]
pub struct OtgHsGotgctl {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS control and status register"]
pub mod otg_hs_gotgctl;
#[doc = "OTG_HS interrupt register"]
pub struct OtgHsGotgint {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS interrupt register"]
pub mod otg_hs_gotgint;
#[doc = "OTG_HS AHB configuration register"]
pub struct OtgHsGahbcfg {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS AHB configuration register"]
pub mod otg_hs_gahbcfg;
#[doc = "OTG_HS USB configuration register"]
pub struct OtgHsGusbcfg {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS USB configuration register"]
pub mod otg_hs_gusbcfg;
#[doc = "OTG_HS reset register"]
pub struct OtgHsGrstctl {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS reset register"]
pub mod otg_hs_grstctl;
#[doc = "OTG_HS core interrupt register"]
pub struct OtgHsGintsts {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS core interrupt register"]
pub mod otg_hs_gintsts;
#[doc = "OTG_HS interrupt mask register"]
pub struct OtgHsGintmsk {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS interrupt mask register"]
pub mod otg_hs_gintmsk;
#[doc = "OTG_HS Receive status debug read register (host mode)"]
pub struct OtgHsGrxstsrHost {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS Receive status debug read register (host mode)"]
pub mod otg_hs_grxstsr_host;
#[doc = "OTG_HS status read and pop register (host mode)"]
pub struct OtgHsGrxstspHost {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS status read and pop register (host mode)"]
pub mod otg_hs_grxstsp_host;
#[doc = "OTG_HS Receive FIFO size register"]
pub struct OtgHsGrxfsiz {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS Receive FIFO size register"]
pub mod otg_hs_grxfsiz;
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)"]
pub struct OtgHsGnptxfsizHost {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)"]
pub mod otg_hs_gnptxfsiz_host;
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)"]
pub struct OtgHsTx0fsizPeripheral {
    register: VolatileCell<u32>,
}
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)"]
pub mod otg_hs_tx0fsiz_peripheral;
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register"]
pub struct OtgHsGnptxsts {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register"]
pub mod otg_hs_gnptxsts;
#[doc = "OTG_HS general core configuration register"]
pub struct OtgHsGccfg {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS general core configuration register"]
pub mod otg_hs_gccfg;
#[doc = "OTG_HS core ID register"]
pub struct OtgHsCid {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS core ID register"]
pub mod otg_hs_cid;
#[doc = "OTG_HS Host periodic transmit FIFO size register"]
pub struct OtgHsHptxfsiz {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS Host periodic transmit FIFO size register"]
pub mod otg_hs_hptxfsiz;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OtgHsDieptxf1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf1;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OtgHsDieptxf2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf2;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OtgHsDieptxf3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf3;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OtgHsDieptxf4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf4;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OtgHsDieptxf5 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf5;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OtgHsDieptxf6 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf6;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OtgHsDieptxf7 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf7;
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)"]
pub struct OtgHsGrxstsrPeripheral {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)"]
pub mod otg_hs_grxstsr_peripheral;
#[doc = "OTG_HS status read and pop register (peripheral mode)"]
pub struct OtgHsGrxstspPeripheral {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS status read and pop register (peripheral mode)"]
pub mod otg_hs_grxstsp_peripheral;
