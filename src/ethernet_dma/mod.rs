use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock { # [ doc = "0x00 - Ethernet DMA bus mode register" ] pub dmabmr : Dmabmr , # [ doc = "0x04 - Ethernet DMA transmit poll demand register" ] pub dmatpdr : Dmatpdr , # [ doc = "0x08 - EHERNET DMA receive poll demand register" ] pub dmarpdr : Dmarpdr , # [ doc = "0x0c - Ethernet DMA receive descriptor list address register" ] pub dmardlar : Dmardlar , # [ doc = "0x10 - Ethernet DMA transmit descriptor list address register" ] pub dmatdlar : Dmatdlar , # [ doc = "0x14 - Ethernet DMA status register" ] pub dmasr : Dmasr , # [ doc = "0x18 - Ethernet DMA operation mode register" ] pub dmaomr : Dmaomr , # [ doc = "0x1c - Ethernet DMA interrupt enable register" ] pub dmaier : Dmaier , # [ doc = "0x20 - Ethernet DMA missed frame and buffer overflow counter register" ] pub dmamfbocr : Dmamfbocr , # [ doc = "0x24 - Ethernet DMA receive status watchdog timer register" ] pub dmarswtr : Dmarswtr , _reserved0 : [ u8 ; 32usize ] , # [ doc = "0x48 - Ethernet DMA current host transmit descriptor register" ] pub dmachtdr : Dmachtdr , # [ doc = "0x4c - Ethernet DMA current host receive descriptor register" ] pub dmachrdr : Dmachrdr , # [ doc = "0x50 - Ethernet DMA current host transmit buffer address register" ] pub dmachtbar : Dmachtbar , # [ doc = "0x54 - Ethernet DMA current host receive buffer address register" ] pub dmachrbar : Dmachrbar }
#[doc = "Ethernet DMA bus mode register"]
pub struct Dmabmr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet DMA bus mode register"]
pub mod dmabmr;
#[doc = "Ethernet DMA transmit poll demand register"]
pub struct Dmatpdr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet DMA transmit poll demand register"]
pub mod dmatpdr;
#[doc = "EHERNET DMA receive poll demand register"]
pub struct Dmarpdr {
    register: VolatileCell<u32>,
}
#[doc = "EHERNET DMA receive poll demand register"]
pub mod dmarpdr;
#[doc = "Ethernet DMA receive descriptor list address register"]
pub struct Dmardlar {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet DMA receive descriptor list address register"]
pub mod dmardlar;
#[doc = "Ethernet DMA transmit descriptor list address register"]
pub struct Dmatdlar {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet DMA transmit descriptor list address register"]
pub mod dmatdlar;
#[doc = "Ethernet DMA status register"]
pub struct Dmasr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet DMA status register"]
pub mod dmasr;
#[doc = "Ethernet DMA operation mode register"]
pub struct Dmaomr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet DMA operation mode register"]
pub mod dmaomr;
#[doc = "Ethernet DMA interrupt enable register"]
pub struct Dmaier {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet DMA interrupt enable register"]
pub mod dmaier;
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub struct Dmamfbocr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub mod dmamfbocr;
#[doc = "Ethernet DMA receive status watchdog timer register"]
pub struct Dmarswtr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet DMA receive status watchdog timer register"]
pub mod dmarswtr;
#[doc = "Ethernet DMA current host transmit descriptor register"]
pub struct Dmachtdr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet DMA current host transmit descriptor register"]
pub mod dmachtdr;
#[doc = "Ethernet DMA current host receive descriptor register"]
pub struct Dmachrdr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet DMA current host receive descriptor register"]
pub mod dmachrdr;
#[doc = "Ethernet DMA current host transmit buffer address register"]
pub struct Dmachtbar {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet DMA current host transmit buffer address register"]
pub mod dmachtbar;
#[doc = "Ethernet DMA current host receive buffer address register"]
pub struct Dmachrbar {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet DMA current host receive buffer address register"]
pub mod dmachrbar;
