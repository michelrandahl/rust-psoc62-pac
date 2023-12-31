#[doc = "Register `TRIM` reader"]
pub type R = crate::R<TRIM_SPEC>;
#[doc = "Register `TRIM` writer"]
pub type W = crate::W<TRIM_SPEC>;
#[doc = "Field `TRIM` reader - WCO trim"]
pub type TRIM_R = crate::FieldReader;
#[doc = "Field `TRIM` writer - WCO trim"]
pub type TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - WCO trim"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - WCO trim"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<TRIM_SPEC> {
        TRIM_W::new(self, 0)
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
#[doc = "Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIM_SPEC;
impl crate::RegisterSpec for TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trim::R`](R) reader structure"]
impl crate::Readable for TRIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trim::W`](W) writer structure"]
impl crate::Writable for TRIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIM to value 0"]
impl crate::Resettable for TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
