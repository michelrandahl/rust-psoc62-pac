#[doc = "Register `MCWDT_CNTLOW` reader"]
pub type R = crate::R<MCWDT_CNTLOW_SPEC>;
#[doc = "Register `MCWDT_CNTLOW` writer"]
pub type W = crate::W<MCWDT_CNTLOW_SPEC>;
#[doc = "Field `WDT_CTR0` reader - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
pub type WDT_CTR0_R = crate::FieldReader<u16>;
#[doc = "Field `WDT_CTR0` writer - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
pub type WDT_CTR0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WDT_CTR1` reader - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
pub type WDT_CTR1_R = crate::FieldReader<u16>;
#[doc = "Field `WDT_CTR1` writer - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
pub type WDT_CTR1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
    #[inline(always)]
    pub fn wdt_ctr0(&self) -> WDT_CTR0_R {
        WDT_CTR0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn wdt_ctr1(&self) -> WDT_CTR1_R {
        WDT_CTR1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ctr0(&mut self) -> WDT_CTR0_W<MCWDT_CNTLOW_SPEC> {
        WDT_CTR0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ctr1(&mut self) -> WDT_CTR1_W<MCWDT_CNTLOW_SPEC> {
        WDT_CTR1_W::new(self, 16)
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
#[doc = "Multi-Counter Watchdog Sub-counters 0/1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_cntlow::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_cntlow::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCWDT_CNTLOW_SPEC;
impl crate::RegisterSpec for MCWDT_CNTLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_cntlow::R`](R) reader structure"]
impl crate::Readable for MCWDT_CNTLOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_cntlow::W`](W) writer structure"]
impl crate::Writable for MCWDT_CNTLOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCWDT_CNTLOW to value 0"]
impl crate::Resettable for MCWDT_CNTLOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
