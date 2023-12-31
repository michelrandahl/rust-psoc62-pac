#[doc = "Register `ACLK_CTL` writer"]
pub type W = crate::W<ACLK_CTL_SPEC>;
#[doc = "Field `ACLK_GEN` writer - A write to this register generates the clock pulse for HV control registers (mpcon outputs)"]
pub type ACLK_GEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - A write to this register generates the clock pulse for HV control registers (mpcon outputs)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gen(&mut self) -> ACLK_GEN_W<ACLK_CTL_SPEC> {
        ACLK_GEN_W::new(self, 0)
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
#[doc = "MPCON clock\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aclk_ctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACLK_CTL_SPEC;
impl crate::RegisterSpec for ACLK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aclk_ctl::W`](W) writer structure"]
impl crate::Writable for ACLK_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACLK_CTL to value 0"]
impl crate::Resettable for ACLK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
