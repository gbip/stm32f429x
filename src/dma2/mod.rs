use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - low interrupt status register"] pub lisr: Lisr,
    #[doc = "0x04 - high interrupt status register"] pub hisr: Hisr,
    #[doc = "0x08 - low interrupt flag clear register"] pub lifcr: Lifcr,
    #[doc = "0x0c - high interrupt flag clear register"] pub hifcr: Hifcr,
    #[doc = "0x10 - stream x configuration register"] pub s0cr: S0cr,
    #[doc = "0x14 - stream x number of data register"] pub s0ndtr: S0ndtr,
    #[doc = "0x18 - stream x peripheral address register"] pub s0par: S0par,
    #[doc = "0x1c - stream x memory 0 address register"] pub s0m0ar: S0m0ar,
    #[doc = "0x20 - stream x memory 1 address register"] pub s0m1ar: S0m1ar,
    #[doc = "0x24 - stream x FIFO control register"] pub s0fcr: S0fcr,
    #[doc = "0x28 - stream x configuration register"] pub s1cr: S1cr,
    #[doc = "0x2c - stream x number of data register"] pub s1ndtr: S1ndtr,
    #[doc = "0x30 - stream x peripheral address register"] pub s1par: S1par,
    #[doc = "0x34 - stream x memory 0 address register"] pub s1m0ar: S1m0ar,
    #[doc = "0x38 - stream x memory 1 address register"] pub s1m1ar: S1m1ar,
    #[doc = "0x3c - stream x FIFO control register"] pub s1fcr: S1fcr,
    #[doc = "0x40 - stream x configuration register"] pub s2cr: S2cr,
    #[doc = "0x44 - stream x number of data register"] pub s2ndtr: S2ndtr,
    #[doc = "0x48 - stream x peripheral address register"] pub s2par: S2par,
    #[doc = "0x4c - stream x memory 0 address register"] pub s2m0ar: S2m0ar,
    #[doc = "0x50 - stream x memory 1 address register"] pub s2m1ar: S2m1ar,
    #[doc = "0x54 - stream x FIFO control register"] pub s2fcr: S2fcr,
    #[doc = "0x58 - stream x configuration register"] pub s3cr: S3cr,
    #[doc = "0x5c - stream x number of data register"] pub s3ndtr: S3ndtr,
    #[doc = "0x60 - stream x peripheral address register"] pub s3par: S3par,
    #[doc = "0x64 - stream x memory 0 address register"] pub s3m0ar: S3m0ar,
    #[doc = "0x68 - stream x memory 1 address register"] pub s3m1ar: S3m1ar,
    #[doc = "0x6c - stream x FIFO control register"] pub s3fcr: S3fcr,
    #[doc = "0x70 - stream x configuration register"] pub s4cr: S4cr,
    #[doc = "0x74 - stream x number of data register"] pub s4ndtr: S4ndtr,
    #[doc = "0x78 - stream x peripheral address register"] pub s4par: S4par,
    #[doc = "0x7c - stream x memory 0 address register"] pub s4m0ar: S4m0ar,
    #[doc = "0x80 - stream x memory 1 address register"] pub s4m1ar: S4m1ar,
    #[doc = "0x84 - stream x FIFO control register"] pub s4fcr: S4fcr,
    #[doc = "0x88 - stream x configuration register"] pub s5cr: S5cr,
    #[doc = "0x8c - stream x number of data register"] pub s5ndtr: S5ndtr,
    #[doc = "0x90 - stream x peripheral address register"] pub s5par: S5par,
    #[doc = "0x94 - stream x memory 0 address register"] pub s5m0ar: S5m0ar,
    #[doc = "0x98 - stream x memory 1 address register"] pub s5m1ar: S5m1ar,
    #[doc = "0x9c - stream x FIFO control register"] pub s5fcr: S5fcr,
    #[doc = "0xa0 - stream x configuration register"] pub s6cr: S6cr,
    #[doc = "0xa4 - stream x number of data register"] pub s6ndtr: S6ndtr,
    #[doc = "0xa8 - stream x peripheral address register"] pub s6par: S6par,
    #[doc = "0xac - stream x memory 0 address register"] pub s6m0ar: S6m0ar,
    #[doc = "0xb0 - stream x memory 1 address register"] pub s6m1ar: S6m1ar,
    #[doc = "0xb4 - stream x FIFO control register"] pub s6fcr: S6fcr,
    #[doc = "0xb8 - stream x configuration register"] pub s7cr: S7cr,
    #[doc = "0xbc - stream x number of data register"] pub s7ndtr: S7ndtr,
    #[doc = "0xc0 - stream x peripheral address register"] pub s7par: S7par,
    #[doc = "0xc4 - stream x memory 0 address register"] pub s7m0ar: S7m0ar,
    #[doc = "0xc8 - stream x memory 1 address register"] pub s7m1ar: S7m1ar,
    #[doc = "0xcc - stream x FIFO control register"] pub s7fcr: S7fcr,
}
#[doc = "low interrupt status register"]
pub struct Lisr {
    register: VolatileCell<u32>,
}
#[doc = "low interrupt status register"]
pub mod lisr;
#[doc = "high interrupt status register"]
pub struct Hisr {
    register: VolatileCell<u32>,
}
#[doc = "high interrupt status register"]
pub mod hisr;
#[doc = "low interrupt flag clear register"]
pub struct Lifcr {
    register: VolatileCell<u32>,
}
#[doc = "low interrupt flag clear register"]
pub mod lifcr;
#[doc = "high interrupt flag clear register"]
pub struct Hifcr {
    register: VolatileCell<u32>,
}
#[doc = "high interrupt flag clear register"]
pub mod hifcr;
#[doc = "stream x configuration register"]
pub struct S0cr {
    register: VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s0cr;
#[doc = "stream x number of data register"]
pub struct S0ndtr {
    register: VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s0ndtr;
#[doc = "stream x peripheral address register"]
pub struct S0par {
    register: VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s0par;
#[doc = "stream x memory 0 address register"]
pub struct S0m0ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s0m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S0m1ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s0m1ar;
#[doc = "stream x FIFO control register"]
pub struct S0fcr {
    register: VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s0fcr;
#[doc = "stream x configuration register"]
pub struct S1cr {
    register: VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s1cr;
#[doc = "stream x number of data register"]
pub struct S1ndtr {
    register: VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s1ndtr;
#[doc = "stream x peripheral address register"]
pub struct S1par {
    register: VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s1par;
#[doc = "stream x memory 0 address register"]
pub struct S1m0ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s1m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S1m1ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s1m1ar;
#[doc = "stream x FIFO control register"]
pub struct S1fcr {
    register: VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s1fcr;
#[doc = "stream x configuration register"]
pub struct S2cr {
    register: VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s2cr;
#[doc = "stream x number of data register"]
pub struct S2ndtr {
    register: VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s2ndtr;
#[doc = "stream x peripheral address register"]
pub struct S2par {
    register: VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s2par;
#[doc = "stream x memory 0 address register"]
pub struct S2m0ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s2m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S2m1ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s2m1ar;
#[doc = "stream x FIFO control register"]
pub struct S2fcr {
    register: VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s2fcr;
#[doc = "stream x configuration register"]
pub struct S3cr {
    register: VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s3cr;
#[doc = "stream x number of data register"]
pub struct S3ndtr {
    register: VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s3ndtr;
#[doc = "stream x peripheral address register"]
pub struct S3par {
    register: VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s3par;
#[doc = "stream x memory 0 address register"]
pub struct S3m0ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s3m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S3m1ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s3m1ar;
#[doc = "stream x FIFO control register"]
pub struct S3fcr {
    register: VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s3fcr;
#[doc = "stream x configuration register"]
pub struct S4cr {
    register: VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s4cr;
#[doc = "stream x number of data register"]
pub struct S4ndtr {
    register: VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s4ndtr;
#[doc = "stream x peripheral address register"]
pub struct S4par {
    register: VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s4par;
#[doc = "stream x memory 0 address register"]
pub struct S4m0ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s4m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S4m1ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s4m1ar;
#[doc = "stream x FIFO control register"]
pub struct S4fcr {
    register: VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s4fcr;
#[doc = "stream x configuration register"]
pub struct S5cr {
    register: VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s5cr;
#[doc = "stream x number of data register"]
pub struct S5ndtr {
    register: VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s5ndtr;
#[doc = "stream x peripheral address register"]
pub struct S5par {
    register: VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s5par;
#[doc = "stream x memory 0 address register"]
pub struct S5m0ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s5m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S5m1ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s5m1ar;
#[doc = "stream x FIFO control register"]
pub struct S5fcr {
    register: VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s5fcr;
#[doc = "stream x configuration register"]
pub struct S6cr {
    register: VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s6cr;
#[doc = "stream x number of data register"]
pub struct S6ndtr {
    register: VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s6ndtr;
#[doc = "stream x peripheral address register"]
pub struct S6par {
    register: VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s6par;
#[doc = "stream x memory 0 address register"]
pub struct S6m0ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s6m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S6m1ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s6m1ar;
#[doc = "stream x FIFO control register"]
pub struct S6fcr {
    register: VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s6fcr;
#[doc = "stream x configuration register"]
pub struct S7cr {
    register: VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s7cr;
#[doc = "stream x number of data register"]
pub struct S7ndtr {
    register: VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s7ndtr;
#[doc = "stream x peripheral address register"]
pub struct S7par {
    register: VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s7par;
#[doc = "stream x memory 0 address register"]
pub struct S7m0ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s7m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S7m1ar {
    register: VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s7m1ar;
#[doc = "stream x FIFO control register"]
pub struct S7fcr {
    register: VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s7fcr;
