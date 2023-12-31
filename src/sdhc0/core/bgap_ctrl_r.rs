#[doc = "Register `BGAP_CTRL_R` reader"]
pub type R = crate::R<BGAP_CTRL_R_SPEC>;
#[doc = "Register `BGAP_CTRL_R` writer"]
pub type W = crate::W<BGAP_CTRL_R_SPEC>;
#[doc = "Field `STOP_BG_REQ` reader - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: - 0x0 (XFER): Transfer - 0x1 (STOP): Stop"]
pub type STOP_BG_REQ_R = crate::BitReader;
#[doc = "Field `STOP_BG_REQ` writer - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: - 0x0 (XFER): Transfer - 0x1 (STOP): Stop"]
pub type STOP_BG_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTINUE_REQ` reader - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: - 0x0 (NO_AFFECT): No Affect - 0x1 (RESTART): Restart"]
pub type CONTINUE_REQ_R = crate::BitReader;
#[doc = "Field `CONTINUE_REQ` writer - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: - 0x0 (NO_AFFECT): No Affect - 0x1 (RESTART): Restart"]
pub type CONTINUE_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_WAIT_CTRL` reader - N/A"]
pub type RD_WAIT_CTRL_R = crate::BitReader;
#[doc = "Field `RD_WAIT_CTRL` writer - N/A"]
pub type RD_WAIT_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_AT_BGAP` reader - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: - 0x0 (DISABLE): Disabled - 0x1 (ENABLE): Enabled"]
pub type INT_AT_BGAP_R = crate::BitReader;
#[doc = "Field `INT_AT_BGAP` writer - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: - 0x0 (DISABLE): Disabled - 0x1 (ENABLE): Enabled"]
pub type INT_AT_BGAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: - 0x0 (XFER): Transfer - 0x1 (STOP): Stop"]
    #[inline(always)]
    pub fn stop_bg_req(&self) -> STOP_BG_REQ_R {
        STOP_BG_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: - 0x0 (NO_AFFECT): No Affect - 0x1 (RESTART): Restart"]
    #[inline(always)]
    pub fn continue_req(&self) -> CONTINUE_REQ_R {
        CONTINUE_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rd_wait_ctrl(&self) -> RD_WAIT_CTRL_R {
        RD_WAIT_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: - 0x0 (DISABLE): Disabled - 0x1 (ENABLE): Enabled"]
    #[inline(always)]
    pub fn int_at_bgap(&self) -> INT_AT_BGAP_R {
        INT_AT_BGAP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: - 0x0 (XFER): Transfer - 0x1 (STOP): Stop"]
    #[inline(always)]
    #[must_use]
    pub fn stop_bg_req(&mut self) -> STOP_BG_REQ_W<BGAP_CTRL_R_SPEC> {
        STOP_BG_REQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: - 0x0 (NO_AFFECT): No Affect - 0x1 (RESTART): Restart"]
    #[inline(always)]
    #[must_use]
    pub fn continue_req(&mut self) -> CONTINUE_REQ_W<BGAP_CTRL_R_SPEC> {
        CONTINUE_REQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rd_wait_ctrl(&mut self) -> RD_WAIT_CTRL_W<BGAP_CTRL_R_SPEC> {
        RD_WAIT_CTRL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: - 0x0 (DISABLE): Disabled - 0x1 (ENABLE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn int_at_bgap(&mut self) -> INT_AT_BGAP_W<BGAP_CTRL_R_SPEC> {
        INT_AT_BGAP_W::new(self, 3)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Block Gap Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgap_ctrl_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgap_ctrl_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BGAP_CTRL_R_SPEC;
impl crate::RegisterSpec for BGAP_CTRL_R_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bgap_ctrl_r::R`](R) reader structure"]
impl crate::Readable for BGAP_CTRL_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bgap_ctrl_r::W`](W) writer structure"]
impl crate::Writable for BGAP_CTRL_R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BGAP_CTRL_R to value 0"]
impl crate::Resettable for BGAP_CTRL_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
