#[doc = "Register `CC` reader"]
pub type R = crate::R<CC_SPEC>;
#[doc = "Register `CC` writer"]
pub type W = crate::W<CC_SPEC>;
#[doc = "Field `CC` reader - In CAPTURE mode, captures the counter value. In other modes, compared to counter value."]
pub type CC_R = crate::FieldReader<u32>;
#[doc = "Field `CC` writer - In CAPTURE mode, captures the counter value. In other modes, compared to counter value."]
pub type CC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - In CAPTURE mode, captures the counter value. In other modes, compared to counter value."]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - In CAPTURE mode, captures the counter value. In other modes, compared to counter value."]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CC_W<CC_SPEC> {
        CC_W::new(self, 0)
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
#[doc = "Counter compare/capture register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC to value 0xffff_ffff"]
impl crate::Resettable for CC_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
