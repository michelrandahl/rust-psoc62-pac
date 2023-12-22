#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `STREAM_EN` reader - Enable data streaming flow: '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.PDMA_EN)"]
pub type STREAM_EN_R = crate::BitReader;
#[doc = "Field `STREAM_EN` writer - Enable data streaming flow: '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.PDMA_EN)"]
pub type STREAM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable data streaming flow: '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.PDMA_EN)"]
    #[inline(always)]
    pub fn stream_en(&self) -> STREAM_EN_R {
        STREAM_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable data streaming flow: '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.PDMA_EN)"]
    #[inline(always)]
    #[must_use]
    pub fn stream_en(&mut self) -> STREAM_EN_W<CMD_SPEC> {
        STREAM_EN_W::new(self, 0)
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
#[doc = "Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
