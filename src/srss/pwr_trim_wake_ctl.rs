#[doc = "Register `PWR_TRIM_WAKE_CTL` reader"]
pub type R = crate::R<PWR_TRIM_WAKE_CTL_SPEC>;
#[doc = "Register `PWR_TRIM_WAKE_CTL` writer"]
pub type W = crate::W<PWR_TRIM_WAKE_CTL_SPEC>;
#[doc = "Field `WAKE_DELAY` reader - Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
pub type WAKE_DELAY_R = crate::FieldReader;
#[doc = "Field `WAKE_DELAY` writer - Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
pub type WAKE_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
    #[inline(always)]
    pub fn wake_delay(&self) -> WAKE_DELAY_R {
        WAKE_DELAY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
    #[inline(always)]
    #[must_use]
    pub fn wake_delay(&mut self) -> WAKE_DELAY_W<PWR_TRIM_WAKE_CTL_SPEC> {
        WAKE_DELAY_W::new(self, 0)
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
#[doc = "Wakeup Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_trim_wake_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_trim_wake_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_TRIM_WAKE_CTL_SPEC;
impl crate::RegisterSpec for PWR_TRIM_WAKE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_trim_wake_ctl::R`](R) reader structure"]
impl crate::Readable for PWR_TRIM_WAKE_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwr_trim_wake_ctl::W`](W) writer structure"]
impl crate::Writable for PWR_TRIM_WAKE_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_TRIM_WAKE_CTL to value 0"]
impl crate::Resettable for PWR_TRIM_WAKE_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
