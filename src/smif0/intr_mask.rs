#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<INTR_MASK_SPEC>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<INTR_MASK_SPEC>;
#[doc = "Field `TR_TX_REQ` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TR_TX_REQ_R = crate::BitReader;
#[doc = "Field `TR_TX_REQ` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TR_TX_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_RX_REQ` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TR_RX_REQ_R = crate::BitReader;
#[doc = "Field `TR_RX_REQ` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TR_RX_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIP_ALIGNMENT_ERROR` reader - Mask bit for corresponding bit in interrupt request register."]
pub type XIP_ALIGNMENT_ERROR_R = crate::BitReader;
#[doc = "Field `XIP_ALIGNMENT_ERROR` writer - Mask bit for corresponding bit in interrupt request register."]
pub type XIP_ALIGNMENT_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TX_CMD_FIFO_OVERFLOW_R = crate::BitReader;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TX_CMD_FIFO_OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TX_DATA_FIFO_OVERFLOW_R = crate::BitReader;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TX_DATA_FIFO_OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type RX_DATA_FIFO_UNDERFLOW_R = crate::BitReader;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type RX_DATA_FIFO_UNDERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TR_TX_REQ_R {
        TR_TX_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TR_RX_REQ_R {
        TR_RX_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XIP_ALIGNMENT_ERROR_R {
        XIP_ALIGNMENT_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TX_CMD_FIFO_OVERFLOW_R {
        TX_CMD_FIFO_OVERFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TX_DATA_FIFO_OVERFLOW_R {
        TX_DATA_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&self) -> RX_DATA_FIFO_UNDERFLOW_R {
        RX_DATA_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tr_tx_req(&mut self) -> TR_TX_REQ_W<INTR_MASK_SPEC> {
        TR_TX_REQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tr_rx_req(&mut self) -> TR_RX_REQ_W<INTR_MASK_SPEC> {
        TR_RX_REQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn xip_alignment_error(&mut self) -> XIP_ALIGNMENT_ERROR_W<INTR_MASK_SPEC> {
        XIP_ALIGNMENT_ERROR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_cmd_fifo_overflow(&mut self) -> TX_CMD_FIFO_OVERFLOW_W<INTR_MASK_SPEC> {
        TX_CMD_FIFO_OVERFLOW_W::new(self, 3)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_fifo_overflow(&mut self) -> TX_DATA_FIFO_OVERFLOW_W<INTR_MASK_SPEC> {
        TX_DATA_FIFO_OVERFLOW_W::new(self, 4)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_fifo_underflow(&mut self) -> RX_DATA_FIFO_UNDERFLOW_W<INTR_MASK_SPEC> {
        RX_DATA_FIFO_UNDERFLOW_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
