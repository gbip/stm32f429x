use cortex_m::ctxt::Context;
use cortex_m::exception;
use cortex_m::interrupt::Nr;
#[doc = "0 - Window Watchdog interrupt"]
pub struct Wwdg {
    _0: (),
}
unsafe impl Context for Wwdg {}
unsafe impl Nr for Wwdg {
    #[inline(always)]
    fn nr(&self) -> u8 {
        0
    }
}
#[doc = "1 - PVD through EXTI line detection interrupt"]
pub struct Pvd {
    _0: (),
}
unsafe impl Context for Pvd {}
unsafe impl Nr for Pvd {
    #[inline(always)]
    fn nr(&self) -> u8 {
        1
    }
}
#[doc = "2 - Tamper and TimeStamp interrupts through the EXTI line"]
pub struct TampStamp {
    _0: (),
}
unsafe impl Context for TampStamp {}
unsafe impl Nr for TampStamp {
    #[inline(always)]
    fn nr(&self) -> u8 {
        2
    }
}
#[doc = "3 - RTC Wakeup interrupt through the EXTI line"]
pub struct RtcWkup {
    _0: (),
}
unsafe impl Context for RtcWkup {}
unsafe impl Nr for RtcWkup {
    #[inline(always)]
    fn nr(&self) -> u8 {
        3
    }
}
#[doc = "4 - Flash global interrupt"]
pub struct Flash {
    _0: (),
}
unsafe impl Context for Flash {}
unsafe impl Nr for Flash {
    #[inline(always)]
    fn nr(&self) -> u8 {
        4
    }
}
#[doc = "5 - RCC global interrupt"]
pub struct Rcc {
    _0: (),
}
unsafe impl Context for Rcc {}
unsafe impl Nr for Rcc {
    #[inline(always)]
    fn nr(&self) -> u8 {
        5
    }
}
#[doc = "6 - EXTI Line0 interrupt"]
pub struct Exti0 {
    _0: (),
}
unsafe impl Context for Exti0 {}
unsafe impl Nr for Exti0 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        6
    }
}
#[doc = "7 - EXTI Line1 interrupt"]
pub struct Exti1 {
    _0: (),
}
unsafe impl Context for Exti1 {}
unsafe impl Nr for Exti1 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        7
    }
}
#[doc = "8 - EXTI Line2 interrupt"]
pub struct Exti2 {
    _0: (),
}
unsafe impl Context for Exti2 {}
unsafe impl Nr for Exti2 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        8
    }
}
#[doc = "9 - EXTI Line3 interrupt"]
pub struct Exti3 {
    _0: (),
}
unsafe impl Context for Exti3 {}
unsafe impl Nr for Exti3 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        9
    }
}
#[doc = "10 - EXTI Line4 interrupt"]
pub struct Exti4 {
    _0: (),
}
unsafe impl Context for Exti4 {}
unsafe impl Nr for Exti4 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        10
    }
}
#[doc = "11 - DMA1 Stream0 global interrupt"]
pub struct Dma1Stream0 {
    _0: (),
}
unsafe impl Context for Dma1Stream0 {}
unsafe impl Nr for Dma1Stream0 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        11
    }
}
#[doc = "12 - DMA1 Stream1 global interrupt"]
pub struct Dma1Stream1 {
    _0: (),
}
unsafe impl Context for Dma1Stream1 {}
unsafe impl Nr for Dma1Stream1 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        12
    }
}
#[doc = "13 - DMA1 Stream2 global interrupt"]
pub struct Dma1Stream2 {
    _0: (),
}
unsafe impl Context for Dma1Stream2 {}
unsafe impl Nr for Dma1Stream2 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        13
    }
}
#[doc = "14 - DMA1 Stream3 global interrupt"]
pub struct Dma1Stream3 {
    _0: (),
}
unsafe impl Context for Dma1Stream3 {}
unsafe impl Nr for Dma1Stream3 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        14
    }
}
#[doc = "15 - DMA1 Stream4 global interrupt"]
pub struct Dma1Stream4 {
    _0: (),
}
unsafe impl Context for Dma1Stream4 {}
unsafe impl Nr for Dma1Stream4 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        15
    }
}
#[doc = "16 - DMA1 Stream5 global interrupt"]
pub struct Dma1Stream5 {
    _0: (),
}
unsafe impl Context for Dma1Stream5 {}
unsafe impl Nr for Dma1Stream5 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        16
    }
}
#[doc = "17 - DMA1 Stream6 global interrupt"]
pub struct Dma1Stream6 {
    _0: (),
}
unsafe impl Context for Dma1Stream6 {}
unsafe impl Nr for Dma1Stream6 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        17
    }
}
#[doc = "18 - ADC3 global interrupts"]
pub struct Adc {
    _0: (),
}
unsafe impl Context for Adc {}
unsafe impl Nr for Adc {
    #[inline(always)]
    fn nr(&self) -> u8 {
        18
    }
}
#[doc = "19 - CAN1 TX interrupts"]
pub struct Can1Tx {
    _0: (),
}
unsafe impl Context for Can1Tx {}
unsafe impl Nr for Can1Tx {
    #[inline(always)]
    fn nr(&self) -> u8 {
        19
    }
}
#[doc = "20 - CAN1 RX0 interrupts"]
pub struct Can1Rx0 {
    _0: (),
}
unsafe impl Context for Can1Rx0 {}
unsafe impl Nr for Can1Rx0 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        20
    }
}
#[doc = "21 - CAN1 RX1 interrupts"]
pub struct Can1Rx1 {
    _0: (),
}
unsafe impl Context for Can1Rx1 {}
unsafe impl Nr for Can1Rx1 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        21
    }
}
#[doc = "22 - CAN1 SCE interrupt"]
pub struct Can1Sce {
    _0: (),
}
unsafe impl Context for Can1Sce {}
unsafe impl Nr for Can1Sce {
    #[inline(always)]
    fn nr(&self) -> u8 {
        22
    }
}
#[doc = "23 - EXTI Line[9:5] interrupts"]
pub struct Exti95 {
    _0: (),
}
unsafe impl Context for Exti95 {}
unsafe impl Nr for Exti95 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        23
    }
}
#[doc = "24 - TIM1 Break interrupt and TIM9 global interrupt"]
pub struct Tim1BrkTim9 {
    _0: (),
}
unsafe impl Context for Tim1BrkTim9 {}
unsafe impl Nr for Tim1BrkTim9 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        24
    }
}
#[doc = "25 - TIM1 Update interrupt and TIM10 global interrupt"]
pub struct Tim1UpTim10 {
    _0: (),
}
unsafe impl Context for Tim1UpTim10 {}
unsafe impl Nr for Tim1UpTim10 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        25
    }
}
#[doc = "26 - TIM1 Trigger and Commutation interrupts and TIM11 global interrupt"]
pub struct Tim1TrgComTim11 {
    _0: (),
}
unsafe impl Context for Tim1TrgComTim11 {}
unsafe impl Nr for Tim1TrgComTim11 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        26
    }
}
#[doc = "27 - TIM1 Capture Compare interrupt"]
pub struct Tim1Cc {
    _0: (),
}
unsafe impl Context for Tim1Cc {}
unsafe impl Nr for Tim1Cc {
    #[inline(always)]
    fn nr(&self) -> u8 {
        27
    }
}
#[doc = "28 - TIM2 global interrupt"]
pub struct Tim2 {
    _0: (),
}
unsafe impl Context for Tim2 {}
unsafe impl Nr for Tim2 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        28
    }
}
#[doc = "29 - TIM3 global interrupt"]
pub struct Tim3 {
    _0: (),
}
unsafe impl Context for Tim3 {}
unsafe impl Nr for Tim3 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        29
    }
}
#[doc = "30 - TIM4 global interrupt"]
pub struct Tim4 {
    _0: (),
}
unsafe impl Context for Tim4 {}
unsafe impl Nr for Tim4 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        30
    }
}
#[doc = "31 - I2C1 event interrupt"]
pub struct I2c1Ev {
    _0: (),
}
unsafe impl Context for I2c1Ev {}
unsafe impl Nr for I2c1Ev {
    #[inline(always)]
    fn nr(&self) -> u8 {
        31
    }
}
#[doc = "32 - I2C1 error interrupt"]
pub struct I2c1Er {
    _0: (),
}
unsafe impl Context for I2c1Er {}
unsafe impl Nr for I2c1Er {
    #[inline(always)]
    fn nr(&self) -> u8 {
        32
    }
}
#[doc = "33 - I2C2 event interrupt"]
pub struct I2c2Ev {
    _0: (),
}
unsafe impl Context for I2c2Ev {}
unsafe impl Nr for I2c2Ev {
    #[inline(always)]
    fn nr(&self) -> u8 {
        33
    }
}
#[doc = "34 - I2C2 error interrupt"]
pub struct I2c2Er {
    _0: (),
}
unsafe impl Context for I2c2Er {}
unsafe impl Nr for I2c2Er {
    #[inline(always)]
    fn nr(&self) -> u8 {
        34
    }
}
#[doc = "35 - SPI1 global interrupt"]
pub struct Spi1 {
    _0: (),
}
unsafe impl Context for Spi1 {}
unsafe impl Nr for Spi1 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        35
    }
}
#[doc = "36 - SPI2 global interrupt"]
pub struct Spi2 {
    _0: (),
}
unsafe impl Context for Spi2 {}
unsafe impl Nr for Spi2 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        36
    }
}
#[doc = "37 - USART1 global interrupt"]
pub struct Usart1 {
    _0: (),
}
unsafe impl Context for Usart1 {}
unsafe impl Nr for Usart1 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        37
    }
}
#[doc = "38 - USART2 global interrupt"]
pub struct Usart2 {
    _0: (),
}
unsafe impl Context for Usart2 {}
unsafe impl Nr for Usart2 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        38
    }
}
#[doc = "39 - USART3 global interrupt"]
pub struct Usart3 {
    _0: (),
}
unsafe impl Context for Usart3 {}
unsafe impl Nr for Usart3 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        39
    }
}
#[doc = "40 - EXTI Line[15:10] interrupts"]
pub struct Exti1510 {
    _0: (),
}
unsafe impl Context for Exti1510 {}
unsafe impl Nr for Exti1510 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        40
    }
}
#[doc = "41 - RTC Alarms (A and B) through EXTI line interrupt"]
pub struct RtcAlarm {
    _0: (),
}
unsafe impl Context for RtcAlarm {}
unsafe impl Nr for RtcAlarm {
    #[inline(always)]
    fn nr(&self) -> u8 {
        41
    }
}
#[doc = "42 - USB On-The-Go FS Wakeup through EXTI line interrupt"]
pub struct OtgFsWkup {
    _0: (),
}
unsafe impl Context for OtgFsWkup {}
unsafe impl Nr for OtgFsWkup {
    #[inline(always)]
    fn nr(&self) -> u8 {
        42
    }
}
#[doc = "43 - TIM8 Break interrupt and TIM12 global interrupt"]
pub struct Tim8BrkTim12 {
    _0: (),
}
unsafe impl Context for Tim8BrkTim12 {}
unsafe impl Nr for Tim8BrkTim12 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        43
    }
}
#[doc = "44 - TIM8 Update interrupt and TIM13 global interrupt"]
pub struct Tim8UpTim13 {
    _0: (),
}
unsafe impl Context for Tim8UpTim13 {}
unsafe impl Nr for Tim8UpTim13 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        44
    }
}
#[doc = "45 - TIM8 Trigger and Commutation interrupts and TIM14 global interrupt"]
pub struct Tim8TrgComTim14 {
    _0: (),
}
unsafe impl Context for Tim8TrgComTim14 {}
unsafe impl Nr for Tim8TrgComTim14 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        45
    }
}
#[doc = "46 - TIM8 Capture Compare interrupt"]
pub struct Tim8Cc {
    _0: (),
}
unsafe impl Context for Tim8Cc {}
unsafe impl Nr for Tim8Cc {
    #[inline(always)]
    fn nr(&self) -> u8 {
        46
    }
}
#[doc = "47 - DMA1 Stream7 global interrupt"]
pub struct Dma1Stream7 {
    _0: (),
}
unsafe impl Context for Dma1Stream7 {}
unsafe impl Nr for Dma1Stream7 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        47
    }
}
#[doc = "48 - FMC global interrupt"]
pub struct Fmc {
    _0: (),
}
unsafe impl Context for Fmc {}
unsafe impl Nr for Fmc {
    #[inline(always)]
    fn nr(&self) -> u8 {
        48
    }
}
#[doc = "49 - SDIO global interrupt"]
pub struct Sdio {
    _0: (),
}
unsafe impl Context for Sdio {}
unsafe impl Nr for Sdio {
    #[inline(always)]
    fn nr(&self) -> u8 {
        49
    }
}
#[doc = "50 - TIM5 global interrupt"]
pub struct Tim5 {
    _0: (),
}
unsafe impl Context for Tim5 {}
unsafe impl Nr for Tim5 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        50
    }
}
#[doc = "51 - SPI3 global interrupt"]
pub struct Spi3 {
    _0: (),
}
unsafe impl Context for Spi3 {}
unsafe impl Nr for Spi3 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        51
    }
}
#[doc = "52 - UART4 global interrupt"]
pub struct Uart4 {
    _0: (),
}
unsafe impl Context for Uart4 {}
unsafe impl Nr for Uart4 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        52
    }
}
#[doc = "53 - UART5 global interrupt"]
pub struct Uart5 {
    _0: (),
}
unsafe impl Context for Uart5 {}
unsafe impl Nr for Uart5 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        53
    }
}
#[doc = "54 - TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt"]
pub struct Tim6Dac {
    _0: (),
}
unsafe impl Context for Tim6Dac {}
unsafe impl Nr for Tim6Dac {
    #[inline(always)]
    fn nr(&self) -> u8 {
        54
    }
}
#[doc = "55 - TIM7 global interrupt"]
pub struct Tim7 {
    _0: (),
}
unsafe impl Context for Tim7 {}
unsafe impl Nr for Tim7 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        55
    }
}
#[doc = "56 - DMA2 Stream0 global interrupt"]
pub struct Dma2Stream0 {
    _0: (),
}
unsafe impl Context for Dma2Stream0 {}
unsafe impl Nr for Dma2Stream0 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        56
    }
}
#[doc = "57 - DMA2 Stream1 global interrupt"]
pub struct Dma2Stream1 {
    _0: (),
}
unsafe impl Context for Dma2Stream1 {}
unsafe impl Nr for Dma2Stream1 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        57
    }
}
#[doc = "58 - DMA2 Stream2 global interrupt"]
pub struct Dma2Stream2 {
    _0: (),
}
unsafe impl Context for Dma2Stream2 {}
unsafe impl Nr for Dma2Stream2 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        58
    }
}
#[doc = "59 - DMA2 Stream3 global interrupt"]
pub struct Dma2Stream3 {
    _0: (),
}
unsafe impl Context for Dma2Stream3 {}
unsafe impl Nr for Dma2Stream3 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        59
    }
}
#[doc = "60 - DMA2 Stream4 global interrupt"]
pub struct Dma2Stream4 {
    _0: (),
}
unsafe impl Context for Dma2Stream4 {}
unsafe impl Nr for Dma2Stream4 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        60
    }
}
#[doc = "61 - Ethernet global interrupt"]
pub struct Eth {
    _0: (),
}
unsafe impl Context for Eth {}
unsafe impl Nr for Eth {
    #[inline(always)]
    fn nr(&self) -> u8 {
        61
    }
}
#[doc = "62 - Ethernet Wakeup through EXTI line interrupt"]
pub struct EthWkup {
    _0: (),
}
unsafe impl Context for EthWkup {}
unsafe impl Nr for EthWkup {
    #[inline(always)]
    fn nr(&self) -> u8 {
        62
    }
}
#[doc = "63 - CAN2 TX interrupts"]
pub struct Can2Tx {
    _0: (),
}
unsafe impl Context for Can2Tx {}
unsafe impl Nr for Can2Tx {
    #[inline(always)]
    fn nr(&self) -> u8 {
        63
    }
}
#[doc = "64 - CAN2 RX0 interrupts"]
pub struct Can2Rx0 {
    _0: (),
}
unsafe impl Context for Can2Rx0 {}
unsafe impl Nr for Can2Rx0 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        64
    }
}
#[doc = "65 - CAN2 RX1 interrupts"]
pub struct Can2Rx1 {
    _0: (),
}
unsafe impl Context for Can2Rx1 {}
unsafe impl Nr for Can2Rx1 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        65
    }
}
#[doc = "66 - CAN2 SCE interrupt"]
pub struct Can2Sce {
    _0: (),
}
unsafe impl Context for Can2Sce {}
unsafe impl Nr for Can2Sce {
    #[inline(always)]
    fn nr(&self) -> u8 {
        66
    }
}
#[doc = "67 - USB On The Go FS global interrupt"]
pub struct OtgFs {
    _0: (),
}
unsafe impl Context for OtgFs {}
unsafe impl Nr for OtgFs {
    #[inline(always)]
    fn nr(&self) -> u8 {
        67
    }
}
#[doc = "68 - DMA2 Stream5 global interrupt"]
pub struct Dma2Stream5 {
    _0: (),
}
unsafe impl Context for Dma2Stream5 {}
unsafe impl Nr for Dma2Stream5 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        68
    }
}
#[doc = "69 - DMA2 Stream6 global interrupt"]
pub struct Dma2Stream6 {
    _0: (),
}
unsafe impl Context for Dma2Stream6 {}
unsafe impl Nr for Dma2Stream6 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        69
    }
}
#[doc = "70 - DMA2 Stream7 global interrupt"]
pub struct Dma2Stream7 {
    _0: (),
}
unsafe impl Context for Dma2Stream7 {}
unsafe impl Nr for Dma2Stream7 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        70
    }
}
#[doc = "71 - USART6 global interrupt"]
pub struct Usart6 {
    _0: (),
}
unsafe impl Context for Usart6 {}
unsafe impl Nr for Usart6 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        71
    }
}
#[doc = "72 - I2C3 event interrupt"]
pub struct I2c3Ev {
    _0: (),
}
unsafe impl Context for I2c3Ev {}
unsafe impl Nr for I2c3Ev {
    #[inline(always)]
    fn nr(&self) -> u8 {
        72
    }
}
#[doc = "73 - I2C3 error interrupt"]
pub struct I2c3Er {
    _0: (),
}
unsafe impl Context for I2c3Er {}
unsafe impl Nr for I2c3Er {
    #[inline(always)]
    fn nr(&self) -> u8 {
        73
    }
}
#[doc = "74 - USB On The Go HS End Point 1 Out global interrupt"]
pub struct OtgHsEp1Out {
    _0: (),
}
unsafe impl Context for OtgHsEp1Out {}
unsafe impl Nr for OtgHsEp1Out {
    #[inline(always)]
    fn nr(&self) -> u8 {
        74
    }
}
#[doc = "75 - USB On The Go HS End Point 1 In global interrupt"]
pub struct OtgHsEp1In {
    _0: (),
}
unsafe impl Context for OtgHsEp1In {}
unsafe impl Nr for OtgHsEp1In {
    #[inline(always)]
    fn nr(&self) -> u8 {
        75
    }
}
#[doc = "76 - USB On The Go HS Wakeup through EXTI interrupt"]
pub struct OtgHsWkup {
    _0: (),
}
unsafe impl Context for OtgHsWkup {}
unsafe impl Nr for OtgHsWkup {
    #[inline(always)]
    fn nr(&self) -> u8 {
        76
    }
}
#[doc = "77 - USB On The Go HS global interrupt"]
pub struct OtgHs {
    _0: (),
}
unsafe impl Context for OtgHs {}
unsafe impl Nr for OtgHs {
    #[inline(always)]
    fn nr(&self) -> u8 {
        77
    }
}
#[doc = "78 - DCMI global interrupt"]
pub struct Dcmi {
    _0: (),
}
unsafe impl Context for Dcmi {}
unsafe impl Nr for Dcmi {
    #[inline(always)]
    fn nr(&self) -> u8 {
        78
    }
}
#[doc = "80 - Rng global interrupt"]
pub struct Rng {
    _0: (),
}
unsafe impl Context for Rng {}
unsafe impl Nr for Rng {
    #[inline(always)]
    fn nr(&self) -> u8 {
        80
    }
}
#[doc = "81 - FPU interrupt"]
pub struct Fpu {
    _0: (),
}
unsafe impl Context for Fpu {}
unsafe impl Nr for Fpu {
    #[inline(always)]
    fn nr(&self) -> u8 {
        81
    }
}
#[doc = "82 - UART 7 global interrupt"]
pub struct Uart7 {
    _0: (),
}
unsafe impl Context for Uart7 {}
unsafe impl Nr for Uart7 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        82
    }
}
#[doc = "83 - UART 8 global interrupt"]
pub struct Uart8 {
    _0: (),
}
unsafe impl Context for Uart8 {}
unsafe impl Nr for Uart8 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        83
    }
}
#[doc = "84 - SPI 4 global interrupt"]
pub struct Spi4 {
    _0: (),
}
unsafe impl Context for Spi4 {}
unsafe impl Nr for Spi4 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        84
    }
}
#[doc = "85 - SPI 5 global interrupt"]
pub struct Spi5 {
    _0: (),
}
unsafe impl Context for Spi5 {}
unsafe impl Nr for Spi5 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        85
    }
}
#[doc = "86 - SPI 6 global interrupt"]
pub struct Spi6 {
    _0: (),
}
unsafe impl Context for Spi6 {}
unsafe impl Nr for Spi6 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        86
    }
}
#[doc = "87 - SAI1 global interrupt"]
pub struct Sai1 {
    _0: (),
}
unsafe impl Context for Sai1 {}
unsafe impl Nr for Sai1 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        87
    }
}
#[doc = "88 - LTDC global interrupt"]
pub struct LcdTft {
    _0: (),
}
unsafe impl Context for LcdTft {}
unsafe impl Nr for LcdTft {
    #[inline(always)]
    fn nr(&self) -> u8 {
        88
    }
}
#[doc = "89 - LTDC global error interrupt"]
pub struct LcdTft1 {
    _0: (),
}
unsafe impl Context for LcdTft1 {}
unsafe impl Nr for LcdTft1 {
    #[inline(always)]
    fn nr(&self) -> u8 {
        89
    }
}
#[doc = "90 - DMA2D global interrupt"]
pub struct Dma2d {
    _0: (),
}
unsafe impl Context for Dma2d {}
unsafe impl Nr for Dma2d {
    #[inline(always)]
    fn nr(&self) -> u8 {
        90
    }
}
use cortex_m::Reserved;
#[doc = r" Interrupt handlers"]
#[allow(non_snake_case)]
#[repr(C)]
pub struct Handlers {
    #[doc = "0 - Window Watchdog interrupt"] pub Wwdg: extern "C" fn(Wwdg),
    #[doc = "1 - PVD through EXTI line detection interrupt"] pub Pvd: extern "C" fn(Pvd),
    #[doc = "2 - Tamper and TimeStamp interrupts through the EXTI line"]
    pub TampStamp: extern "C" fn(TampStamp),
    #[doc = "3 - RTC Wakeup interrupt through the EXTI line"] pub RtcWkup: extern "C" fn(RtcWkup),
    #[doc = "4 - Flash global interrupt"] pub Flash: extern "C" fn(Flash),
    #[doc = "5 - RCC global interrupt"] pub Rcc: extern "C" fn(Rcc),
    #[doc = "6 - EXTI Line0 interrupt"] pub Exti0: extern "C" fn(Exti0),
    #[doc = "7 - EXTI Line1 interrupt"] pub Exti1: extern "C" fn(Exti1),
    #[doc = "8 - EXTI Line2 interrupt"] pub Exti2: extern "C" fn(Exti2),
    #[doc = "9 - EXTI Line3 interrupt"] pub Exti3: extern "C" fn(Exti3),
    #[doc = "10 - EXTI Line4 interrupt"] pub Exti4: extern "C" fn(Exti4),
    #[doc = "11 - DMA1 Stream0 global interrupt"] pub Dma1Stream0: extern "C" fn(Dma1Stream0),
    #[doc = "12 - DMA1 Stream1 global interrupt"] pub Dma1Stream1: extern "C" fn(Dma1Stream1),
    #[doc = "13 - DMA1 Stream2 global interrupt"] pub Dma1Stream2: extern "C" fn(Dma1Stream2),
    #[doc = "14 - DMA1 Stream3 global interrupt"] pub Dma1Stream3: extern "C" fn(Dma1Stream3),
    #[doc = "15 - DMA1 Stream4 global interrupt"] pub Dma1Stream4: extern "C" fn(Dma1Stream4),
    #[doc = "16 - DMA1 Stream5 global interrupt"] pub Dma1Stream5: extern "C" fn(Dma1Stream5),
    #[doc = "17 - DMA1 Stream6 global interrupt"] pub Dma1Stream6: extern "C" fn(Dma1Stream6),
    #[doc = "18 - ADC3 global interrupts"] pub Adc: extern "C" fn(Adc),
    #[doc = "19 - CAN1 TX interrupts"] pub Can1Tx: extern "C" fn(Can1Tx),
    #[doc = "20 - CAN1 RX0 interrupts"] pub Can1Rx0: extern "C" fn(Can1Rx0),
    #[doc = "21 - CAN1 RX1 interrupts"] pub Can1Rx1: extern "C" fn(Can1Rx1),
    #[doc = "22 - CAN1 SCE interrupt"] pub Can1Sce: extern "C" fn(Can1Sce),
    #[doc = "23 - EXTI Line[9:5] interrupts"] pub Exti95: extern "C" fn(Exti95),
    #[doc = "24 - TIM1 Break interrupt and TIM9 global interrupt"]
    pub Tim1BrkTim9: extern "C" fn(Tim1BrkTim9),
    #[doc = "25 - TIM1 Update interrupt and TIM10 global interrupt"]
    pub Tim1UpTim10: extern "C" fn(Tim1UpTim10),
    #[doc = "26 - TIM1 Trigger and Commutation interrupts and TIM11 global interrupt"]
    pub Tim1TrgComTim11: extern "C" fn(Tim1TrgComTim11),
    #[doc = "27 - TIM1 Capture Compare interrupt"] pub Tim1Cc: extern "C" fn(Tim1Cc),
    #[doc = "28 - TIM2 global interrupt"] pub Tim2: extern "C" fn(Tim2),
    #[doc = "29 - TIM3 global interrupt"] pub Tim3: extern "C" fn(Tim3),
    #[doc = "30 - TIM4 global interrupt"] pub Tim4: extern "C" fn(Tim4),
    #[doc = "31 - I2C1 event interrupt"] pub I2c1Ev: extern "C" fn(I2c1Ev),
    #[doc = "32 - I2C1 error interrupt"] pub I2c1Er: extern "C" fn(I2c1Er),
    #[doc = "33 - I2C2 event interrupt"] pub I2c2Ev: extern "C" fn(I2c2Ev),
    #[doc = "34 - I2C2 error interrupt"] pub I2c2Er: extern "C" fn(I2c2Er),
    #[doc = "35 - SPI1 global interrupt"] pub Spi1: extern "C" fn(Spi1),
    #[doc = "36 - SPI2 global interrupt"] pub Spi2: extern "C" fn(Spi2),
    #[doc = "37 - USART1 global interrupt"] pub Usart1: extern "C" fn(Usart1),
    #[doc = "38 - USART2 global interrupt"] pub Usart2: extern "C" fn(Usart2),
    #[doc = "39 - USART3 global interrupt"] pub Usart3: extern "C" fn(Usart3),
    #[doc = "40 - EXTI Line[15:10] interrupts"] pub Exti1510: extern "C" fn(Exti1510),
    #[doc = "41 - RTC Alarms (A and B) through EXTI line interrupt"]
    pub RtcAlarm: extern "C" fn(RtcAlarm),
    #[doc = "42 - USB On-The-Go FS Wakeup through EXTI line interrupt"]
    pub OtgFsWkup: extern "C" fn(OtgFsWkup),
    #[doc = "43 - TIM8 Break interrupt and TIM12 global interrupt"]
    pub Tim8BrkTim12: extern "C" fn(Tim8BrkTim12),
    #[doc = "44 - TIM8 Update interrupt and TIM13 global interrupt"]
    pub Tim8UpTim13: extern "C" fn(Tim8UpTim13),
    #[doc = "45 - TIM8 Trigger and Commutation interrupts and TIM14 global interrupt"]
    pub Tim8TrgComTim14: extern "C" fn(Tim8TrgComTim14),
    #[doc = "46 - TIM8 Capture Compare interrupt"] pub Tim8Cc: extern "C" fn(Tim8Cc),
    #[doc = "47 - DMA1 Stream7 global interrupt"] pub Dma1Stream7: extern "C" fn(Dma1Stream7),
    #[doc = "48 - FMC global interrupt"] pub Fmc: extern "C" fn(Fmc),
    #[doc = "49 - SDIO global interrupt"] pub Sdio: extern "C" fn(Sdio),
    #[doc = "50 - TIM5 global interrupt"] pub Tim5: extern "C" fn(Tim5),
    #[doc = "51 - SPI3 global interrupt"] pub Spi3: extern "C" fn(Spi3),
    #[doc = "52 - UART4 global interrupt"] pub Uart4: extern "C" fn(Uart4),
    #[doc = "53 - UART5 global interrupt"] pub Uart5: extern "C" fn(Uart5),
    #[doc = "54 - TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt"]
    pub Tim6Dac: extern "C" fn(Tim6Dac),
    #[doc = "55 - TIM7 global interrupt"] pub Tim7: extern "C" fn(Tim7),
    #[doc = "56 - DMA2 Stream0 global interrupt"] pub Dma2Stream0: extern "C" fn(Dma2Stream0),
    #[doc = "57 - DMA2 Stream1 global interrupt"] pub Dma2Stream1: extern "C" fn(Dma2Stream1),
    #[doc = "58 - DMA2 Stream2 global interrupt"] pub Dma2Stream2: extern "C" fn(Dma2Stream2),
    #[doc = "59 - DMA2 Stream3 global interrupt"] pub Dma2Stream3: extern "C" fn(Dma2Stream3),
    #[doc = "60 - DMA2 Stream4 global interrupt"] pub Dma2Stream4: extern "C" fn(Dma2Stream4),
    #[doc = "61 - Ethernet global interrupt"] pub Eth: extern "C" fn(Eth),
    #[doc = "62 - Ethernet Wakeup through EXTI line interrupt"] pub EthWkup: extern "C" fn(EthWkup),
    #[doc = "63 - CAN2 TX interrupts"] pub Can2Tx: extern "C" fn(Can2Tx),
    #[doc = "64 - CAN2 RX0 interrupts"] pub Can2Rx0: extern "C" fn(Can2Rx0),
    #[doc = "65 - CAN2 RX1 interrupts"] pub Can2Rx1: extern "C" fn(Can2Rx1),
    #[doc = "66 - CAN2 SCE interrupt"] pub Can2Sce: extern "C" fn(Can2Sce),
    #[doc = "67 - USB On The Go FS global interrupt"] pub OtgFs: extern "C" fn(OtgFs),
    #[doc = "68 - DMA2 Stream5 global interrupt"] pub Dma2Stream5: extern "C" fn(Dma2Stream5),
    #[doc = "69 - DMA2 Stream6 global interrupt"] pub Dma2Stream6: extern "C" fn(Dma2Stream6),
    #[doc = "70 - DMA2 Stream7 global interrupt"] pub Dma2Stream7: extern "C" fn(Dma2Stream7),
    #[doc = "71 - USART6 global interrupt"] pub Usart6: extern "C" fn(Usart6),
    #[doc = "72 - I2C3 event interrupt"] pub I2c3Ev: extern "C" fn(I2c3Ev),
    #[doc = "73 - I2C3 error interrupt"] pub I2c3Er: extern "C" fn(I2c3Er),
    #[doc = "74 - USB On The Go HS End Point 1 Out global interrupt"]
    pub OtgHsEp1Out: extern "C" fn(OtgHsEp1Out),
    #[doc = "75 - USB On The Go HS End Point 1 In global interrupt"]
    pub OtgHsEp1In: extern "C" fn(OtgHsEp1In),
    #[doc = "76 - USB On The Go HS Wakeup through EXTI interrupt"]
    pub OtgHsWkup: extern "C" fn(OtgHsWkup),
    #[doc = "77 - USB On The Go HS global interrupt"] pub OtgHs: extern "C" fn(OtgHs),
    #[doc = "78 - DCMI global interrupt"] pub Dcmi: extern "C" fn(Dcmi),
    #[doc = r" Reserved spot in the vector table"] pub _reserved0: [Reserved; 1],
    #[doc = "80 - Rng global interrupt"] pub Rng: extern "C" fn(Rng),
    #[doc = "81 - FPU interrupt"] pub Fpu: extern "C" fn(Fpu),
    #[doc = "82 - UART 7 global interrupt"] pub Uart7: extern "C" fn(Uart7),
    #[doc = "83 - UART 8 global interrupt"] pub Uart8: extern "C" fn(Uart8),
    #[doc = "84 - SPI 4 global interrupt"] pub Spi4: extern "C" fn(Spi4),
    #[doc = "85 - SPI 5 global interrupt"] pub Spi5: extern "C" fn(Spi5),
    #[doc = "86 - SPI 6 global interrupt"] pub Spi6: extern "C" fn(Spi6),
    #[doc = "87 - SAI1 global interrupt"] pub Sai1: extern "C" fn(Sai1),
    #[doc = "88 - LTDC global interrupt"] pub LcdTft: extern "C" fn(LcdTft),
    #[doc = "89 - LTDC global error interrupt"] pub LcdTft1: extern "C" fn(LcdTft1),
    #[doc = "90 - DMA2D global interrupt"] pub Dma2d: extern "C" fn(Dma2d),
}
#[doc = r" Default interrupt handlers"]
pub const DEFAULT_HANDLERS: Handlers = Handlers {
    Wwdg: exception::default_handler,
    Pvd: exception::default_handler,
    TampStamp: exception::default_handler,
    RtcWkup: exception::default_handler,
    Flash: exception::default_handler,
    Rcc: exception::default_handler,
    Exti0: exception::default_handler,
    Exti1: exception::default_handler,
    Exti2: exception::default_handler,
    Exti3: exception::default_handler,
    Exti4: exception::default_handler,
    Dma1Stream0: exception::default_handler,
    Dma1Stream1: exception::default_handler,
    Dma1Stream2: exception::default_handler,
    Dma1Stream3: exception::default_handler,
    Dma1Stream4: exception::default_handler,
    Dma1Stream5: exception::default_handler,
    Dma1Stream6: exception::default_handler,
    Adc: exception::default_handler,
    Can1Tx: exception::default_handler,
    Can1Rx0: exception::default_handler,
    Can1Rx1: exception::default_handler,
    Can1Sce: exception::default_handler,
    Exti95: exception::default_handler,
    Tim1BrkTim9: exception::default_handler,
    Tim1UpTim10: exception::default_handler,
    Tim1TrgComTim11: exception::default_handler,
    Tim1Cc: exception::default_handler,
    Tim2: exception::default_handler,
    Tim3: exception::default_handler,
    Tim4: exception::default_handler,
    I2c1Ev: exception::default_handler,
    I2c1Er: exception::default_handler,
    I2c2Ev: exception::default_handler,
    I2c2Er: exception::default_handler,
    Spi1: exception::default_handler,
    Spi2: exception::default_handler,
    Usart1: exception::default_handler,
    Usart2: exception::default_handler,
    Usart3: exception::default_handler,
    Exti1510: exception::default_handler,
    RtcAlarm: exception::default_handler,
    OtgFsWkup: exception::default_handler,
    Tim8BrkTim12: exception::default_handler,
    Tim8UpTim13: exception::default_handler,
    Tim8TrgComTim14: exception::default_handler,
    Tim8Cc: exception::default_handler,
    Dma1Stream7: exception::default_handler,
    Fmc: exception::default_handler,
    Sdio: exception::default_handler,
    Tim5: exception::default_handler,
    Spi3: exception::default_handler,
    Uart4: exception::default_handler,
    Uart5: exception::default_handler,
    Tim6Dac: exception::default_handler,
    Tim7: exception::default_handler,
    Dma2Stream0: exception::default_handler,
    Dma2Stream1: exception::default_handler,
    Dma2Stream2: exception::default_handler,
    Dma2Stream3: exception::default_handler,
    Dma2Stream4: exception::default_handler,
    Eth: exception::default_handler,
    EthWkup: exception::default_handler,
    Can2Tx: exception::default_handler,
    Can2Rx0: exception::default_handler,
    Can2Rx1: exception::default_handler,
    Can2Sce: exception::default_handler,
    OtgFs: exception::default_handler,
    Dma2Stream5: exception::default_handler,
    Dma2Stream6: exception::default_handler,
    Dma2Stream7: exception::default_handler,
    Usart6: exception::default_handler,
    I2c3Ev: exception::default_handler,
    I2c3Er: exception::default_handler,
    OtgHsEp1Out: exception::default_handler,
    OtgHsEp1In: exception::default_handler,
    OtgHsWkup: exception::default_handler,
    OtgHs: exception::default_handler,
    Dcmi: exception::default_handler,
    _reserved0: [Reserved::Vector; 1],
    Rng: exception::default_handler,
    Fpu: exception::default_handler,
    Uart7: exception::default_handler,
    Uart8: exception::default_handler,
    Spi4: exception::default_handler,
    Spi5: exception::default_handler,
    Spi6: exception::default_handler,
    Sai1: exception::default_handler,
    LcdTft: exception::default_handler,
    LcdTft1: exception::default_handler,
    Dma2d: exception::default_handler,
};
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"] Wwdg,
    #[doc = "1 - PVD through EXTI line detection interrupt"] Pvd,
    #[doc = "2 - Tamper and TimeStamp interrupts through the EXTI line"] TampStamp,
    #[doc = "3 - RTC Wakeup interrupt through the EXTI line"] RtcWkup,
    #[doc = "4 - Flash global interrupt"] Flash,
    #[doc = "5 - RCC global interrupt"] Rcc,
    #[doc = "6 - EXTI Line0 interrupt"] Exti0,
    #[doc = "7 - EXTI Line1 interrupt"] Exti1,
    #[doc = "8 - EXTI Line2 interrupt"] Exti2,
    #[doc = "9 - EXTI Line3 interrupt"] Exti3,
    #[doc = "10 - EXTI Line4 interrupt"] Exti4,
    #[doc = "11 - DMA1 Stream0 global interrupt"] Dma1Stream0,
    #[doc = "12 - DMA1 Stream1 global interrupt"] Dma1Stream1,
    #[doc = "13 - DMA1 Stream2 global interrupt"] Dma1Stream2,
    #[doc = "14 - DMA1 Stream3 global interrupt"] Dma1Stream3,
    #[doc = "15 - DMA1 Stream4 global interrupt"] Dma1Stream4,
    #[doc = "16 - DMA1 Stream5 global interrupt"] Dma1Stream5,
    #[doc = "17 - DMA1 Stream6 global interrupt"] Dma1Stream6,
    #[doc = "18 - ADC3 global interrupts"] Adc,
    #[doc = "19 - CAN1 TX interrupts"] Can1Tx,
    #[doc = "20 - CAN1 RX0 interrupts"] Can1Rx0,
    #[doc = "21 - CAN1 RX1 interrupts"] Can1Rx1,
    #[doc = "22 - CAN1 SCE interrupt"] Can1Sce,
    #[doc = "23 - EXTI Line[9:5] interrupts"] Exti95,
    #[doc = "24 - TIM1 Break interrupt and TIM9 global interrupt"] Tim1BrkTim9,
    #[doc = "25 - TIM1 Update interrupt and TIM10 global interrupt"] Tim1UpTim10,
    #[doc = "26 - TIM1 Trigger and Commutation interrupts and TIM11 global interrupt"]
    Tim1TrgComTim11,
    #[doc = "27 - TIM1 Capture Compare interrupt"] Tim1Cc,
    #[doc = "28 - TIM2 global interrupt"] Tim2,
    #[doc = "29 - TIM3 global interrupt"] Tim3,
    #[doc = "30 - TIM4 global interrupt"] Tim4,
    #[doc = "31 - I2C1 event interrupt"] I2c1Ev,
    #[doc = "32 - I2C1 error interrupt"] I2c1Er,
    #[doc = "33 - I2C2 event interrupt"] I2c2Ev,
    #[doc = "34 - I2C2 error interrupt"] I2c2Er,
    #[doc = "35 - SPI1 global interrupt"] Spi1,
    #[doc = "36 - SPI2 global interrupt"] Spi2,
    #[doc = "37 - USART1 global interrupt"] Usart1,
    #[doc = "38 - USART2 global interrupt"] Usart2,
    #[doc = "39 - USART3 global interrupt"] Usart3,
    #[doc = "40 - EXTI Line[15:10] interrupts"] Exti1510,
    #[doc = "41 - RTC Alarms (A and B) through EXTI line interrupt"] RtcAlarm,
    #[doc = "42 - USB On-The-Go FS Wakeup through EXTI line interrupt"] OtgFsWkup,
    #[doc = "43 - TIM8 Break interrupt and TIM12 global interrupt"] Tim8BrkTim12,
    #[doc = "44 - TIM8 Update interrupt and TIM13 global interrupt"] Tim8UpTim13,
    #[doc = "45 - TIM8 Trigger and Commutation interrupts and TIM14 global interrupt"]
    Tim8TrgComTim14,
    #[doc = "46 - TIM8 Capture Compare interrupt"] Tim8Cc,
    #[doc = "47 - DMA1 Stream7 global interrupt"] Dma1Stream7,
    #[doc = "48 - FMC global interrupt"] Fmc,
    #[doc = "49 - SDIO global interrupt"] Sdio,
    #[doc = "50 - TIM5 global interrupt"] Tim5,
    #[doc = "51 - SPI3 global interrupt"] Spi3,
    #[doc = "52 - UART4 global interrupt"] Uart4,
    #[doc = "53 - UART5 global interrupt"] Uart5,
    #[doc = "54 - TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt"] Tim6Dac,
    #[doc = "55 - TIM7 global interrupt"] Tim7,
    #[doc = "56 - DMA2 Stream0 global interrupt"] Dma2Stream0,
    #[doc = "57 - DMA2 Stream1 global interrupt"] Dma2Stream1,
    #[doc = "58 - DMA2 Stream2 global interrupt"] Dma2Stream2,
    #[doc = "59 - DMA2 Stream3 global interrupt"] Dma2Stream3,
    #[doc = "60 - DMA2 Stream4 global interrupt"] Dma2Stream4,
    #[doc = "61 - Ethernet global interrupt"] Eth,
    #[doc = "62 - Ethernet Wakeup through EXTI line interrupt"] EthWkup,
    #[doc = "63 - CAN2 TX interrupts"] Can2Tx,
    #[doc = "64 - CAN2 RX0 interrupts"] Can2Rx0,
    #[doc = "65 - CAN2 RX1 interrupts"] Can2Rx1,
    #[doc = "66 - CAN2 SCE interrupt"] Can2Sce,
    #[doc = "67 - USB On The Go FS global interrupt"] OtgFs,
    #[doc = "68 - DMA2 Stream5 global interrupt"] Dma2Stream5,
    #[doc = "69 - DMA2 Stream6 global interrupt"] Dma2Stream6,
    #[doc = "70 - DMA2 Stream7 global interrupt"] Dma2Stream7,
    #[doc = "71 - USART6 global interrupt"] Usart6,
    #[doc = "72 - I2C3 event interrupt"] I2c3Ev,
    #[doc = "73 - I2C3 error interrupt"] I2c3Er,
    #[doc = "74 - USB On The Go HS End Point 1 Out global interrupt"] OtgHsEp1Out,
    #[doc = "75 - USB On The Go HS End Point 1 In global interrupt"] OtgHsEp1In,
    #[doc = "76 - USB On The Go HS Wakeup through EXTI interrupt"] OtgHsWkup,
    #[doc = "77 - USB On The Go HS global interrupt"] OtgHs,
    #[doc = "78 - DCMI global interrupt"] Dcmi,
    #[doc = "80 - Rng global interrupt"] Rng,
    #[doc = "81 - FPU interrupt"] Fpu,
    #[doc = "82 - UART 7 global interrupt"] Uart7,
    #[doc = "83 - UART 8 global interrupt"] Uart8,
    #[doc = "84 - SPI 4 global interrupt"] Spi4,
    #[doc = "85 - SPI 5 global interrupt"] Spi5,
    #[doc = "86 - SPI 6 global interrupt"] Spi6,
    #[doc = "87 - SAI1 global interrupt"] Sai1,
    #[doc = "88 - LTDC global interrupt"] LcdTft,
    #[doc = "89 - LTDC global error interrupt"] LcdTft1,
    #[doc = "90 - DMA2D global interrupt"] Dma2d,
}
unsafe impl Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::Wwdg => 0,
            Interrupt::Pvd => 1,
            Interrupt::TampStamp => 2,
            Interrupt::RtcWkup => 3,
            Interrupt::Flash => 4,
            Interrupt::Rcc => 5,
            Interrupt::Exti0 => 6,
            Interrupt::Exti1 => 7,
            Interrupt::Exti2 => 8,
            Interrupt::Exti3 => 9,
            Interrupt::Exti4 => 10,
            Interrupt::Dma1Stream0 => 11,
            Interrupt::Dma1Stream1 => 12,
            Interrupt::Dma1Stream2 => 13,
            Interrupt::Dma1Stream3 => 14,
            Interrupt::Dma1Stream4 => 15,
            Interrupt::Dma1Stream5 => 16,
            Interrupt::Dma1Stream6 => 17,
            Interrupt::Adc => 18,
            Interrupt::Can1Tx => 19,
            Interrupt::Can1Rx0 => 20,
            Interrupt::Can1Rx1 => 21,
            Interrupt::Can1Sce => 22,
            Interrupt::Exti95 => 23,
            Interrupt::Tim1BrkTim9 => 24,
            Interrupt::Tim1UpTim10 => 25,
            Interrupt::Tim1TrgComTim11 => 26,
            Interrupt::Tim1Cc => 27,
            Interrupt::Tim2 => 28,
            Interrupt::Tim3 => 29,
            Interrupt::Tim4 => 30,
            Interrupt::I2c1Ev => 31,
            Interrupt::I2c1Er => 32,
            Interrupt::I2c2Ev => 33,
            Interrupt::I2c2Er => 34,
            Interrupt::Spi1 => 35,
            Interrupt::Spi2 => 36,
            Interrupt::Usart1 => 37,
            Interrupt::Usart2 => 38,
            Interrupt::Usart3 => 39,
            Interrupt::Exti1510 => 40,
            Interrupt::RtcAlarm => 41,
            Interrupt::OtgFsWkup => 42,
            Interrupt::Tim8BrkTim12 => 43,
            Interrupt::Tim8UpTim13 => 44,
            Interrupt::Tim8TrgComTim14 => 45,
            Interrupt::Tim8Cc => 46,
            Interrupt::Dma1Stream7 => 47,
            Interrupt::Fmc => 48,
            Interrupt::Sdio => 49,
            Interrupt::Tim5 => 50,
            Interrupt::Spi3 => 51,
            Interrupt::Uart4 => 52,
            Interrupt::Uart5 => 53,
            Interrupt::Tim6Dac => 54,
            Interrupt::Tim7 => 55,
            Interrupt::Dma2Stream0 => 56,
            Interrupt::Dma2Stream1 => 57,
            Interrupt::Dma2Stream2 => 58,
            Interrupt::Dma2Stream3 => 59,
            Interrupt::Dma2Stream4 => 60,
            Interrupt::Eth => 61,
            Interrupt::EthWkup => 62,
            Interrupt::Can2Tx => 63,
            Interrupt::Can2Rx0 => 64,
            Interrupt::Can2Rx1 => 65,
            Interrupt::Can2Sce => 66,
            Interrupt::OtgFs => 67,
            Interrupt::Dma2Stream5 => 68,
            Interrupt::Dma2Stream6 => 69,
            Interrupt::Dma2Stream7 => 70,
            Interrupt::Usart6 => 71,
            Interrupt::I2c3Ev => 72,
            Interrupt::I2c3Er => 73,
            Interrupt::OtgHsEp1Out => 74,
            Interrupt::OtgHsEp1In => 75,
            Interrupt::OtgHsWkup => 76,
            Interrupt::OtgHs => 77,
            Interrupt::Dcmi => 78,
            Interrupt::Rng => 80,
            Interrupt::Fpu => 81,
            Interrupt::Uart7 => 82,
            Interrupt::Uart8 => 83,
            Interrupt::Spi4 => 84,
            Interrupt::Spi5 => 85,
            Interrupt::Spi6 => 86,
            Interrupt::Sai1 => 87,
            Interrupt::LcdTft => 88,
            Interrupt::LcdTft1 => 89,
            Interrupt::Dma2d => 90,
        }
    }
}
