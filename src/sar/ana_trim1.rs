#[doc = "Register `ANA_TRIM1` reader"]
pub type R = crate::R<ANA_TRIM1_SPEC>;
#[doc = "Register `ANA_TRIM1` writer"]
pub type W = crate::W<ANA_TRIM1_SPEC>;
#[doc = "Field `SAR_REF_BUF_TRIM` reader - SAR Reference buffer trim"]
pub type SAR_REF_BUF_TRIM_R = crate::FieldReader;
#[doc = "Field `SAR_REF_BUF_TRIM` writer - SAR Reference buffer trim"]
pub type SAR_REF_BUF_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - SAR Reference buffer trim"]
    #[inline(always)]
    pub fn sar_ref_buf_trim(&self) -> SAR_REF_BUF_TRIM_R {
        SAR_REF_BUF_TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - SAR Reference buffer trim"]
    #[inline(always)]
    #[must_use]
    pub fn sar_ref_buf_trim(&mut self) -> SAR_REF_BUF_TRIM_W<ANA_TRIM1_SPEC> {
        SAR_REF_BUF_TRIM_W::new(self, 0)
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
#[doc = "Analog trim register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_trim1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_trim1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_TRIM1_SPEC;
impl crate::RegisterSpec for ANA_TRIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_trim1::R`](R) reader structure"]
impl crate::Readable for ANA_TRIM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_trim1::W`](W) writer structure"]
impl crate::Writable for ANA_TRIM1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANA_TRIM1 to value 0"]
impl crate::Resettable for ANA_TRIM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
