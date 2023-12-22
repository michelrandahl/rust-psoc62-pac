#[doc = "Register `VREF_TRIM2` reader"]
pub type R = crate::R<VREF_TRIM2_SPEC>;
#[doc = "Register `VREF_TRIM2` writer"]
pub type W = crate::W<VREF_TRIM2_SPEC>;
#[doc = "Field `VREF_CURV_TRIM` reader - N/A"]
pub type VREF_CURV_TRIM_R = crate::FieldReader;
#[doc = "Field `VREF_CURV_TRIM` writer - N/A"]
pub type VREF_CURV_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn vref_curv_trim(&self) -> VREF_CURV_TRIM_R {
        VREF_CURV_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vref_curv_trim(&mut self) -> VREF_CURV_TRIM_W<VREF_TRIM2_SPEC> {
        VREF_CURV_TRIM_W::new(self, 0)
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
#[doc = "VREF Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vref_trim2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vref_trim2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREF_TRIM2_SPEC;
impl crate::RegisterSpec for VREF_TRIM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref_trim2::R`](R) reader structure"]
impl crate::Readable for VREF_TRIM2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vref_trim2::W`](W) writer structure"]
impl crate::Writable for VREF_TRIM2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREF_TRIM2 to value 0"]
impl crate::Resettable for VREF_TRIM2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
