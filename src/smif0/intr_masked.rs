#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<INTR_MASKED_SPEC>;
#[doc = "Field `TR_TX_REQ` reader - Logical and of corresponding request and mask bits."]
pub type TR_TX_REQ_R = crate::BitReader;
#[doc = "Field `TR_RX_REQ` reader - Logical and of corresponding request and mask bits."]
pub type TR_RX_REQ_R = crate::BitReader;
#[doc = "Field `XIP_ALIGNMENT_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type XIP_ALIGNMENT_ERROR_R = crate::BitReader;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type TX_CMD_FIFO_OVERFLOW_R = crate::BitReader;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type TX_DATA_FIFO_OVERFLOW_R = crate::BitReader;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type RX_DATA_FIFO_UNDERFLOW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TR_TX_REQ_R {
        TR_TX_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TR_RX_REQ_R {
        TR_RX_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XIP_ALIGNMENT_ERROR_R {
        XIP_ALIGNMENT_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TX_CMD_FIFO_OVERFLOW_R {
        TX_CMD_FIFO_OVERFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TX_DATA_FIFO_OVERFLOW_R {
        TX_DATA_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&self) -> RX_DATA_FIFO_UNDERFLOW_R {
        RX_DATA_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt masked register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_masked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_masked::R`](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
