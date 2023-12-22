#[doc = "Register `CTRL_CLR` reader"]
pub type R = crate::R<CTRL_CLR_SPEC>;
#[doc = "Register `CTRL_CLR` writer"]
pub type W = crate::W<CTRL_CLR_SPEC>;
#[doc = "Field `COUNTER_ENABLED` reader - Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
pub type COUNTER_ENABLED_R = crate::FieldReader<u32>;
#[doc = "Field `COUNTER_ENABLED` writer - Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
pub type COUNTER_ENABLED_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    pub fn counter_enabled(&self) -> COUNTER_ENABLED_R {
        COUNTER_ENABLED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    #[must_use]
    pub fn counter_enabled(&mut self) -> COUNTER_ENABLED_W<CTRL_CLR_SPEC> {
        COUNTER_ENABLED_W::new(self, 0)
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
#[doc = "TCPWM control clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_CLR_SPEC;
impl crate::RegisterSpec for CTRL_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_clr::R`](R) reader structure"]
impl crate::Readable for CTRL_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_clr::W`](W) writer structure"]
impl crate::Writable for CTRL_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_CLR to value 0"]
impl crate::Resettable for CTRL_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
