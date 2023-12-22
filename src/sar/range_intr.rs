#[doc = "Register `RANGE_INTR` reader"]
pub type R = crate::R<RANGE_INTR_SPEC>;
#[doc = "Register `RANGE_INTR` writer"]
pub type W = crate::W<RANGE_INTR_SPEC>;
#[doc = "Field `RANGE_INTR` reader - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
pub type RANGE_INTR_R = crate::FieldReader<u16>;
#[doc = "Field `RANGE_INTR` writer - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
pub type RANGE_INTR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn range_intr(&self) -> RANGE_INTR_R {
        RANGE_INTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn range_intr(&mut self) -> RANGE_INTR_W<RANGE_INTR_SPEC> {
        RANGE_INTR_W::new(self, 0)
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
#[doc = "Range detect interrupt request register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`range_intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`range_intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANGE_INTR_SPEC;
impl crate::RegisterSpec for RANGE_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`range_intr::R`](R) reader structure"]
impl crate::Readable for RANGE_INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`range_intr::W`](W) writer structure"]
impl crate::Writable for RANGE_INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RANGE_INTR to value 0"]
impl crate::Resettable for RANGE_INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
