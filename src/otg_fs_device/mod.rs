use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS device configuration register (OTG_FS_DCFG)"] pub fs_dcfg: FsDcfg,
    #[doc = "0x04 - OTG_FS device control register (OTG_FS_DCTL)"] pub fs_dctl: FsDctl,
    #[doc = "0x08 - OTG_FS device status register (OTG_FS_DSTS)"] pub fs_dsts: FsDsts,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
    pub fs_diepmsk: FsDiepmsk,
    #[doc = "0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
    pub fs_doepmsk: FsDoepmsk,
    #[doc = "0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
    pub fs_daint: FsDaint,
    #[doc = "0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
    pub fs_daintmsk: FsDaintmsk,
    _reserved1: [u8; 8usize],
    #[doc = "0x28 - OTG_FS device VBUS discharge time register"] pub dvbusdis: Dvbusdis,
    #[doc = "0x2c - OTG_FS device VBUS pulsing time register"] pub dvbuspulse: Dvbuspulse,
    _reserved2: [u8; 4usize],
    #[doc = "0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register"]
    pub diepempmsk: Diepempmsk,
    _reserved3: [u8; 200usize],
    #[doc = "0x100 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
    pub fs_diepctl0: FsDiepctl0,
    _reserved4: [u8; 4usize],
    #[doc = "0x108 - device endpoint-x interrupt register"] pub diepint0: Diepint0,
    _reserved5: [u8; 4usize],
    #[doc = "0x110 - device endpoint-0 transfer size register"] pub dieptsiz0: Dieptsiz0,
    _reserved6: [u8; 4usize],
    #[doc = "0x118 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts0: Dtxfsts0,
    _reserved7: [u8; 4usize],
    #[doc = "0x120 - OTG device endpoint-1 control register"] pub diepctl1: Diepctl1,
    _reserved8: [u8; 4usize],
    #[doc = "0x128 - device endpoint-1 interrupt register"] pub diepint1: Diepint1,
    _reserved9: [u8; 4usize],
    #[doc = "0x130 - device endpoint-1 transfer size register"] pub dieptsiz1: Dieptsiz1,
    _reserved10: [u8; 4usize],
    #[doc = "0x138 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts1: Dtxfsts1,
    _reserved11: [u8; 4usize],
    #[doc = "0x140 - OTG device endpoint-2 control register"] pub diepctl2: Diepctl2,
    _reserved12: [u8; 4usize],
    #[doc = "0x148 - device endpoint-2 interrupt register"] pub diepint2: Diepint2,
    _reserved13: [u8; 4usize],
    #[doc = "0x150 - device endpoint-2 transfer size register"] pub dieptsiz2: Dieptsiz2,
    _reserved14: [u8; 4usize],
    #[doc = "0x158 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts2: Dtxfsts2,
    _reserved15: [u8; 4usize],
    #[doc = "0x160 - OTG device endpoint-3 control register"] pub diepctl3: Diepctl3,
    _reserved16: [u8; 4usize],
    #[doc = "0x168 - device endpoint-3 interrupt register"] pub diepint3: Diepint3,
    _reserved17: [u8; 4usize],
    #[doc = "0x170 - device endpoint-3 transfer size register"] pub dieptsiz3: Dieptsiz3,
    _reserved18: [u8; 4usize],
    #[doc = "0x178 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts3: Dtxfsts3,
    _reserved19: [u8; 388usize],
    #[doc = "0x300 - device endpoint-0 control register"] pub doepctl0: Doepctl0,
    _reserved20: [u8; 4usize],
    #[doc = "0x308 - device endpoint-0 interrupt register"] pub doepint0: Doepint0,
    _reserved21: [u8; 4usize],
    #[doc = "0x310 - device OUT endpoint-0 transfer size register"] pub doeptsiz0: Doeptsiz0,
    _reserved22: [u8; 12usize],
    #[doc = "0x320 - device endpoint-1 control register"] pub doepctl1: Doepctl1,
    _reserved23: [u8; 4usize],
    #[doc = "0x328 - device endpoint-1 interrupt register"] pub doepint1: Doepint1,
    _reserved24: [u8; 4usize],
    #[doc = "0x330 - device OUT endpoint-1 transfer size register"] pub doeptsiz1: Doeptsiz1,
    _reserved25: [u8; 12usize],
    #[doc = "0x340 - device endpoint-2 control register"] pub doepctl2: Doepctl2,
    _reserved26: [u8; 4usize],
    #[doc = "0x348 - device endpoint-2 interrupt register"] pub doepint2: Doepint2,
    _reserved27: [u8; 4usize],
    #[doc = "0x350 - device OUT endpoint-2 transfer size register"] pub doeptsiz2: Doeptsiz2,
    _reserved28: [u8; 12usize],
    #[doc = "0x360 - device endpoint-3 control register"] pub doepctl3: Doepctl3,
    _reserved29: [u8; 4usize],
    #[doc = "0x368 - device endpoint-3 interrupt register"] pub doepint3: Doepint3,
    _reserved30: [u8; 4usize],
    #[doc = "0x370 - device OUT endpoint-3 transfer size register"] pub doeptsiz3: Doeptsiz3,
}
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)"]
pub struct FsDcfg {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)"]
pub mod fs_dcfg;
#[doc = "OTG_FS device control register (OTG_FS_DCTL)"]
pub struct FsDctl {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device control register (OTG_FS_DCTL)"]
pub mod fs_dctl;
#[doc = "OTG_FS device status register (OTG_FS_DSTS)"]
pub struct FsDsts {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device status register (OTG_FS_DSTS)"]
pub mod fs_dsts;
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
pub struct FsDiepmsk {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
pub mod fs_diepmsk;
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
pub struct FsDoepmsk {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
pub mod fs_doepmsk;
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
pub struct FsDaint {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
pub mod fs_daint;
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
pub struct FsDaintmsk {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
pub mod fs_daintmsk;
#[doc = "OTG_FS device VBUS discharge time register"]
pub struct Dvbusdis {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "OTG_FS device VBUS pulsing time register"]
pub struct Dvbuspulse {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register"]
pub struct Diepempmsk {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
pub struct FsDiepctl0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
pub mod fs_diepctl0;
#[doc = "OTG device endpoint-1 control register"]
pub struct Diepctl1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-1 control register"]
pub mod diepctl1;
#[doc = "OTG device endpoint-2 control register"]
pub struct Diepctl2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-2 control register"]
pub mod diepctl2;
#[doc = "OTG device endpoint-3 control register"]
pub struct Diepctl3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-3 control register"]
pub mod diepctl3;
#[doc = "device endpoint-0 control register"]
pub struct Doepctl0 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-0 control register"]
pub mod doepctl0;
#[doc = "device endpoint-1 control register"]
pub struct Doepctl1 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-1 control register"]
pub mod doepctl1;
#[doc = "device endpoint-2 control register"]
pub struct Doepctl2 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-2 control register"]
pub mod doepctl2;
#[doc = "device endpoint-3 control register"]
pub struct Doepctl3 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-3 control register"]
pub mod doepctl3;
#[doc = "device endpoint-x interrupt register"]
pub struct Diepint0 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-x interrupt register"]
pub mod diepint0;
#[doc = "device endpoint-1 interrupt register"]
pub struct Diepint1 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-1 interrupt register"]
pub mod diepint1;
#[doc = "device endpoint-2 interrupt register"]
pub struct Diepint2 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-2 interrupt register"]
pub mod diepint2;
#[doc = "device endpoint-3 interrupt register"]
pub struct Diepint3 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-3 interrupt register"]
pub mod diepint3;
#[doc = "device endpoint-0 interrupt register"]
pub struct Doepint0 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-0 interrupt register"]
pub mod doepint0;
#[doc = "device endpoint-1 interrupt register"]
pub struct Doepint1 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-1 interrupt register"]
pub mod doepint1;
#[doc = "device endpoint-2 interrupt register"]
pub struct Doepint2 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-2 interrupt register"]
pub mod doepint2;
#[doc = "device endpoint-3 interrupt register"]
pub struct Doepint3 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-3 interrupt register"]
pub mod doepint3;
#[doc = "device endpoint-0 transfer size register"]
pub struct Dieptsiz0 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-0 transfer size register"]
pub mod dieptsiz0;
#[doc = "device OUT endpoint-0 transfer size register"]
pub struct Doeptsiz0 {
    register: VolatileCell<u32>,
}
#[doc = "device OUT endpoint-0 transfer size register"]
pub mod doeptsiz0;
#[doc = "device endpoint-1 transfer size register"]
pub struct Dieptsiz1 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-1 transfer size register"]
pub mod dieptsiz1;
#[doc = "device endpoint-2 transfer size register"]
pub struct Dieptsiz2 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-2 transfer size register"]
pub mod dieptsiz2;
#[doc = "device endpoint-3 transfer size register"]
pub struct Dieptsiz3 {
    register: VolatileCell<u32>,
}
#[doc = "device endpoint-3 transfer size register"]
pub mod dieptsiz3;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct Dtxfsts0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts0;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct Dtxfsts1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts1;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct Dtxfsts2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts2;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct Dtxfsts3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts3;
#[doc = "device OUT endpoint-1 transfer size register"]
pub struct Doeptsiz1 {
    register: VolatileCell<u32>,
}
#[doc = "device OUT endpoint-1 transfer size register"]
pub mod doeptsiz1;
#[doc = "device OUT endpoint-2 transfer size register"]
pub struct Doeptsiz2 {
    register: VolatileCell<u32>,
}
#[doc = "device OUT endpoint-2 transfer size register"]
pub mod doeptsiz2;
#[doc = "device OUT endpoint-3 transfer size register"]
pub struct Doeptsiz3 {
    register: VolatileCell<u32>,
}
#[doc = "device OUT endpoint-3 transfer size register"]
pub mod doeptsiz3;
