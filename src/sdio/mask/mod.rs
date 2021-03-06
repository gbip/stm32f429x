#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Mask {
    #[doc = r" Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = r" Value of the field"]
pub struct CeataendieR {
    bits: u8,
}
impl CeataendieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SdioitieR {
    bits: u8,
}
impl SdioitieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RxdavlieR {
    bits: u8,
}
impl RxdavlieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TxdavlieR {
    bits: u8,
}
impl TxdavlieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RxfifoeieR {
    bits: u8,
}
impl RxfifoeieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TxfifoeieR {
    bits: u8,
}
impl TxfifoeieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RxfifofieR {
    bits: u8,
}
impl RxfifofieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TxfifofieR {
    bits: u8,
}
impl TxfifofieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RxfifohfieR {
    bits: u8,
}
impl RxfifohfieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TxfifoheieR {
    bits: u8,
}
impl TxfifoheieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RxactieR {
    bits: u8,
}
impl RxactieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TxactieR {
    bits: u8,
}
impl TxactieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CmdactieR {
    bits: u8,
}
impl CmdactieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DbckendieR {
    bits: u8,
}
impl DbckendieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct StbiterrieR {
    bits: u8,
}
impl StbiterrieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DataendieR {
    bits: u8,
}
impl DataendieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CmdsentieR {
    bits: u8,
}
impl CmdsentieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CmdrendieR {
    bits: u8,
}
impl CmdrendieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RxoverrieR {
    bits: u8,
}
impl RxoverrieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TxunderrieR {
    bits: u8,
}
impl TxunderrieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DtimeoutieR {
    bits: u8,
}
impl DtimeoutieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CtimeoutieR {
    bits: u8,
}
impl CtimeoutieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DcrcfailieR {
    bits: u8,
}
impl DcrcfailieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CcrcfailieR {
    bits: u8,
}
impl CcrcfailieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CeataendieW<'a> {
    w: &'a mut W,
}
impl<'a> _CeataendieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SdioitieW<'a> {
    w: &'a mut W,
}
impl<'a> _SdioitieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RxdavlieW<'a> {
    w: &'a mut W,
}
impl<'a> _RxdavlieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TxdavlieW<'a> {
    w: &'a mut W,
}
impl<'a> _TxdavlieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RxfifoeieW<'a> {
    w: &'a mut W,
}
impl<'a> _RxfifoeieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TxfifoeieW<'a> {
    w: &'a mut W,
}
impl<'a> _TxfifoeieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RxfifofieW<'a> {
    w: &'a mut W,
}
impl<'a> _RxfifofieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TxfifofieW<'a> {
    w: &'a mut W,
}
impl<'a> _TxfifofieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RxfifohfieW<'a> {
    w: &'a mut W,
}
impl<'a> _RxfifohfieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TxfifoheieW<'a> {
    w: &'a mut W,
}
impl<'a> _TxfifoheieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RxactieW<'a> {
    w: &'a mut W,
}
impl<'a> _RxactieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TxactieW<'a> {
    w: &'a mut W,
}
impl<'a> _TxactieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CmdactieW<'a> {
    w: &'a mut W,
}
impl<'a> _CmdactieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DbckendieW<'a> {
    w: &'a mut W,
}
impl<'a> _DbckendieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _StbiterrieW<'a> {
    w: &'a mut W,
}
impl<'a> _StbiterrieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DataendieW<'a> {
    w: &'a mut W,
}
impl<'a> _DataendieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CmdsentieW<'a> {
    w: &'a mut W,
}
impl<'a> _CmdsentieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CmdrendieW<'a> {
    w: &'a mut W,
}
impl<'a> _CmdrendieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RxoverrieW<'a> {
    w: &'a mut W,
}
impl<'a> _RxoverrieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TxunderrieW<'a> {
    w: &'a mut W,
}
impl<'a> _TxunderrieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DtimeoutieW<'a> {
    w: &'a mut W,
}
impl<'a> _DtimeoutieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CtimeoutieW<'a> {
    w: &'a mut W,
}
impl<'a> _CtimeoutieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DcrcfailieW<'a> {
    w: &'a mut W,
}
impl<'a> _DcrcfailieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CcrcfailieW<'a> {
    w: &'a mut W,
}
impl<'a> _CcrcfailieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received interrupt enable"]
    #[inline(always)]
    pub fn ceataendie(&self) -> CeataendieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CeataendieR { bits }
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable"]
    #[inline(always)]
    pub fn sdioitie(&self) -> SdioitieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SdioitieR { bits }
    }
    #[doc = "Bit 21 - Data available in Rx FIFO interrupt enable"]
    #[inline(always)]
    pub fn rxdavlie(&self) -> RxdavlieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RxdavlieR { bits }
    }
    #[doc = "Bit 20 - Data available in Tx FIFO interrupt enable"]
    #[inline(always)]
    pub fn txdavlie(&self) -> TxdavlieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TxdavlieR { bits }
    }
    #[doc = "Bit 19 - Rx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn rxfifoeie(&self) -> RxfifoeieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RxfifoeieR { bits }
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TxfifoeieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TxfifoeieR { bits }
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RxfifofieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RxfifofieR { bits }
    }
    #[doc = "Bit 16 - Tx FIFO full interrupt enable"]
    #[inline(always)]
    pub fn txfifofie(&self) -> TxfifofieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TxfifofieR { bits }
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable"]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RxfifohfieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RxfifohfieR { bits }
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TxfifoheieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TxfifoheieR { bits }
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable"]
    #[inline(always)]
    pub fn rxactie(&self) -> RxactieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RxactieR { bits }
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable"]
    #[inline(always)]
    pub fn txactie(&self) -> TxactieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TxactieR { bits }
    }
    #[doc = "Bit 11 - Command acting interrupt enable"]
    #[inline(always)]
    pub fn cmdactie(&self) -> CmdactieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CmdactieR { bits }
    }
    #[doc = "Bit 10 - Data block end interrupt enable"]
    #[inline(always)]
    pub fn dbckendie(&self) -> DbckendieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DbckendieR { bits }
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    pub fn stbiterrie(&self) -> StbiterrieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        StbiterrieR { bits }
    }
    #[doc = "Bit 8 - Data end interrupt enable"]
    #[inline(always)]
    pub fn dataendie(&self) -> DataendieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DataendieR { bits }
    }
    #[doc = "Bit 7 - Command sent interrupt enable"]
    #[inline(always)]
    pub fn cmdsentie(&self) -> CmdsentieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CmdsentieR { bits }
    }
    #[doc = "Bit 6 - Command response received interrupt enable"]
    #[inline(always)]
    pub fn cmdrendie(&self) -> CmdrendieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CmdrendieR { bits }
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxoverrie(&self) -> RxoverrieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RxoverrieR { bits }
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable"]
    #[inline(always)]
    pub fn txunderrie(&self) -> TxunderrieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TxunderrieR { bits }
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DtimeoutieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DtimeoutieR { bits }
    }
    #[doc = "Bit 2 - Command timeout interrupt enable"]
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CtimeoutieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CtimeoutieR { bits }
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable"]
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DcrcfailieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DcrcfailieR { bits }
    }
    #[doc = "Bit 0 - Command CRC fail interrupt enable"]
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CcrcfailieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CcrcfailieR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received interrupt enable"]
    #[inline(always)]
    pub fn ceataendie(&mut self) -> _CeataendieW {
        _CeataendieW { w: self }
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable"]
    #[inline(always)]
    pub fn sdioitie(&mut self) -> _SdioitieW {
        _SdioitieW { w: self }
    }
    #[doc = "Bit 21 - Data available in Rx FIFO interrupt enable"]
    #[inline(always)]
    pub fn rxdavlie(&mut self) -> _RxdavlieW {
        _RxdavlieW { w: self }
    }
    #[doc = "Bit 20 - Data available in Tx FIFO interrupt enable"]
    #[inline(always)]
    pub fn txdavlie(&mut self) -> _TxdavlieW {
        _TxdavlieW { w: self }
    }
    #[doc = "Bit 19 - Rx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn rxfifoeie(&mut self) -> _RxfifoeieW {
        _RxfifoeieW { w: self }
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoeie(&mut self) -> _TxfifoeieW {
        _TxfifoeieW { w: self }
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rxfifofie(&mut self) -> _RxfifofieW {
        _RxfifofieW { w: self }
    }
    #[doc = "Bit 16 - Tx FIFO full interrupt enable"]
    #[inline(always)]
    pub fn txfifofie(&mut self) -> _TxfifofieW {
        _TxfifofieW { w: self }
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable"]
    #[inline(always)]
    pub fn rxfifohfie(&mut self) -> _RxfifohfieW {
        _RxfifohfieW { w: self }
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoheie(&mut self) -> _TxfifoheieW {
        _TxfifoheieW { w: self }
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable"]
    #[inline(always)]
    pub fn rxactie(&mut self) -> _RxactieW {
        _RxactieW { w: self }
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable"]
    #[inline(always)]
    pub fn txactie(&mut self) -> _TxactieW {
        _TxactieW { w: self }
    }
    #[doc = "Bit 11 - Command acting interrupt enable"]
    #[inline(always)]
    pub fn cmdactie(&mut self) -> _CmdactieW {
        _CmdactieW { w: self }
    }
    #[doc = "Bit 10 - Data block end interrupt enable"]
    #[inline(always)]
    pub fn dbckendie(&mut self) -> _DbckendieW {
        _DbckendieW { w: self }
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    pub fn stbiterrie(&mut self) -> _StbiterrieW {
        _StbiterrieW { w: self }
    }
    #[doc = "Bit 8 - Data end interrupt enable"]
    #[inline(always)]
    pub fn dataendie(&mut self) -> _DataendieW {
        _DataendieW { w: self }
    }
    #[doc = "Bit 7 - Command sent interrupt enable"]
    #[inline(always)]
    pub fn cmdsentie(&mut self) -> _CmdsentieW {
        _CmdsentieW { w: self }
    }
    #[doc = "Bit 6 - Command response received interrupt enable"]
    #[inline(always)]
    pub fn cmdrendie(&mut self) -> _CmdrendieW {
        _CmdrendieW { w: self }
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxoverrie(&mut self) -> _RxoverrieW {
        _RxoverrieW { w: self }
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable"]
    #[inline(always)]
    pub fn txunderrie(&mut self) -> _TxunderrieW {
        _TxunderrieW { w: self }
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dtimeoutie(&mut self) -> _DtimeoutieW {
        _DtimeoutieW { w: self }
    }
    #[doc = "Bit 2 - Command timeout interrupt enable"]
    #[inline(always)]
    pub fn ctimeoutie(&mut self) -> _CtimeoutieW {
        _CtimeoutieW { w: self }
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable"]
    #[inline(always)]
    pub fn dcrcfailie(&mut self) -> _DcrcfailieW {
        _DcrcfailieW { w: self }
    }
    #[doc = "Bit 0 - Command CRC fail interrupt enable"]
    #[inline(always)]
    pub fn ccrcfailie(&mut self) -> _CcrcfailieW {
        _CcrcfailieW { w: self }
    }
}
