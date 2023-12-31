#[doc = "Register `INTR` reader"]
pub type R = crate::R<INTR_SPEC>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<INTR_SPEC>;
#[doc = "Field `TC` reader - Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type TC_R = crate::BitReader;
#[doc = "Field `TC` writer - Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC_MATCH` reader - Counter matches CC register event. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type CC_MATCH_R = crate::BitReader;
#[doc = "Field `CC_MATCH` writer - Counter matches CC register event. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type CC_MATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter matches CC register event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn cc_match(&self) -> CC_MATCH_R {
        CC_MATCH_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<INTR_SPEC> {
        TC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Counter matches CC register event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn cc_match(&mut self) -> CC_MATCH_W<INTR_SPEC> {
        CC_MATCH_W::new(self, 1)
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
#[doc = "Interrupt request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
