#[doc = "Register `TX_FIFO_STATUS` reader"]
pub type R = crate::R<TX_FIFO_STATUS_SPEC>;
#[doc = "Field `USED` reader - N/A"]
pub type USED_R = crate::FieldReader<u16>;
#[doc = "Field `SR_VALID` reader - N/A"]
pub type SR_VALID_R = crate::BitReader;
#[doc = "Field `RD_PTR` reader - N/A"]
pub type RD_PTR_R = crate::FieldReader;
#[doc = "Field `WR_PTR` reader - N/A"]
pub type WR_PTR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:8 - N/A"]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn sr_valid(&self) -> SR_VALID_R {
        SR_VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - N/A"]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RD_PTR_R {
        RD_PTR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - N/A"]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WR_PTR_R {
        WR_PTR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Transmitter FIFO status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_fifo_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for TX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_fifo_status::R`](R) reader structure"]
impl crate::Readable for TX_FIFO_STATUS_SPEC {}
#[doc = "`reset()` method sets TX_FIFO_STATUS to value 0"]
impl crate::Resettable for TX_FIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
