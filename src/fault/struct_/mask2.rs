#[doc = "Register `MASK2` reader"]
pub type R = crate::R<MASK2_SPEC>;
#[doc = "Register `MASK2` writer"]
pub type W = crate::W<MASK2_SPEC>;
#[doc = "Field `SOURCE` reader - Fault source enables: Bits 31-0: Fault sources 95 to 64."]
pub type SOURCE_R = crate::FieldReader<u32>;
#[doc = "Field `SOURCE` writer - Fault source enables: Bits 31-0: Fault sources 95 to 64."]
pub type SOURCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 95 to 64."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 95 to 64."]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<MASK2_SPEC> {
        SOURCE_W::new(self, 0)
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
#[doc = "Fault mask 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASK2_SPEC;
impl crate::RegisterSpec for MASK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask2::R`](R) reader structure"]
impl crate::Readable for MASK2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mask2::W`](W) writer structure"]
impl crate::Writable for MASK2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK2 to value 0"]
impl crate::Resettable for MASK2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
