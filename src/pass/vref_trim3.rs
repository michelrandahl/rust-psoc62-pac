#[doc = "Register `VREF_TRIM3` reader"]
pub type R = crate::R<VREF_TRIM3_SPEC>;
#[doc = "Register `VREF_TRIM3` writer"]
pub type W = crate::W<VREF_TRIM3_SPEC>;
#[doc = "Field `VREF_ATTEN_TRIM` reader - Obsolete"]
pub type VREF_ATTEN_TRIM_R = crate::FieldReader;
#[doc = "Field `VREF_ATTEN_TRIM` writer - Obsolete"]
pub type VREF_ATTEN_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Obsolete"]
    #[inline(always)]
    pub fn vref_atten_trim(&self) -> VREF_ATTEN_TRIM_R {
        VREF_ATTEN_TRIM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Obsolete"]
    #[inline(always)]
    #[must_use]
    pub fn vref_atten_trim(&mut self) -> VREF_ATTEN_TRIM_W<VREF_TRIM3_SPEC> {
        VREF_ATTEN_TRIM_W::new(self, 0)
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
#[doc = "VREF Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vref_trim3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vref_trim3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREF_TRIM3_SPEC;
impl crate::RegisterSpec for VREF_TRIM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref_trim3::R`](R) reader structure"]
impl crate::Readable for VREF_TRIM3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vref_trim3::W`](W) writer structure"]
impl crate::Writable for VREF_TRIM3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREF_TRIM3 to value 0"]
impl crate::Resettable for VREF_TRIM3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
