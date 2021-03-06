use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)"]
    pub fs_pcgcctl: FsPcgcctl,
}
#[doc = "OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)"]
pub struct FsPcgcctl {
    register: VolatileCell<u32>,
}
#[doc = "OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)"]
pub mod fs_pcgcctl;
