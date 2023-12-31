#[doc = "Register `MCWDT_CNTHIGH` reader"]
pub type R = crate::R<MCWDT_CNTHIGH_SPEC>;
#[doc = "Register `MCWDT_CNTHIGH` writer"]
pub type W = crate::W<MCWDT_CNTHIGH_SPEC>;
#[doc = "Field `WDT_CTR2` reader - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
pub type WDT_CTR2_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_CTR2` writer - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
pub type WDT_CTR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn wdt_ctr2(&self) -> WDT_CTR2_R {
        WDT_CTR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ctr2(&mut self) -> WDT_CTR2_W<MCWDT_CNTHIGH_SPEC> {
        WDT_CTR2_W::new(self, 0)
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
#[doc = "Multi-Counter Watchdog Sub-counter 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_cnthigh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_cnthigh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCWDT_CNTHIGH_SPEC;
impl crate::RegisterSpec for MCWDT_CNTHIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_cnthigh::R`](R) reader structure"]
impl crate::Readable for MCWDT_CNTHIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_cnthigh::W`](W) writer structure"]
impl crate::Writable for MCWDT_CNTHIGH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCWDT_CNTHIGH to value 0"]
impl crate::Resettable for MCWDT_CNTHIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
