use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock { # [ doc = "0x00 - OTG_HS device configuration register" ] pub otg_hs_dcfg : OtgHsDcfg , # [ doc = "0x04 - OTG_HS device control register" ] pub otg_hs_dctl : OtgHsDctl , # [ doc = "0x08 - OTG_HS device status register" ] pub otg_hs_dsts : OtgHsDsts , _reserved0 : [ u8 ; 4usize ] , # [ doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register" ] pub otg_hs_diepmsk : OtgHsDiepmsk , # [ doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register" ] pub otg_hs_doepmsk : OtgHsDoepmsk , # [ doc = "0x18 - OTG_HS device all endpoints interrupt register" ] pub otg_hs_daint : OtgHsDaint , # [ doc = "0x1c - OTG_HS all endpoints interrupt mask register" ] pub otg_hs_daintmsk : OtgHsDaintmsk , _reserved1 : [ u8 ; 8usize ] , # [ doc = "0x28 - OTG_HS device VBUS discharge time register" ] pub otg_hs_dvbusdis : OtgHsDvbusdis , # [ doc = "0x2c - OTG_HS device VBUS pulsing time register" ] pub otg_hs_dvbuspulse : OtgHsDvbuspulse , # [ doc = "0x30 - OTG_HS Device threshold control register" ] pub otg_hs_dthrctl : OtgHsDthrctl , # [ doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register" ] pub otg_hs_diepempmsk : OtgHsDiepempmsk , # [ doc = "0x38 - OTG_HS device each endpoint interrupt register" ] pub otg_hs_deachint : OtgHsDeachint , # [ doc = "0x3c - OTG_HS device each endpoint interrupt register mask" ] pub otg_hs_deachintmsk : OtgHsDeachintmsk , # [ doc = "0x40 - OTG_HS device each in endpoint-1 interrupt register" ] pub otg_hs_diepeachmsk1 : OtgHsDiepeachmsk1 , _reserved2 : [ u8 ; 60usize ] , # [ doc = "0x80 - OTG_HS device each OUT endpoint-1 interrupt register" ] pub otg_hs_doepeachmsk1 : OtgHsDoepeachmsk1 , _reserved3 : [ u8 ; 124usize ] , # [ doc = "0x100 - OTG device endpoint-0 control register" ] pub otg_hs_diepctl0 : OtgHsDiepctl0 , _reserved4 : [ u8 ; 4usize ] , # [ doc = "0x108 - OTG device endpoint-0 interrupt register" ] pub otg_hs_diepint0 : OtgHsDiepint0 , _reserved5 : [ u8 ; 4usize ] , # [ doc = "0x110 - OTG_HS device IN endpoint 0 transfer size register" ] pub otg_hs_dieptsiz0 : OtgHsDieptsiz0 , # [ doc = "0x114 - OTG_HS device endpoint-1 DMA address register" ] pub otg_hs_diepdma1 : OtgHsDiepdma1 , # [ doc = "0x118 - OTG_HS device IN endpoint transmit FIFO status register" ] pub otg_hs_dtxfsts0 : OtgHsDtxfsts0 , _reserved6 : [ u8 ; 4usize ] , # [ doc = "0x120 - OTG device endpoint-1 control register" ] pub otg_hs_diepctl1 : OtgHsDiepctl1 , _reserved7 : [ u8 ; 4usize ] , # [ doc = "0x128 - OTG device endpoint-1 interrupt register" ] pub otg_hs_diepint1 : OtgHsDiepint1 , _reserved8 : [ u8 ; 4usize ] , # [ doc = "0x130 - OTG_HS device endpoint transfer size register" ] pub otg_hs_dieptsiz1 : OtgHsDieptsiz1 , # [ doc = "0x134 - OTG_HS device endpoint-2 DMA address register" ] pub otg_hs_diepdma2 : OtgHsDiepdma2 , # [ doc = "0x138 - OTG_HS device IN endpoint transmit FIFO status register" ] pub otg_hs_dtxfsts1 : OtgHsDtxfsts1 , _reserved9 : [ u8 ; 4usize ] , # [ doc = "0x140 - OTG device endpoint-2 control register" ] pub otg_hs_diepctl2 : OtgHsDiepctl2 , _reserved10 : [ u8 ; 4usize ] , # [ doc = "0x148 - OTG device endpoint-2 interrupt register" ] pub otg_hs_diepint2 : OtgHsDiepint2 , _reserved11 : [ u8 ; 4usize ] , # [ doc = "0x150 - OTG_HS device endpoint transfer size register" ] pub otg_hs_dieptsiz2 : OtgHsDieptsiz2 , # [ doc = "0x154 - OTG_HS device endpoint-3 DMA address register" ] pub otg_hs_diepdma3 : OtgHsDiepdma3 , # [ doc = "0x158 - OTG_HS device IN endpoint transmit FIFO status register" ] pub otg_hs_dtxfsts2 : OtgHsDtxfsts2 , _reserved12 : [ u8 ; 4usize ] , # [ doc = "0x160 - OTG device endpoint-3 control register" ] pub otg_hs_diepctl3 : OtgHsDiepctl3 , _reserved13 : [ u8 ; 4usize ] , # [ doc = "0x168 - OTG device endpoint-3 interrupt register" ] pub otg_hs_diepint3 : OtgHsDiepint3 , _reserved14 : [ u8 ; 4usize ] , # [ doc = "0x170 - OTG_HS device endpoint transfer size register" ] pub otg_hs_dieptsiz3 : OtgHsDieptsiz3 , # [ doc = "0x174 - OTG_HS device endpoint-4 DMA address register" ] pub otg_hs_diepdma4 : OtgHsDiepdma4 , # [ doc = "0x178 - OTG_HS device IN endpoint transmit FIFO status register" ] pub otg_hs_dtxfsts3 : OtgHsDtxfsts3 , _reserved15 : [ u8 ; 4usize ] , # [ doc = "0x180 - OTG device endpoint-4 control register" ] pub otg_hs_diepctl4 : OtgHsDiepctl4 , _reserved16 : [ u8 ; 4usize ] , # [ doc = "0x188 - OTG device endpoint-4 interrupt register" ] pub otg_hs_diepint4 : OtgHsDiepint4 , _reserved17 : [ u8 ; 4usize ] , # [ doc = "0x190 - OTG_HS device endpoint transfer size register" ] pub otg_hs_dieptsiz4 : OtgHsDieptsiz4 , # [ doc = "0x194 - OTG_HS device endpoint-5 DMA address register" ] pub otg_hs_diepdma5 : OtgHsDiepdma5 , # [ doc = "0x198 - OTG_HS device IN endpoint transmit FIFO status register" ] pub otg_hs_dtxfsts4 : OtgHsDtxfsts4 , _reserved18 : [ u8 ; 4usize ] , # [ doc = "0x1a0 - OTG device endpoint-5 control register" ] pub otg_hs_diepctl5 : OtgHsDiepctl5 , _reserved19 : [ u8 ; 4usize ] , # [ doc = "0x1a8 - OTG device endpoint-5 interrupt register" ] pub otg_hs_diepint5 : OtgHsDiepint5 , _reserved20 : [ u8 ; 4usize ] , # [ doc = "0x1b0 - OTG_HS device endpoint transfer size register" ] pub otg_hs_dieptsiz5 : OtgHsDieptsiz5 , _reserved21 : [ u8 ; 4usize ] , # [ doc = "0x1b8 - OTG_HS device IN endpoint transmit FIFO status register" ] pub otg_hs_dtxfsts5 : OtgHsDtxfsts5 , _reserved22 : [ u8 ; 4usize ] , # [ doc = "0x1c0 - OTG device endpoint-6 control register" ] pub otg_hs_diepctl6 : OtgHsDiepctl6 , _reserved23 : [ u8 ; 4usize ] , # [ doc = "0x1c8 - OTG device endpoint-6 interrupt register" ] pub otg_hs_diepint6 : OtgHsDiepint6 , _reserved24 : [ u8 ; 20usize ] , # [ doc = "0x1e0 - OTG device endpoint-7 control register" ] pub otg_hs_diepctl7 : OtgHsDiepctl7 , _reserved25 : [ u8 ; 4usize ] , # [ doc = "0x1e8 - OTG device endpoint-7 interrupt register" ] pub otg_hs_diepint7 : OtgHsDiepint7 , _reserved26 : [ u8 ; 276usize ] , # [ doc = "0x300 - OTG_HS device control OUT endpoint 0 control register" ] pub otg_hs_doepctl0 : OtgHsDoepctl0 , _reserved27 : [ u8 ; 4usize ] , # [ doc = "0x308 - OTG_HS device endpoint-0 interrupt register" ] pub otg_hs_doepint0 : OtgHsDoepint0 , _reserved28 : [ u8 ; 4usize ] , # [ doc = "0x310 - OTG_HS device endpoint-1 transfer size register" ] pub otg_hs_doeptsiz0 : OtgHsDoeptsiz0 , _reserved29 : [ u8 ; 12usize ] , # [ doc = "0x320 - OTG device endpoint-1 control register" ] pub otg_hs_doepctl1 : OtgHsDoepctl1 , _reserved30 : [ u8 ; 4usize ] , # [ doc = "0x328 - OTG_HS device endpoint-1 interrupt register" ] pub otg_hs_doepint1 : OtgHsDoepint1 , _reserved31 : [ u8 ; 4usize ] , # [ doc = "0x330 - OTG_HS device endpoint-2 transfer size register" ] pub otg_hs_doeptsiz1 : OtgHsDoeptsiz1 , _reserved32 : [ u8 ; 12usize ] , # [ doc = "0x340 - OTG device endpoint-2 control register" ] pub otg_hs_doepctl2 : OtgHsDoepctl2 , _reserved33 : [ u8 ; 4usize ] , # [ doc = "0x348 - OTG_HS device endpoint-2 interrupt register" ] pub otg_hs_doepint2 : OtgHsDoepint2 , _reserved34 : [ u8 ; 4usize ] , # [ doc = "0x350 - OTG_HS device endpoint-3 transfer size register" ] pub otg_hs_doeptsiz2 : OtgHsDoeptsiz2 , _reserved35 : [ u8 ; 12usize ] , # [ doc = "0x360 - OTG device endpoint-3 control register" ] pub otg_hs_doepctl3 : OtgHsDoepctl3 , _reserved36 : [ u8 ; 4usize ] , # [ doc = "0x368 - OTG_HS device endpoint-3 interrupt register" ] pub otg_hs_doepint3 : OtgHsDoepint3 , _reserved37 : [ u8 ; 4usize ] , # [ doc = "0x370 - OTG_HS device endpoint-4 transfer size register" ] pub otg_hs_doeptsiz3 : OtgHsDoeptsiz3 , _reserved38 : [ u8 ; 20usize ] , # [ doc = "0x388 - OTG_HS device endpoint-4 interrupt register" ] pub otg_hs_doepint4 : OtgHsDoepint4 , _reserved39 : [ u8 ; 4usize ] , # [ doc = "0x390 - OTG_HS device endpoint-5 transfer size register" ] pub otg_hs_doeptsiz4 : OtgHsDoeptsiz4 , _reserved40 : [ u8 ; 20usize ] , # [ doc = "0x3a8 - OTG_HS device endpoint-5 interrupt register" ] pub otg_hs_doepint5 : OtgHsDoepint5 , _reserved41 : [ u8 ; 28usize ] , # [ doc = "0x3c8 - OTG_HS device endpoint-6 interrupt register" ] pub otg_hs_doepint6 : OtgHsDoepint6 , _reserved42 : [ u8 ; 28usize ] , # [ doc = "0x3e8 - OTG_HS device endpoint-7 interrupt register" ] pub otg_hs_doepint7 : OtgHsDoepint7 }
#[doc = "OTG_HS device configuration register"]
pub struct OtgHsDcfg {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device configuration register"]
pub mod otg_hs_dcfg;
#[doc = "OTG_HS device control register"]
pub struct OtgHsDctl {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device control register"]
pub mod otg_hs_dctl;
#[doc = "OTG_HS device status register"]
pub struct OtgHsDsts {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device status register"]
pub mod otg_hs_dsts;
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub struct OtgHsDiepmsk {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub mod otg_hs_diepmsk;
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub struct OtgHsDoepmsk {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub mod otg_hs_doepmsk;
#[doc = "OTG_HS device all endpoints interrupt register"]
pub struct OtgHsDaint {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device all endpoints interrupt register"]
pub mod otg_hs_daint;
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub struct OtgHsDaintmsk {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub mod otg_hs_daintmsk;
#[doc = "OTG_HS device VBUS discharge time register"]
pub struct OtgHsDvbusdis {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device VBUS discharge time register"]
pub mod otg_hs_dvbusdis;
#[doc = "OTG_HS device VBUS pulsing time register"]
pub struct OtgHsDvbuspulse {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device VBUS pulsing time register"]
pub mod otg_hs_dvbuspulse;
#[doc = "OTG_HS Device threshold control register"]
pub struct OtgHsDthrctl {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS Device threshold control register"]
pub mod otg_hs_dthrctl;
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub struct OtgHsDiepempmsk {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub mod otg_hs_diepempmsk;
#[doc = "OTG_HS device each endpoint interrupt register"]
pub struct OtgHsDeachint {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device each endpoint interrupt register"]
pub mod otg_hs_deachint;
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub struct OtgHsDeachintmsk {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub mod otg_hs_deachintmsk;
#[doc = "OTG_HS device each in endpoint-1 interrupt register"]
pub struct OtgHsDiepeachmsk1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device each in endpoint-1 interrupt register"]
pub mod otg_hs_diepeachmsk1;
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register"]
pub struct OtgHsDoepeachmsk1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register"]
pub mod otg_hs_doepeachmsk1;
#[doc = "OTG device endpoint-0 control register"]
pub struct OtgHsDiepctl0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-0 control register"]
pub mod otg_hs_diepctl0;
#[doc = "OTG device endpoint-1 control register"]
pub struct OtgHsDiepctl1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-1 control register"]
pub mod otg_hs_diepctl1;
#[doc = "OTG device endpoint-2 control register"]
pub struct OtgHsDiepctl2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-2 control register"]
pub mod otg_hs_diepctl2;
#[doc = "OTG device endpoint-3 control register"]
pub struct OtgHsDiepctl3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-3 control register"]
pub mod otg_hs_diepctl3;
#[doc = "OTG device endpoint-4 control register"]
pub struct OtgHsDiepctl4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-4 control register"]
pub mod otg_hs_diepctl4;
#[doc = "OTG device endpoint-5 control register"]
pub struct OtgHsDiepctl5 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-5 control register"]
pub mod otg_hs_diepctl5;
#[doc = "OTG device endpoint-6 control register"]
pub struct OtgHsDiepctl6 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-6 control register"]
pub mod otg_hs_diepctl6;
#[doc = "OTG device endpoint-7 control register"]
pub struct OtgHsDiepctl7 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-7 control register"]
pub mod otg_hs_diepctl7;
#[doc = "OTG device endpoint-0 interrupt register"]
pub struct OtgHsDiepint0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-0 interrupt register"]
pub mod otg_hs_diepint0;
#[doc = "OTG device endpoint-1 interrupt register"]
pub struct OtgHsDiepint1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-1 interrupt register"]
pub mod otg_hs_diepint1;
#[doc = "OTG device endpoint-2 interrupt register"]
pub struct OtgHsDiepint2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-2 interrupt register"]
pub mod otg_hs_diepint2;
#[doc = "OTG device endpoint-3 interrupt register"]
pub struct OtgHsDiepint3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-3 interrupt register"]
pub mod otg_hs_diepint3;
#[doc = "OTG device endpoint-4 interrupt register"]
pub struct OtgHsDiepint4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-4 interrupt register"]
pub mod otg_hs_diepint4;
#[doc = "OTG device endpoint-5 interrupt register"]
pub struct OtgHsDiepint5 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-5 interrupt register"]
pub mod otg_hs_diepint5;
#[doc = "OTG device endpoint-6 interrupt register"]
pub struct OtgHsDiepint6 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-6 interrupt register"]
pub mod otg_hs_diepint6;
#[doc = "OTG device endpoint-7 interrupt register"]
pub struct OtgHsDiepint7 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-7 interrupt register"]
pub mod otg_hs_diepint7;
#[doc = "OTG_HS device IN endpoint 0 transfer size register"]
pub struct OtgHsDieptsiz0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint 0 transfer size register"]
pub mod otg_hs_dieptsiz0;
#[doc = "OTG_HS device endpoint-1 DMA address register"]
pub struct OtgHsDiepdma1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-1 DMA address register"]
pub mod otg_hs_diepdma1;
#[doc = "OTG_HS device endpoint-2 DMA address register"]
pub struct OtgHsDiepdma2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-2 DMA address register"]
pub mod otg_hs_diepdma2;
#[doc = "OTG_HS device endpoint-3 DMA address register"]
pub struct OtgHsDiepdma3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-3 DMA address register"]
pub mod otg_hs_diepdma3;
#[doc = "OTG_HS device endpoint-4 DMA address register"]
pub struct OtgHsDiepdma4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-4 DMA address register"]
pub mod otg_hs_diepdma4;
#[doc = "OTG_HS device endpoint-5 DMA address register"]
pub struct OtgHsDiepdma5 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-5 DMA address register"]
pub mod otg_hs_diepdma5;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub struct OtgHsDtxfsts0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts0;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub struct OtgHsDtxfsts1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts1;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub struct OtgHsDtxfsts2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts2;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub struct OtgHsDtxfsts3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts3;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub struct OtgHsDtxfsts4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts4;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub struct OtgHsDtxfsts5 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts5;
#[doc = "OTG_HS device endpoint transfer size register"]
pub struct OtgHsDieptsiz1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz1;
#[doc = "OTG_HS device endpoint transfer size register"]
pub struct OtgHsDieptsiz2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz2;
#[doc = "OTG_HS device endpoint transfer size register"]
pub struct OtgHsDieptsiz3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz3;
#[doc = "OTG_HS device endpoint transfer size register"]
pub struct OtgHsDieptsiz4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz4;
#[doc = "OTG_HS device endpoint transfer size register"]
pub struct OtgHsDieptsiz5 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz5;
#[doc = "OTG_HS device control OUT endpoint 0 control register"]
pub struct OtgHsDoepctl0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device control OUT endpoint 0 control register"]
pub mod otg_hs_doepctl0;
#[doc = "OTG device endpoint-1 control register"]
pub struct OtgHsDoepctl1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-1 control register"]
pub mod otg_hs_doepctl1;
#[doc = "OTG device endpoint-2 control register"]
pub struct OtgHsDoepctl2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-2 control register"]
pub mod otg_hs_doepctl2;
#[doc = "OTG device endpoint-3 control register"]
pub struct OtgHsDoepctl3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG device endpoint-3 control register"]
pub mod otg_hs_doepctl3;
#[doc = "OTG_HS device endpoint-0 interrupt register"]
pub struct OtgHsDoepint0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-0 interrupt register"]
pub mod otg_hs_doepint0;
#[doc = "OTG_HS device endpoint-1 interrupt register"]
pub struct OtgHsDoepint1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-1 interrupt register"]
pub mod otg_hs_doepint1;
#[doc = "OTG_HS device endpoint-2 interrupt register"]
pub struct OtgHsDoepint2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-2 interrupt register"]
pub mod otg_hs_doepint2;
#[doc = "OTG_HS device endpoint-3 interrupt register"]
pub struct OtgHsDoepint3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-3 interrupt register"]
pub mod otg_hs_doepint3;
#[doc = "OTG_HS device endpoint-4 interrupt register"]
pub struct OtgHsDoepint4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-4 interrupt register"]
pub mod otg_hs_doepint4;
#[doc = "OTG_HS device endpoint-5 interrupt register"]
pub struct OtgHsDoepint5 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-5 interrupt register"]
pub mod otg_hs_doepint5;
#[doc = "OTG_HS device endpoint-6 interrupt register"]
pub struct OtgHsDoepint6 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-6 interrupt register"]
pub mod otg_hs_doepint6;
#[doc = "OTG_HS device endpoint-7 interrupt register"]
pub struct OtgHsDoepint7 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-7 interrupt register"]
pub mod otg_hs_doepint7;
#[doc = "OTG_HS device endpoint-1 transfer size register"]
pub struct OtgHsDoeptsiz0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-1 transfer size register"]
pub mod otg_hs_doeptsiz0;
#[doc = "OTG_HS device endpoint-2 transfer size register"]
pub struct OtgHsDoeptsiz1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-2 transfer size register"]
pub mod otg_hs_doeptsiz1;
#[doc = "OTG_HS device endpoint-3 transfer size register"]
pub struct OtgHsDoeptsiz2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-3 transfer size register"]
pub mod otg_hs_doeptsiz2;
#[doc = "OTG_HS device endpoint-4 transfer size register"]
pub struct OtgHsDoeptsiz3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-4 transfer size register"]
pub mod otg_hs_doeptsiz3;
#[doc = "OTG_HS device endpoint-5 transfer size register"]
pub struct OtgHsDoeptsiz4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-5 transfer size register"]
pub mod otg_hs_doeptsiz4;
