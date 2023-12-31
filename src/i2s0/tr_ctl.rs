#[doc = "Register `TR_CTL` reader"]
pub type R = crate::R<TR_CTL_SPEC>;
#[doc = "Register `TR_CTL` writer"]
pub type W = crate::W<TR_CTL_SPEC>;
#[doc = "Field `TX_REQ_EN` reader - Trigger output ('tr_i2s_tx_req') enable for requests of DMA transfer in transmission '0': Disabled. '1': Enabled."]
pub type TX_REQ_EN_R = crate::BitReader;
#[doc = "Field `TX_REQ_EN` writer - Trigger output ('tr_i2s_tx_req') enable for requests of DMA transfer in transmission '0': Disabled. '1': Enabled."]
pub type TX_REQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_REQ_EN` reader - Trigger output ('tr_i2s_rx_req') enable for requests of DMA transfer in reception '0': Disabled. '1': Enabled."]
pub type RX_REQ_EN_R = crate::BitReader;
#[doc = "Field `RX_REQ_EN` writer - Trigger output ('tr_i2s_rx_req') enable for requests of DMA transfer in reception '0': Disabled. '1': Enabled."]
pub type RX_REQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Trigger output ('tr_i2s_tx_req') enable for requests of DMA transfer in transmission '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_req_en(&self) -> TX_REQ_EN_R {
        TX_REQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Trigger output ('tr_i2s_rx_req') enable for requests of DMA transfer in reception '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_req_en(&self) -> RX_REQ_EN_R {
        RX_REQ_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger output ('tr_i2s_tx_req') enable for requests of DMA transfer in transmission '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tx_req_en(&mut self) -> TX_REQ_EN_W<TR_CTL_SPEC> {
        TX_REQ_EN_W::new(self, 0)
    }
    #[doc = "Bit 16 - Trigger output ('tr_i2s_rx_req') enable for requests of DMA transfer in reception '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rx_req_en(&mut self) -> RX_REQ_EN_W<TR_CTL_SPEC> {
        RX_REQ_EN_W::new(self, 16)
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
#[doc = "Trigger control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR_CTL_SPEC;
impl crate::RegisterSpec for TR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_ctl::R`](R) reader structure"]
impl crate::Readable for TR_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tr_ctl::W`](W) writer structure"]
impl crate::Writable for TR_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_CTL to value 0"]
impl crate::Resettable for TR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
