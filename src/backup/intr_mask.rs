#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<INTR_MASK_SPEC>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<INTR_MASK_SPEC>;
#[doc = "Field `ALARM1` reader - Mask bit for corresponding bit in interrupt request register."]
pub type ALARM1_R = crate::BitReader;
#[doc = "Field `ALARM1` writer - Mask bit for corresponding bit in interrupt request register."]
pub type ALARM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM2` reader - Mask bit for corresponding bit in interrupt request register."]
pub type ALARM2_R = crate::BitReader;
#[doc = "Field `ALARM2` writer - Mask bit for corresponding bit in interrupt request register."]
pub type ALARM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CENTURY` reader - Mask bit for corresponding bit in interrupt request register."]
pub type CENTURY_R = crate::BitReader;
#[doc = "Field `CENTURY` writer - Mask bit for corresponding bit in interrupt request register."]
pub type CENTURY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn alarm2(&self) -> ALARM2_R {
        ALARM2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn century(&self) -> CENTURY_R {
        CENTURY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn alarm1(&mut self) -> ALARM1_W<INTR_MASK_SPEC> {
        ALARM1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn alarm2(&mut self) -> ALARM2_W<INTR_MASK_SPEC> {
        ALARM2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn century(&mut self) -> CENTURY_W<INTR_MASK_SPEC> {
        CENTURY_W::new(self, 2)
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
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
