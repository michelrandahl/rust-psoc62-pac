#[doc = "Register `RX_DATA_FIFO_CTL` reader"]
pub type R = crate::R<RX_DATA_FIFO_CTL_SPEC>;
#[doc = "Register `RX_DATA_FIFO_CTL` writer"]
pub type W = crate::W<RX_DATA_FIFO_CTL_SPEC>;
#[doc = "Field `TRIGGER_LEVEL` reader - Determines when RX data FIFO 'tr_rx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when RX_DATA_FIFO_STATUS.USED > TRIGGER_LEVEL."]
pub type TRIGGER_LEVEL_R = crate::FieldReader;
#[doc = "Field `TRIGGER_LEVEL` writer - Determines when RX data FIFO 'tr_rx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when RX_DATA_FIFO_STATUS.USED > TRIGGER_LEVEL."]
pub type TRIGGER_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Determines when RX data FIFO 'tr_rx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when RX_DATA_FIFO_STATUS.USED > TRIGGER_LEVEL."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Determines when RX data FIFO 'tr_rx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when RX_DATA_FIFO_STATUS.USED > TRIGGER_LEVEL."]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W<RX_DATA_FIFO_CTL_SPEC> {
        TRIGGER_LEVEL_W::new(self, 0)
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
#[doc = "Receiver data FIFO control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_fifo_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_data_fifo_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DATA_FIFO_CTL_SPEC;
impl crate::RegisterSpec for RX_DATA_FIFO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data_fifo_ctl::R`](R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_data_fifo_ctl::W`](W) writer structure"]
impl crate::Writable for RX_DATA_FIFO_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_DATA_FIFO_CTL to value 0"]
impl crate::Resettable for RX_DATA_FIFO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
