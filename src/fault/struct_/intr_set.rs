#[doc = "Register `INTR_SET` reader"]
pub type R = crate::R<INTR_SET_SPEC>;
#[doc = "Register `INTR_SET` writer"]
pub type W = crate::W<INTR_SET_SPEC>;
#[doc = "Field `FAULT` reader - SW writes a '1' to this field to set the corresponding field in the INTR register."]
pub type FAULT_R = crate::BitReader;
#[doc = "Field `FAULT` writer - SW writes a '1' to this field to set the corresponding field in the INTR register."]
pub type FAULT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn fault(&mut self) -> FAULT_W<INTR_SET_SPEC> {
        FAULT_W::new(self, 0)
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
#[doc = "Interrupt set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SET_SPEC;
impl crate::RegisterSpec for INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_set::R`](R) reader structure"]
impl crate::Readable for INTR_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_set::W`](W) writer structure"]
impl crate::Writable for INTR_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for INTR_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
