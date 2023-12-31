#[doc = "Register `SPARE` reader"]
pub type R = crate::R<SPARE_SPEC>;
#[doc = "Register `SPARE` writer"]
pub type W = crate::W<SPARE_SPEC>;
#[doc = "Field `SPARE` reader - Spare MMIO"]
pub type SPARE_R = crate::FieldReader;
#[doc = "Field `SPARE` writer - Spare MMIO"]
pub type SPARE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Spare MMIO"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Spare MMIO"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SPARE_W<SPARE_SPEC> {
        SPARE_W::new(self, 0)
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
#[doc = "Spare MMIO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spare::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spare::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPARE_SPEC;
impl crate::RegisterSpec for SPARE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spare::R`](R) reader structure"]
impl crate::Readable for SPARE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spare::W`](W) writer structure"]
impl crate::Writable for SPARE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPARE to value 0"]
impl crate::Resettable for SPARE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
