use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt mask register (EXTI_IMR)"] pub imr: Imr,
    #[doc = "0x04 - Event mask register (EXTI_EMR)"] pub emr: Emr,
    #[doc = "0x08 - Rising Trigger selection register (EXTI_RTSR)"] pub rtsr: Rtsr,
    #[doc = "0x0c - Falling Trigger selection register (EXTI_FTSR)"] pub ftsr: Ftsr,
    #[doc = "0x10 - Software interrupt event register (EXTI_SWIER)"] pub swier: Swier,
    #[doc = "0x14 - Pending register (EXTI_PR)"] pub pr: Pr,
}
#[doc = "Interrupt mask register (EXTI_IMR)"]
pub struct Imr {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt mask register (EXTI_IMR)"]
pub mod imr;
#[doc = "Event mask register (EXTI_EMR)"]
pub struct Emr {
    register: VolatileCell<u32>,
}
#[doc = "Event mask register (EXTI_EMR)"]
pub mod emr;
#[doc = "Rising Trigger selection register (EXTI_RTSR)"]
pub struct Rtsr {
    register: VolatileCell<u32>,
}
#[doc = "Rising Trigger selection register (EXTI_RTSR)"]
pub mod rtsr;
#[doc = "Falling Trigger selection register (EXTI_FTSR)"]
pub struct Ftsr {
    register: VolatileCell<u32>,
}
#[doc = "Falling Trigger selection register (EXTI_FTSR)"]
pub mod ftsr;
#[doc = "Software interrupt event register (EXTI_SWIER)"]
pub struct Swier {
    register: VolatileCell<u32>,
}
#[doc = "Software interrupt event register (EXTI_SWIER)"]
pub mod swier;
#[doc = "Pending register (EXTI_PR)"]
pub struct Pr {
    register: VolatileCell<u32>,
}
#[doc = "Pending register (EXTI_PR)"]
pub mod pr;
