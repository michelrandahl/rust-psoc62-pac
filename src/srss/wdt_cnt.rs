#[doc = "Register `WDT_CNT` reader"]
pub type R = crate::R<WDT_CNT_SPEC>;
#[doc = "Register `WDT_CNT` writer"]
pub type W = crate::W<WDT_CNT_SPEC>;
#[doc = "Field `COUNTER` reader - Current value of WDT Counter. The write feature of this register is for engineering use (DfV), have no synchronization, and can only be applied when the WDT is fully off. When writing, the value is updated immediately in the WDT counter, but it will read back as the old value until this register resynchronizes just after the negedge of ILO. Writes will be ignored if they occur when the WDT is enabled."]
pub type COUNTER_R = crate::FieldReader<u16>;
#[doc = "Field `COUNTER` writer - Current value of WDT Counter. The write feature of this register is for engineering use (DfV), have no synchronization, and can only be applied when the WDT is fully off. When writing, the value is updated immediately in the WDT counter, but it will read back as the old value until this register resynchronizes just after the negedge of ILO. Writes will be ignored if they occur when the WDT is enabled."]
pub type COUNTER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current value of WDT Counter. The write feature of this register is for engineering use (DfV), have no synchronization, and can only be applied when the WDT is fully off. When writing, the value is updated immediately in the WDT counter, but it will read back as the old value until this register resynchronizes just after the negedge of ILO. Writes will be ignored if they occur when the WDT is enabled."]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current value of WDT Counter. The write feature of this register is for engineering use (DfV), have no synchronization, and can only be applied when the WDT is fully off. When writing, the value is updated immediately in the WDT counter, but it will read back as the old value until this register resynchronizes just after the negedge of ILO. Writes will be ignored if they occur when the WDT is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> COUNTER_W<WDT_CNT_SPEC> {
        COUNTER_W::new(self, 0)
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
#[doc = "Watchdog Counter Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDT_CNT_SPEC;
impl crate::RegisterSpec for WDT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_cnt::R`](R) reader structure"]
impl crate::Readable for WDT_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdt_cnt::W`](W) writer structure"]
impl crate::Writable for WDT_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDT_CNT to value 0"]
impl crate::Resettable for WDT_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
