#[doc = "Register `ARGUMENT_R` reader"]
pub type R = crate::R<ARGUMENT_R_SPEC>;
#[doc = "Register `ARGUMENT_R` writer"]
pub type W = crate::W<ARGUMENT_R_SPEC>;
#[doc = "Field `ARGUMENT` reader - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
pub type ARGUMENT_R = crate::FieldReader<u32>;
#[doc = "Field `ARGUMENT` writer - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
pub type ARGUMENT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
    #[inline(always)]
    pub fn argument(&self) -> ARGUMENT_R {
        ARGUMENT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
    #[inline(always)]
    #[must_use]
    pub fn argument(&mut self) -> ARGUMENT_W<ARGUMENT_R_SPEC> {
        ARGUMENT_W::new(self, 0)
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
#[doc = "Argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argument_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argument_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARGUMENT_R_SPEC;
impl crate::RegisterSpec for ARGUMENT_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`argument_r::R`](R) reader structure"]
impl crate::Readable for ARGUMENT_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`argument_r::W`](W) writer structure"]
impl crate::Writable for ARGUMENT_R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARGUMENT_R to value 0"]
impl crate::Resettable for ARGUMENT_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
