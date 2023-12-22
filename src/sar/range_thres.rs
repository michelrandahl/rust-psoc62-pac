#[doc = "Register `RANGE_THRES` reader"]
pub type R = crate::R<RANGE_THRES_SPEC>;
#[doc = "Register `RANGE_THRES` writer"]
pub type W = crate::W<RANGE_THRES_SPEC>;
#[doc = "Field `RANGE_LOW` reader - Low threshold for range detect."]
pub type RANGE_LOW_R = crate::FieldReader<u16>;
#[doc = "Field `RANGE_LOW` writer - Low threshold for range detect."]
pub type RANGE_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RANGE_HIGH` reader - High threshold for range detect."]
pub type RANGE_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `RANGE_HIGH` writer - High threshold for range detect."]
pub type RANGE_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low threshold for range detect."]
    #[inline(always)]
    pub fn range_low(&self) -> RANGE_LOW_R {
        RANGE_LOW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High threshold for range detect."]
    #[inline(always)]
    pub fn range_high(&self) -> RANGE_HIGH_R {
        RANGE_HIGH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low threshold for range detect."]
    #[inline(always)]
    #[must_use]
    pub fn range_low(&mut self) -> RANGE_LOW_W<RANGE_THRES_SPEC> {
        RANGE_LOW_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - High threshold for range detect."]
    #[inline(always)]
    #[must_use]
    pub fn range_high(&mut self) -> RANGE_HIGH_W<RANGE_THRES_SPEC> {
        RANGE_HIGH_W::new(self, 16)
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
#[doc = "Global range detect threshold register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`range_thres::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`range_thres::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANGE_THRES_SPEC;
impl crate::RegisterSpec for RANGE_THRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`range_thres::R`](R) reader structure"]
impl crate::Readable for RANGE_THRES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`range_thres::W`](W) writer structure"]
impl crate::Writable for RANGE_THRES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RANGE_THRES to value 0"]
impl crate::Resettable for RANGE_THRES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
