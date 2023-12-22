#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<INTR_MASK_SPEC>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<INTR_MASK_SPEC>;
#[doc = "Field `SAMPLE` reader - Mask bit for corresponding bit in interrupt request register."]
pub type SAMPLE_R = crate::BitReader;
#[doc = "Field `SAMPLE` writer - Mask bit for corresponding bit in interrupt request register."]
pub type SAMPLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INIT` reader - Mask bit for corresponding bit in interrupt request register."]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - Mask bit for corresponding bit in interrupt request register."]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_RES` reader - Mask bit for corresponding bit in interrupt request register."]
pub type ADC_RES_R = crate::BitReader;
#[doc = "Field `ADC_RES` writer - Mask bit for corresponding bit in interrupt request register."]
pub type ADC_RES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn adc_res(&self) -> ADC_RES_R {
        ADC_RES_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn sample(&mut self) -> SAMPLE_W<INTR_MASK_SPEC> {
        SAMPLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<INTR_MASK_SPEC> {
        INIT_W::new(self, 2)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn adc_res(&mut self) -> ADC_RES_W<INTR_MASK_SPEC> {
        ADC_RES_W::new(self, 8)
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
#[doc = "CSD Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
