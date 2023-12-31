#[doc = "Register `SATURATE_INTR` reader"]
pub type R = crate::R<SATURATE_INTR_SPEC>;
#[doc = "Register `SATURATE_INTR` writer"]
pub type W = crate::W<SATURATE_INTR_SPEC>;
#[doc = "Field `SATURATE_INTR` reader - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
pub type SATURATE_INTR_R = crate::FieldReader<u16>;
#[doc = "Field `SATURATE_INTR` writer - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
pub type SATURATE_INTR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn saturate_intr(&self) -> SATURATE_INTR_R {
        SATURATE_INTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn saturate_intr(&mut self) -> SATURATE_INTR_W<SATURATE_INTR_SPEC> {
        SATURATE_INTR_W::new(self, 0)
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
#[doc = "Saturate interrupt request register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saturate_intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saturate_intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SATURATE_INTR_SPEC;
impl crate::RegisterSpec for SATURATE_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saturate_intr::R`](R) reader structure"]
impl crate::Readable for SATURATE_INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saturate_intr::W`](W) writer structure"]
impl crate::Writable for SATURATE_INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SATURATE_INTR to value 0"]
impl crate::Resettable for SATURATE_INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
