#[doc = "Register `MASK1` reader"]
pub type R = crate::R<MASK1_SPEC>;
#[doc = "Register `MASK1` writer"]
pub type W = crate::W<MASK1_SPEC>;
#[doc = "Field `SOURCE` reader - Fault source enables: Bits 31-0: Fault sources 63 to 32."]
pub type SOURCE_R = crate::FieldReader<u32>;
#[doc = "Field `SOURCE` writer - Fault source enables: Bits 31-0: Fault sources 63 to 32."]
pub type SOURCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 63 to 32."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 63 to 32."]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<MASK1_SPEC> {
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
#[doc = "Fault mask 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASK1_SPEC;
impl crate::RegisterSpec for MASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask1::R`](R) reader structure"]
impl crate::Readable for MASK1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mask1::W`](W) writer structure"]
impl crate::Writable for MASK1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK1 to value 0"]
impl crate::Resettable for MASK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
