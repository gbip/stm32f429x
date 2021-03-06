use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)"] pub fs_gotgctl: FsGotgctl,
    #[doc = "0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)"] pub fs_gotgint: FsGotgint,
    #[doc = "0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"] pub fs_gahbcfg: FsGahbcfg,
    #[doc = "0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)"] pub fs_gusbcfg: FsGusbcfg,
    #[doc = "0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)"] pub fs_grstctl: FsGrstctl,
    #[doc = "0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)"] pub fs_gintsts: FsGintsts,
    #[doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"] pub fs_gintmsk: FsGintmsk,
    #[doc = "0x1c - OTG_FS Receive status debug read(Device mode)"]
    pub fs_grxstsr_device: FsGrxstsrDevice,
    _reserved0: [u8; 4usize],
    #[doc = "0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"] pub fs_grxfsiz: FsGrxfsiz,
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)"]
    pub fs_gnptxfsiz_device: FsGnptxfsizDevice,
    #[doc = "0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
    pub fs_gnptxsts: FsGnptxsts,
    _reserved1: [u8; 8usize],
    #[doc = "0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)"]
    pub fs_gccfg: FsGccfg,
    #[doc = "0x3c - core ID register"] pub fs_cid: FsCid,
    _reserved2: [u8; 192usize],
    #[doc = "0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
    pub fs_hptxfsiz: FsHptxfsiz,
    #[doc = "0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
    pub fs_dieptxf1: FsDieptxf1,
    #[doc = "0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
    pub fs_dieptxf2: FsDieptxf2,
    #[doc = "0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
    pub fs_dieptxf3: FsDieptxf3,
}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub struct FsGotgctl {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub mod fs_gotgctl;
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub struct FsGotgint {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub mod fs_gotgint;
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub struct FsGahbcfg {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub mod fs_gahbcfg;
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub struct FsGusbcfg {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub mod fs_gusbcfg;
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub struct FsGrstctl {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub mod fs_grstctl;
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub struct FsGintsts {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub mod fs_gintsts;
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub struct FsGintmsk {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub mod fs_gintmsk;
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub struct FsGrxstsrDevice {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub mod fs_grxstsr_device;
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub struct FsGrxstsrHost {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub mod fs_grxstsr_host;
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub struct FsGrxfsiz {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub mod fs_grxfsiz;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)"]
pub struct FsGnptxfsizDevice {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)"]
pub mod fs_gnptxfsiz_device;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Host mode)"]
pub struct FsGnptxfsizHost {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS non-periodic transmit FIFO size register (Host mode)"]
pub mod fs_gnptxfsiz_host;
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub struct FsGnptxsts {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub mod fs_gnptxsts;
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub struct FsGccfg {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub mod fs_gccfg;
#[doc = "core ID register"]
pub struct FsCid {
    register: VolatileCell<u32>,
}
#[doc = "core ID register"]
pub mod fs_cid;
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub struct FsHptxfsiz {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub mod fs_hptxfsiz;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
pub struct FsDieptxf1 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
pub mod fs_dieptxf1;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
pub struct FsDieptxf2 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
pub mod fs_dieptxf2;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
pub struct FsDieptxf3 {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
pub mod fs_dieptxf3;
