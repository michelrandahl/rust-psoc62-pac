#[doc = "Register `MCWDT_MATCH` reader"]
pub type R = crate::R<MCWDT_MATCH_SPEC>;
#[doc = "Register `MCWDT_MATCH` writer"]
pub type W = crate::W<MCWDT_MATCH_SPEC>;
#[doc = "Field `WDT_MATCH0` reader - Match value for sub-counter 0 of this MCWDT"]
pub type WDT_MATCH0_R = crate::FieldReader<u16>;
#[doc = "Field `WDT_MATCH0` writer - Match value for sub-counter 0 of this MCWDT"]
pub type WDT_MATCH0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WDT_MATCH1` reader - Match value for sub-counter 1 of this MCWDT"]
pub type WDT_MATCH1_R = crate::FieldReader<u16>;
#[doc = "Field `WDT_MATCH1` writer - Match value for sub-counter 1 of this MCWDT"]
pub type WDT_MATCH1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Match value for sub-counter 0 of this MCWDT"]
    #[inline(always)]
    pub fn wdt_match0(&self) -> WDT_MATCH0_R {
        WDT_MATCH0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Match value for sub-counter 1 of this MCWDT"]
    #[inline(always)]
    pub fn wdt_match1(&self) -> WDT_MATCH1_R {
        WDT_MATCH1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Match value for sub-counter 0 of this MCWDT"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_match0(&mut self) -> WDT_MATCH0_W<MCWDT_MATCH_SPEC> {
        WDT_MATCH0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Match value for sub-counter 1 of this MCWDT"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_match1(&mut self) -> WDT_MATCH1_W<MCWDT_MATCH_SPEC> {
        WDT_MATCH1_W::new(self, 16)
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
#[doc = "Multi-Counter Watchdog Counter Match Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_match::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_match::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCWDT_MATCH_SPEC;
impl crate::RegisterSpec for MCWDT_MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_match::R`](R) reader structure"]
impl crate::Readable for MCWDT_MATCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_match::W`](W) writer structure"]
impl crate::Writable for MCWDT_MATCH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCWDT_MATCH to value 0"]
impl crate::Resettable for MCWDT_MATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
