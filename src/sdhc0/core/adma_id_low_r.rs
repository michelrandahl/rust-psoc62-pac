#[doc = "Register `ADMA_ID_LOW_R` reader"]
pub type R = crate::R<ADMA_ID_LOW_R_SPEC>;
#[doc = "Register `ADMA_ID_LOW_R` writer"]
pub type W = crate::W<ADMA_ID_LOW_R_SPEC>;
#[doc = "Field `ADMA_ID_LOW` reader - ADMA Integrated Descriptor Address These bits indicate the lower 32-bit of the ADMA Integrated Descriptor address. The start address of Integrated Descriptor is set to these register bits. The ADMA3 fetches one Descriptor Address and increments these bits to indicate the next Descriptor address."]
pub type ADMA_ID_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `ADMA_ID_LOW` writer - ADMA Integrated Descriptor Address These bits indicate the lower 32-bit of the ADMA Integrated Descriptor address. The start address of Integrated Descriptor is set to these register bits. The ADMA3 fetches one Descriptor Address and increments these bits to indicate the next Descriptor address."]
pub type ADMA_ID_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADMA Integrated Descriptor Address These bits indicate the lower 32-bit of the ADMA Integrated Descriptor address. The start address of Integrated Descriptor is set to these register bits. The ADMA3 fetches one Descriptor Address and increments these bits to indicate the next Descriptor address."]
    #[inline(always)]
    pub fn adma_id_low(&self) -> ADMA_ID_LOW_R {
        ADMA_ID_LOW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA Integrated Descriptor Address These bits indicate the lower 32-bit of the ADMA Integrated Descriptor address. The start address of Integrated Descriptor is set to these register bits. The ADMA3 fetches one Descriptor Address and increments these bits to indicate the next Descriptor address."]
    #[inline(always)]
    #[must_use]
    pub fn adma_id_low(&mut self) -> ADMA_ID_LOW_W<ADMA_ID_LOW_R_SPEC> {
        ADMA_ID_LOW_W::new(self, 0)
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
#[doc = "ADMA3 Integrated Descriptor Address Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adma_id_low_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adma_id_low_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADMA_ID_LOW_R_SPEC;
impl crate::RegisterSpec for ADMA_ID_LOW_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adma_id_low_r::R`](R) reader structure"]
impl crate::Readable for ADMA_ID_LOW_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adma_id_low_r::W`](W) writer structure"]
impl crate::Writable for ADMA_ID_LOW_R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADMA_ID_LOW_R to value 0"]
impl crate::Resettable for ADMA_ID_LOW_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
