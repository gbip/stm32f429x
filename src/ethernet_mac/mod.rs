use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register"] pub maccr: Maccr,
    #[doc = "0x04 - Ethernet MAC frame filter register"] pub macffr: Macffr,
    #[doc = "0x08 - Ethernet MAC hash table high register"] pub machthr: Machthr,
    #[doc = "0x0c - Ethernet MAC hash table low register"] pub machtlr: Machtlr,
    #[doc = "0x10 - Ethernet MAC MII address register"] pub macmiiar: Macmiiar,
    #[doc = "0x14 - Ethernet MAC MII data register"] pub macmiidr: Macmiidr,
    #[doc = "0x18 - Ethernet MAC flow control register"] pub macfcr: Macfcr,
    #[doc = "0x1c - Ethernet MAC VLAN tag register"] pub macvlantr: Macvlantr,
    _reserved0: [u8; 12usize],
    #[doc = "0x2c - Ethernet MAC PMT control and status register"] pub macpmtcsr: Macpmtcsr,
    _reserved1: [u8; 4usize],
    #[doc = "0x34 - Ethernet MAC debug register"] pub macdbgr: Macdbgr,
    #[doc = "0x38 - Ethernet MAC interrupt status register"] pub macsr: Macsr,
    #[doc = "0x3c - Ethernet MAC interrupt mask register"] pub macimr: Macimr,
    #[doc = "0x40 - Ethernet MAC address 0 high register"] pub maca0hr: Maca0hr,
    #[doc = "0x44 - Ethernet MAC address 0 low register"] pub maca0lr: Maca0lr,
    #[doc = "0x48 - Ethernet MAC address 1 high register"] pub maca1hr: Maca1hr,
    #[doc = "0x4c - Ethernet MAC address1 low register"] pub maca1lr: Maca1lr,
    #[doc = "0x50 - Ethernet MAC address 2 high register"] pub maca2hr: Maca2hr,
    #[doc = "0x54 - Ethernet MAC address 2 low register"] pub maca2lr: Maca2lr,
    #[doc = "0x58 - Ethernet MAC address 3 high register"] pub maca3hr: Maca3hr,
    #[doc = "0x5c - Ethernet MAC address 3 low register"] pub maca3lr: Maca3lr,
}
#[doc = "Ethernet MAC configuration register"]
pub struct Maccr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC configuration register"]
pub mod maccr;
#[doc = "Ethernet MAC frame filter register"]
pub struct Macffr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC frame filter register"]
pub mod macffr;
#[doc = "Ethernet MAC hash table high register"]
pub struct Machthr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC hash table high register"]
pub mod machthr;
#[doc = "Ethernet MAC hash table low register"]
pub struct Machtlr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC hash table low register"]
pub mod machtlr;
#[doc = "Ethernet MAC MII address register"]
pub struct Macmiiar {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC MII address register"]
pub mod macmiiar;
#[doc = "Ethernet MAC MII data register"]
pub struct Macmiidr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC MII data register"]
pub mod macmiidr;
#[doc = "Ethernet MAC flow control register"]
pub struct Macfcr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC flow control register"]
pub mod macfcr;
#[doc = "Ethernet MAC VLAN tag register"]
pub struct Macvlantr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC VLAN tag register"]
pub mod macvlantr;
#[doc = "Ethernet MAC PMT control and status register"]
pub struct Macpmtcsr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC PMT control and status register"]
pub mod macpmtcsr;
#[doc = "Ethernet MAC debug register"]
pub struct Macdbgr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC debug register"]
pub mod macdbgr;
#[doc = "Ethernet MAC interrupt status register"]
pub struct Macsr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC interrupt status register"]
pub mod macsr;
#[doc = "Ethernet MAC interrupt mask register"]
pub struct Macimr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC interrupt mask register"]
pub mod macimr;
#[doc = "Ethernet MAC address 0 high register"]
pub struct Maca0hr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 0 high register"]
pub mod maca0hr;
#[doc = "Ethernet MAC address 0 low register"]
pub struct Maca0lr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 0 low register"]
pub mod maca0lr;
#[doc = "Ethernet MAC address 1 high register"]
pub struct Maca1hr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 1 high register"]
pub mod maca1hr;
#[doc = "Ethernet MAC address1 low register"]
pub struct Maca1lr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC address1 low register"]
pub mod maca1lr;
#[doc = "Ethernet MAC address 2 high register"]
pub struct Maca2hr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 2 high register"]
pub mod maca2hr;
#[doc = "Ethernet MAC address 2 low register"]
pub struct Maca2lr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 2 low register"]
pub mod maca2lr;
#[doc = "Ethernet MAC address 3 high register"]
pub struct Maca3hr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 3 high register"]
pub mod maca3hr;
#[doc = "Ethernet MAC address 3 low register"]
pub struct Maca3lr {
    register: VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 3 low register"]
pub mod maca3lr;
