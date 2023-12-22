#[doc = "Register `SATURATE_INTR_MASK` reader"]
pub type R = crate::R<SATURATE_INTR_MASK_SPEC>;
#[doc = "Register `SATURATE_INTR_MASK` writer"]
pub type W = crate::W<SATURATE_INTR_MASK_SPEC>;
#[doc = "Field `SATURATE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type SATURATE_MASK_R = crate::FieldReader<u16>;
#[doc = "Field `SATURATE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type SATURATE_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn saturate_mask(&self) -> SATURATE_MASK_R {
        SATURATE_MASK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn saturate_mask(&mut self) -> SATURATE_MASK_W<SATURATE_INTR_MASK_SPEC> {
        SATURATE_MASK_W::new(self, 0)
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
#[doc = "Saturate interrupt mask register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saturate_intr_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saturate_intr_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SATURATE_INTR_MASK_SPEC;
impl crate::RegisterSpec for SATURATE_INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saturate_intr_mask::R`](R) reader structure"]
impl crate::Readable for SATURATE_INTR_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saturate_intr_mask::W`](W) writer structure"]
impl crate::Writable for SATURATE_INTR_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SATURATE_INTR_MASK to value 0"]
impl crate::Resettable for SATURATE_INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
