use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS host configuration register"] pub otg_hs_hcfg: OtgHsHcfg,
    #[doc = "0x04 - OTG_HS Host frame interval register"] pub otg_hs_hfir: OtgHsHfir,
    #[doc = "0x08 - OTG_HS host frame number/frame time remaining register"]
    pub otg_hs_hfnum: OtgHsHfnum,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - OTG_HS_Host periodic transmit FIFO/queue status register"]
    pub otg_hs_hptxsts: OtgHsHptxsts,
    #[doc = "0x14 - OTG_HS Host all channels interrupt register"] pub otg_hs_haint: OtgHsHaint,
    #[doc = "0x18 - OTG_HS host all channels interrupt mask register"]
    pub otg_hs_haintmsk: OtgHsHaintmsk,
    _reserved1: [u8; 36usize],
    #[doc = "0x40 - OTG_HS host port control and status register"] pub otg_hs_hprt: OtgHsHprt,
    _reserved2: [u8; 188usize],
    #[doc = "0x100 - OTG_HS host channel-0 characteristics register"]
    pub otg_hs_hcchar0: OtgHsHcchar0,
    #[doc = "0x104 - OTG_HS host channel-0 split control register"]
    pub otg_hs_hcsplt0: OtgHsHcsplt0,
    #[doc = "0x108 - OTG_HS host channel-11 interrupt register"] pub otg_hs_hcint0: OtgHsHcint0,
    #[doc = "0x10c - OTG_HS host channel-11 interrupt mask register"]
    pub otg_hs_hcintmsk0: OtgHsHcintmsk0,
    #[doc = "0x110 - OTG_HS host channel-11 transfer size register"]
    pub otg_hs_hctsiz0: OtgHsHctsiz0,
    #[doc = "0x114 - OTG_HS host channel-0 DMA address register"] pub otg_hs_hcdma0: OtgHsHcdma0,
    _reserved3: [u8; 8usize],
    #[doc = "0x120 - OTG_HS host channel-1 characteristics register"]
    pub otg_hs_hcchar1: OtgHsHcchar1,
    #[doc = "0x124 - OTG_HS host channel-1 split control register"]
    pub otg_hs_hcsplt1: OtgHsHcsplt1,
    #[doc = "0x128 - OTG_HS host channel-1 interrupt register"] pub otg_hs_hcint1: OtgHsHcint1,
    #[doc = "0x12c - OTG_HS host channel-1 interrupt mask register"]
    pub otg_hs_hcintmsk1: OtgHsHcintmsk1,
    #[doc = "0x130 - OTG_HS host channel-1 transfer size register"]
    pub otg_hs_hctsiz1: OtgHsHctsiz1,
    #[doc = "0x134 - OTG_HS host channel-1 DMA address register"] pub otg_hs_hcdma1: OtgHsHcdma1,
    _reserved4: [u8; 8usize],
    #[doc = "0x140 - OTG_HS host channel-2 characteristics register"]
    pub otg_hs_hcchar2: OtgHsHcchar2,
    #[doc = "0x144 - OTG_HS host channel-2 split control register"]
    pub otg_hs_hcsplt2: OtgHsHcsplt2,
    #[doc = "0x148 - OTG_HS host channel-2 interrupt register"] pub otg_hs_hcint2: OtgHsHcint2,
    #[doc = "0x14c - OTG_HS host channel-2 interrupt mask register"]
    pub otg_hs_hcintmsk2: OtgHsHcintmsk2,
    #[doc = "0x150 - OTG_HS host channel-2 transfer size register"]
    pub otg_hs_hctsiz2: OtgHsHctsiz2,
    #[doc = "0x154 - OTG_HS host channel-2 DMA address register"] pub otg_hs_hcdma2: OtgHsHcdma2,
    _reserved5: [u8; 8usize],
    #[doc = "0x160 - OTG_HS host channel-3 characteristics register"]
    pub otg_hs_hcchar3: OtgHsHcchar3,
    #[doc = "0x164 - OTG_HS host channel-3 split control register"]
    pub otg_hs_hcsplt3: OtgHsHcsplt3,
    #[doc = "0x168 - OTG_HS host channel-3 interrupt register"] pub otg_hs_hcint3: OtgHsHcint3,
    #[doc = "0x16c - OTG_HS host channel-3 interrupt mask register"]
    pub otg_hs_hcintmsk3: OtgHsHcintmsk3,
    #[doc = "0x170 - OTG_HS host channel-3 transfer size register"]
    pub otg_hs_hctsiz3: OtgHsHctsiz3,
    #[doc = "0x174 - OTG_HS host channel-3 DMA address register"] pub otg_hs_hcdma3: OtgHsHcdma3,
    _reserved6: [u8; 8usize],
    #[doc = "0x180 - OTG_HS host channel-4 characteristics register"]
    pub otg_hs_hcchar4: OtgHsHcchar4,
    #[doc = "0x184 - OTG_HS host channel-4 split control register"]
    pub otg_hs_hcsplt4: OtgHsHcsplt4,
    #[doc = "0x188 - OTG_HS host channel-4 interrupt register"] pub otg_hs_hcint4: OtgHsHcint4,
    #[doc = "0x18c - OTG_HS host channel-4 interrupt mask register"]
    pub otg_hs_hcintmsk4: OtgHsHcintmsk4,
    #[doc = "0x190 - OTG_HS host channel-4 transfer size register"]
    pub otg_hs_hctsiz4: OtgHsHctsiz4,
    #[doc = "0x194 - OTG_HS host channel-4 DMA address register"] pub otg_hs_hcdma4: OtgHsHcdma4,
    _reserved7: [u8; 8usize],
    #[doc = "0x1a0 - OTG_HS host channel-5 characteristics register"]
    pub otg_hs_hcchar5: OtgHsHcchar5,
    #[doc = "0x1a4 - OTG_HS host channel-5 split control register"]
    pub otg_hs_hcsplt5: OtgHsHcsplt5,
    #[doc = "0x1a8 - OTG_HS host channel-5 interrupt register"] pub otg_hs_hcint5: OtgHsHcint5,
    #[doc = "0x1ac - OTG_HS host channel-5 interrupt mask register"]
    pub otg_hs_hcintmsk5: OtgHsHcintmsk5,
    #[doc = "0x1b0 - OTG_HS host channel-5 transfer size register"]
    pub otg_hs_hctsiz5: OtgHsHctsiz5,
    #[doc = "0x1b4 - OTG_HS host channel-5 DMA address register"] pub otg_hs_hcdma5: OtgHsHcdma5,
    _reserved8: [u8; 8usize],
    #[doc = "0x1c0 - OTG_HS host channel-6 characteristics register"]
    pub otg_hs_hcchar6: OtgHsHcchar6,
    #[doc = "0x1c4 - OTG_HS host channel-6 split control register"]
    pub otg_hs_hcsplt6: OtgHsHcsplt6,
    #[doc = "0x1c8 - OTG_HS host channel-6 interrupt register"] pub otg_hs_hcint6: OtgHsHcint6,
    #[doc = "0x1cc - OTG_HS host channel-6 interrupt mask register"]
    pub otg_hs_hcintmsk6: OtgHsHcintmsk6,
    #[doc = "0x1d0 - OTG_HS host channel-6 transfer size register"]
    pub otg_hs_hctsiz6: OtgHsHctsiz6,
    #[doc = "0x1d4 - OTG_HS host channel-6 DMA address register"] pub otg_hs_hcdma6: OtgHsHcdma6,
    _reserved9: [u8; 8usize],
    #[doc = "0x1e0 - OTG_HS host channel-7 characteristics register"]
    pub otg_hs_hcchar7: OtgHsHcchar7,
    #[doc = "0x1e4 - OTG_HS host channel-7 split control register"]
    pub otg_hs_hcsplt7: OtgHsHcsplt7,
    #[doc = "0x1e8 - OTG_HS host channel-7 interrupt register"] pub otg_hs_hcint7: OtgHsHcint7,
    #[doc = "0x1ec - OTG_HS host channel-7 interrupt mask register"]
    pub otg_hs_hcintmsk7: OtgHsHcintmsk7,
    #[doc = "0x1f0 - OTG_HS host channel-7 transfer size register"]
    pub otg_hs_hctsiz7: OtgHsHctsiz7,
    #[doc = "0x1f4 - OTG_HS host channel-7 DMA address register"] pub otg_hs_hcdma7: OtgHsHcdma7,
    _reserved10: [u8; 8usize],
    #[doc = "0x200 - OTG_HS host channel-8 characteristics register"]
    pub otg_hs_hcchar8: OtgHsHcchar8,
    #[doc = "0x204 - OTG_HS host channel-8 split control register"]
    pub otg_hs_hcsplt8: OtgHsHcsplt8,
    #[doc = "0x208 - OTG_HS host channel-8 interrupt register"] pub otg_hs_hcint8: OtgHsHcint8,
    #[doc = "0x20c - OTG_HS host channel-8 interrupt mask register"]
    pub otg_hs_hcintmsk8: OtgHsHcintmsk8,
    #[doc = "0x210 - OTG_HS host channel-8 transfer size register"]
    pub otg_hs_hctsiz8: OtgHsHctsiz8,
    #[doc = "0x214 - OTG_HS host channel-8 DMA address register"] pub otg_hs_hcdma8: OtgHsHcdma8,
    _reserved11: [u8; 8usize],
    #[doc = "0x220 - OTG_HS host channel-9 characteristics register"]
    pub otg_hs_hcchar9: OtgHsHcchar9,
    #[doc = "0x224 - OTG_HS host channel-9 split control register"]
    pub otg_hs_hcsplt9: OtgHsHcsplt9,
    #[doc = "0x228 - OTG_HS host channel-9 interrupt register"] pub otg_hs_hcint9: OtgHsHcint9,
    #[doc = "0x22c - OTG_HS host channel-9 interrupt mask register"]
    pub otg_hs_hcintmsk9: OtgHsHcintmsk9,
    #[doc = "0x230 - OTG_HS host channel-9 transfer size register"]
    pub otg_hs_hctsiz9: OtgHsHctsiz9,
    #[doc = "0x234 - OTG_HS host channel-9 DMA address register"] pub otg_hs_hcdma9: OtgHsHcdma9,
    _reserved12: [u8; 8usize],
    #[doc = "0x240 - OTG_HS host channel-10 characteristics register"]
    pub otg_hs_hcchar10: OtgHsHcchar10,
    #[doc = "0x244 - OTG_HS host channel-10 split control register"]
    pub otg_hs_hcsplt10: OtgHsHcsplt10,
    #[doc = "0x248 - OTG_HS host channel-10 interrupt register"] pub otg_hs_hcint10: OtgHsHcint10,
    #[doc = "0x24c - OTG_HS host channel-10 interrupt mask register"]
    pub otg_hs_hcintmsk10: OtgHsHcintmsk10,
    #[doc = "0x250 - OTG_HS host channel-10 transfer size register"]
    pub otg_hs_hctsiz10: OtgHsHctsiz10,
    #[doc = "0x254 - OTG_HS host channel-10 DMA address register"] pub otg_hs_hcdma10: OtgHsHcdma10,
    _reserved13: [u8; 8usize],
    #[doc = "0x260 - OTG_HS host channel-11 characteristics register"]
    pub otg_hs_hcchar11: OtgHsHcchar11,
    #[doc = "0x264 - OTG_HS host channel-11 split control register"]
    pub otg_hs_hcsplt11: OtgHsHcsplt11,
    #[doc = "0x268 - OTG_HS host channel-11 interrupt register"] pub otg_hs_hcint11: OtgHsHcint11,
    #[doc = "0x26c - OTG_HS host channel-11 interrupt mask register"]
    pub otg_hs_hcintmsk11: OtgHsHcintmsk11,
    #[doc = "0x270 - OTG_HS host channel-11 transfer size register"]
    pub otg_hs_hctsiz11: OtgHsHctsiz11,
    #[doc = "0x274 - OTG_HS host channel-11 DMA address register"] pub otg_hs_hcdma11: OtgHsHcdma11,
}
#[doc = "OTG_HS host configuration register"]
pub struct OtgHsHcfg {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host configuration register"]
pub mod otg_hs_hcfg;
#[doc = "OTG_HS Host frame interval register"]
pub struct OtgHsHfir {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS Host frame interval register"]
pub mod otg_hs_hfir;
#[doc = "OTG_HS host frame number/frame time remaining register"]
pub struct OtgHsHfnum {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host frame number/frame time remaining register"]
pub mod otg_hs_hfnum;
#[doc = "OTG_HS_Host periodic transmit FIFO/queue status register"]
pub struct OtgHsHptxsts {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS_Host periodic transmit FIFO/queue status register"]
pub mod otg_hs_hptxsts;
#[doc = "OTG_HS Host all channels interrupt register"]
pub struct OtgHsHaint {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS Host all channels interrupt register"]
pub mod otg_hs_haint;
#[doc = "OTG_HS host all channels interrupt mask register"]
pub struct OtgHsHaintmsk {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host all channels interrupt mask register"]
pub mod otg_hs_haintmsk;
#[doc = "OTG_HS host port control and status register"]
pub struct OtgHsHprt {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host port control and status register"]
pub mod otg_hs_hprt;
#[doc = "OTG_HS host channel-0 characteristics register"]
pub struct OtgHsHcchar0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-0 characteristics register"]
pub mod otg_hs_hcchar0;
#[doc = "OTG_HS host channel-1 characteristics register"]
pub struct OtgHsHcchar1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-1 characteristics register"]
pub mod otg_hs_hcchar1;
#[doc = "OTG_HS host channel-2 characteristics register"]
pub struct OtgHsHcchar2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-2 characteristics register"]
pub mod otg_hs_hcchar2;
#[doc = "OTG_HS host channel-3 characteristics register"]
pub struct OtgHsHcchar3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-3 characteristics register"]
pub mod otg_hs_hcchar3;
#[doc = "OTG_HS host channel-4 characteristics register"]
pub struct OtgHsHcchar4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-4 characteristics register"]
pub mod otg_hs_hcchar4;
#[doc = "OTG_HS host channel-5 characteristics register"]
pub struct OtgHsHcchar5 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-5 characteristics register"]
pub mod otg_hs_hcchar5;
#[doc = "OTG_HS host channel-6 characteristics register"]
pub struct OtgHsHcchar6 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-6 characteristics register"]
pub mod otg_hs_hcchar6;
#[doc = "OTG_HS host channel-7 characteristics register"]
pub struct OtgHsHcchar7 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-7 characteristics register"]
pub mod otg_hs_hcchar7;
#[doc = "OTG_HS host channel-8 characteristics register"]
pub struct OtgHsHcchar8 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-8 characteristics register"]
pub mod otg_hs_hcchar8;
#[doc = "OTG_HS host channel-9 characteristics register"]
pub struct OtgHsHcchar9 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-9 characteristics register"]
pub mod otg_hs_hcchar9;
#[doc = "OTG_HS host channel-10 characteristics register"]
pub struct OtgHsHcchar10 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-10 characteristics register"]
pub mod otg_hs_hcchar10;
#[doc = "OTG_HS host channel-11 characteristics register"]
pub struct OtgHsHcchar11 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-11 characteristics register"]
pub mod otg_hs_hcchar11;
#[doc = "OTG_HS host channel-0 split control register"]
pub struct OtgHsHcsplt0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-0 split control register"]
pub mod otg_hs_hcsplt0;
#[doc = "OTG_HS host channel-1 split control register"]
pub struct OtgHsHcsplt1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-1 split control register"]
pub mod otg_hs_hcsplt1;
#[doc = "OTG_HS host channel-2 split control register"]
pub struct OtgHsHcsplt2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-2 split control register"]
pub mod otg_hs_hcsplt2;
#[doc = "OTG_HS host channel-3 split control register"]
pub struct OtgHsHcsplt3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-3 split control register"]
pub mod otg_hs_hcsplt3;
#[doc = "OTG_HS host channel-4 split control register"]
pub struct OtgHsHcsplt4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-4 split control register"]
pub mod otg_hs_hcsplt4;
#[doc = "OTG_HS host channel-5 split control register"]
pub struct OtgHsHcsplt5 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-5 split control register"]
pub mod otg_hs_hcsplt5;
#[doc = "OTG_HS host channel-6 split control register"]
pub struct OtgHsHcsplt6 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-6 split control register"]
pub mod otg_hs_hcsplt6;
#[doc = "OTG_HS host channel-7 split control register"]
pub struct OtgHsHcsplt7 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-7 split control register"]
pub mod otg_hs_hcsplt7;
#[doc = "OTG_HS host channel-8 split control register"]
pub struct OtgHsHcsplt8 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-8 split control register"]
pub mod otg_hs_hcsplt8;
#[doc = "OTG_HS host channel-9 split control register"]
pub struct OtgHsHcsplt9 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-9 split control register"]
pub mod otg_hs_hcsplt9;
#[doc = "OTG_HS host channel-10 split control register"]
pub struct OtgHsHcsplt10 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-10 split control register"]
pub mod otg_hs_hcsplt10;
#[doc = "OTG_HS host channel-11 split control register"]
pub struct OtgHsHcsplt11 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-11 split control register"]
pub mod otg_hs_hcsplt11;
#[doc = "OTG_HS host channel-11 interrupt register"]
pub struct OtgHsHcint0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-11 interrupt register"]
pub mod otg_hs_hcint0;
#[doc = "OTG_HS host channel-1 interrupt register"]
pub struct OtgHsHcint1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-1 interrupt register"]
pub mod otg_hs_hcint1;
#[doc = "OTG_HS host channel-2 interrupt register"]
pub struct OtgHsHcint2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-2 interrupt register"]
pub mod otg_hs_hcint2;
#[doc = "OTG_HS host channel-3 interrupt register"]
pub struct OtgHsHcint3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-3 interrupt register"]
pub mod otg_hs_hcint3;
#[doc = "OTG_HS host channel-4 interrupt register"]
pub struct OtgHsHcint4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-4 interrupt register"]
pub mod otg_hs_hcint4;
#[doc = "OTG_HS host channel-5 interrupt register"]
pub struct OtgHsHcint5 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-5 interrupt register"]
pub mod otg_hs_hcint5;
#[doc = "OTG_HS host channel-6 interrupt register"]
pub struct OtgHsHcint6 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-6 interrupt register"]
pub mod otg_hs_hcint6;
#[doc = "OTG_HS host channel-7 interrupt register"]
pub struct OtgHsHcint7 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-7 interrupt register"]
pub mod otg_hs_hcint7;
#[doc = "OTG_HS host channel-8 interrupt register"]
pub struct OtgHsHcint8 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-8 interrupt register"]
pub mod otg_hs_hcint8;
#[doc = "OTG_HS host channel-9 interrupt register"]
pub struct OtgHsHcint9 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-9 interrupt register"]
pub mod otg_hs_hcint9;
#[doc = "OTG_HS host channel-10 interrupt register"]
pub struct OtgHsHcint10 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-10 interrupt register"]
pub mod otg_hs_hcint10;
#[doc = "OTG_HS host channel-11 interrupt register"]
pub struct OtgHsHcint11 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-11 interrupt register"]
pub mod otg_hs_hcint11;
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub struct OtgHsHcintmsk0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub mod otg_hs_hcintmsk0;
#[doc = "OTG_HS host channel-1 interrupt mask register"]
pub struct OtgHsHcintmsk1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-1 interrupt mask register"]
pub mod otg_hs_hcintmsk1;
#[doc = "OTG_HS host channel-2 interrupt mask register"]
pub struct OtgHsHcintmsk2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-2 interrupt mask register"]
pub mod otg_hs_hcintmsk2;
#[doc = "OTG_HS host channel-3 interrupt mask register"]
pub struct OtgHsHcintmsk3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-3 interrupt mask register"]
pub mod otg_hs_hcintmsk3;
#[doc = "OTG_HS host channel-4 interrupt mask register"]
pub struct OtgHsHcintmsk4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-4 interrupt mask register"]
pub mod otg_hs_hcintmsk4;
#[doc = "OTG_HS host channel-5 interrupt mask register"]
pub struct OtgHsHcintmsk5 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-5 interrupt mask register"]
pub mod otg_hs_hcintmsk5;
#[doc = "OTG_HS host channel-6 interrupt mask register"]
pub struct OtgHsHcintmsk6 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-6 interrupt mask register"]
pub mod otg_hs_hcintmsk6;
#[doc = "OTG_HS host channel-7 interrupt mask register"]
pub struct OtgHsHcintmsk7 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-7 interrupt mask register"]
pub mod otg_hs_hcintmsk7;
#[doc = "OTG_HS host channel-8 interrupt mask register"]
pub struct OtgHsHcintmsk8 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-8 interrupt mask register"]
pub mod otg_hs_hcintmsk8;
#[doc = "OTG_HS host channel-9 interrupt mask register"]
pub struct OtgHsHcintmsk9 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-9 interrupt mask register"]
pub mod otg_hs_hcintmsk9;
#[doc = "OTG_HS host channel-10 interrupt mask register"]
pub struct OtgHsHcintmsk10 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-10 interrupt mask register"]
pub mod otg_hs_hcintmsk10;
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub struct OtgHsHcintmsk11 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub mod otg_hs_hcintmsk11;
#[doc = "OTG_HS host channel-11 transfer size register"]
pub struct OtgHsHctsiz0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-11 transfer size register"]
pub mod otg_hs_hctsiz0;
#[doc = "OTG_HS host channel-1 transfer size register"]
pub struct OtgHsHctsiz1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-1 transfer size register"]
pub mod otg_hs_hctsiz1;
#[doc = "OTG_HS host channel-2 transfer size register"]
pub struct OtgHsHctsiz2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-2 transfer size register"]
pub mod otg_hs_hctsiz2;
#[doc = "OTG_HS host channel-3 transfer size register"]
pub struct OtgHsHctsiz3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-3 transfer size register"]
pub mod otg_hs_hctsiz3;
#[doc = "OTG_HS host channel-4 transfer size register"]
pub struct OtgHsHctsiz4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-4 transfer size register"]
pub mod otg_hs_hctsiz4;
#[doc = "OTG_HS host channel-5 transfer size register"]
pub struct OtgHsHctsiz5 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-5 transfer size register"]
pub mod otg_hs_hctsiz5;
#[doc = "OTG_HS host channel-6 transfer size register"]
pub struct OtgHsHctsiz6 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-6 transfer size register"]
pub mod otg_hs_hctsiz6;
#[doc = "OTG_HS host channel-7 transfer size register"]
pub struct OtgHsHctsiz7 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-7 transfer size register"]
pub mod otg_hs_hctsiz7;
#[doc = "OTG_HS host channel-8 transfer size register"]
pub struct OtgHsHctsiz8 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-8 transfer size register"]
pub mod otg_hs_hctsiz8;
#[doc = "OTG_HS host channel-9 transfer size register"]
pub struct OtgHsHctsiz9 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-9 transfer size register"]
pub mod otg_hs_hctsiz9;
#[doc = "OTG_HS host channel-10 transfer size register"]
pub struct OtgHsHctsiz10 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-10 transfer size register"]
pub mod otg_hs_hctsiz10;
#[doc = "OTG_HS host channel-11 transfer size register"]
pub struct OtgHsHctsiz11 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-11 transfer size register"]
pub mod otg_hs_hctsiz11;
#[doc = "OTG_HS host channel-0 DMA address register"]
pub struct OtgHsHcdma0 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-0 DMA address register"]
pub mod otg_hs_hcdma0;
#[doc = "OTG_HS host channel-1 DMA address register"]
pub struct OtgHsHcdma1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-1 DMA address register"]
pub mod otg_hs_hcdma1;
#[doc = "OTG_HS host channel-2 DMA address register"]
pub struct OtgHsHcdma2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-2 DMA address register"]
pub mod otg_hs_hcdma2;
#[doc = "OTG_HS host channel-3 DMA address register"]
pub struct OtgHsHcdma3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-3 DMA address register"]
pub mod otg_hs_hcdma3;
#[doc = "OTG_HS host channel-4 DMA address register"]
pub struct OtgHsHcdma4 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-4 DMA address register"]
pub mod otg_hs_hcdma4;
#[doc = "OTG_HS host channel-5 DMA address register"]
pub struct OtgHsHcdma5 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-5 DMA address register"]
pub mod otg_hs_hcdma5;
#[doc = "OTG_HS host channel-6 DMA address register"]
pub struct OtgHsHcdma6 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-6 DMA address register"]
pub mod otg_hs_hcdma6;
#[doc = "OTG_HS host channel-7 DMA address register"]
pub struct OtgHsHcdma7 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-7 DMA address register"]
pub mod otg_hs_hcdma7;
#[doc = "OTG_HS host channel-8 DMA address register"]
pub struct OtgHsHcdma8 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-8 DMA address register"]
pub mod otg_hs_hcdma8;
#[doc = "OTG_HS host channel-9 DMA address register"]
pub struct OtgHsHcdma9 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-9 DMA address register"]
pub mod otg_hs_hcdma9;
#[doc = "OTG_HS host channel-10 DMA address register"]
pub struct OtgHsHcdma10 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-10 DMA address register"]
pub mod otg_hs_hcdma10;
#[doc = "OTG_HS host channel-11 DMA address register"]
pub struct OtgHsHcdma11 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_HS host channel-11 DMA address register"]
pub mod otg_hs_hcdma11;
