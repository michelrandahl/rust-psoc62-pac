#[doc = "Register `INTR` reader"]
pub type R = crate::R<INTR_SPEC>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<INTR_SPEC>;
#[doc = "Field `SAMPLE` reader - A normal sample is complete"]
pub type SAMPLE_R = crate::BitReader;
#[doc = "Field `SAMPLE` writer - A normal sample is complete"]
pub type SAMPLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INIT` reader - Coarse initialization complete or Sample initialization complete (the latter is typically ignored)"]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - Coarse initialization complete or Sample initialization complete (the latter is typically ignored)"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_RES` reader - ADC Result ready"]
pub type ADC_RES_R = crate::BitReader;
#[doc = "Field `ADC_RES` writer - ADC Result ready"]
pub type ADC_RES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - A normal sample is complete"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Coarse initialization complete or Sample initialization complete (the latter is typically ignored)"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC Result ready"]
    #[inline(always)]
    pub fn adc_res(&self) -> ADC_RES_R {
        ADC_RES_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - A normal sample is complete"]
    #[inline(always)]
    #[must_use]
    pub fn sample(&mut self) -> SAMPLE_W<INTR_SPEC> {
        SAMPLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Coarse initialization complete or Sample initialization complete (the latter is typically ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<INTR_SPEC> {
        INIT_W::new(self, 2)
    }
    #[doc = "Bit 8 - ADC Result ready"]
    #[inline(always)]
    #[must_use]
    pub fn adc_res(&mut self) -> ADC_RES_W<INTR_SPEC> {
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
#[doc = "CSD Interrupt Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
