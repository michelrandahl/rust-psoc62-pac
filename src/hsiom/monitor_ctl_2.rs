#[doc = "Register `MONITOR_CTL_2` reader"]
pub type R = crate::R<MONITOR_CTL_2_SPEC>;
#[doc = "Register `MONITOR_CTL_2` writer"]
pub type W = crate::W<MONITOR_CTL_2_SPEC>;
#[doc = "Field `MONITOR_EN` reader - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
pub type MONITOR_EN_R = crate::FieldReader<u32>;
#[doc = "Field `MONITOR_EN` writer - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
pub type MONITOR_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn monitor_en(&self) -> MONITOR_EN_R {
        MONITOR_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
    #[inline(always)]
    #[must_use]
    pub fn monitor_en(&mut self) -> MONITOR_EN_W<MONITOR_CTL_2_SPEC> {
        MONITOR_EN_W::new(self, 0)
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
#[doc = "Power/Ground Monitor cell control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`monitor_ctl_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`monitor_ctl_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MONITOR_CTL_2_SPEC;
impl crate::RegisterSpec for MONITOR_CTL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`monitor_ctl_2::R`](R) reader structure"]
impl crate::Readable for MONITOR_CTL_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`monitor_ctl_2::W`](W) writer structure"]
impl crate::Writable for MONITOR_CTL_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MONITOR_CTL_2 to value 0"]
impl crate::Resettable for MONITOR_CTL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
