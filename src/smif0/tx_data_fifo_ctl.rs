#[doc = "Register `TX_DATA_FIFO_CTL` reader"]
pub type R = crate::R<TX_DATA_FIFO_CTL_SPEC>;
#[doc = "Register `TX_DATA_FIFO_CTL` writer"]
pub type W = crate::W<TX_DATA_FIFO_CTL_SPEC>;
#[doc = "Field `TRIGGER_LEVEL` reader - Determines when the TX data FIFO 'tr_tx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when TX_DATA_FIFO_STATUS.USED &lt;= TRIGGER_LEVEL."]
pub type TRIGGER_LEVEL_R = crate::FieldReader;
#[doc = "Field `TRIGGER_LEVEL` writer - Determines when the TX data FIFO 'tr_tx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when TX_DATA_FIFO_STATUS.USED &lt;= TRIGGER_LEVEL."]
pub type TRIGGER_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Determines when the TX data FIFO 'tr_tx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when TX_DATA_FIFO_STATUS.USED &lt;= TRIGGER_LEVEL."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Determines when the TX data FIFO 'tr_tx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when TX_DATA_FIFO_STATUS.USED &lt;= TRIGGER_LEVEL."]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W<TX_DATA_FIFO_CTL_SPEC> {
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
#[doc = "Transmitter data FIFO control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_data_fifo_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data_fifo_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_DATA_FIFO_CTL_SPEC;
impl crate::RegisterSpec for TX_DATA_FIFO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_data_fifo_ctl::R`](R) reader structure"]
impl crate::Readable for TX_DATA_FIFO_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_data_fifo_ctl::W`](W) writer structure"]
impl crate::Writable for TX_DATA_FIFO_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_DATA_FIFO_CTL to value 0"]
impl crate::Resettable for TX_DATA_FIFO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
