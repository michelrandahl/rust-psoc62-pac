#[doc = "Register `PERIOD` reader"]
pub type R = crate::R<PERIOD_SPEC>;
#[doc = "Register `PERIOD` writer"]
pub type W = crate::W<PERIOD_SPEC>;
#[doc = "Field `PERIOD` reader - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
pub type PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `PERIOD` writer - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
pub type PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<PERIOD_SPEC> {
        PERIOD_W::new(self, 0)
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
#[doc = "Counter period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`period::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`period::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIOD_SPEC;
impl crate::RegisterSpec for PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`period::R`](R) reader structure"]
impl crate::Readable for PERIOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`period::W`](W) writer structure"]
impl crate::Writable for PERIOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIOD to value 0xffff_ffff"]
impl crate::Resettable for PERIOD_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
