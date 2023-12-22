#[doc = "Register `SW_BYP_SEL` reader"]
pub type R = crate::R<SW_BYP_SEL_SPEC>;
#[doc = "Register `SW_BYP_SEL` writer"]
pub type W = crate::W<SW_BYP_SEL_SPEC>;
#[doc = "Field `SW_BYA` reader - Set corresponding switch"]
pub type SW_BYA_R = crate::BitReader;
#[doc = "Field `SW_BYA` writer - Set corresponding switch"]
pub type SW_BYA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_BYB` reader - Set corresponding switch"]
pub type SW_BYB_R = crate::BitReader;
#[doc = "Field `SW_BYB` writer - Set corresponding switch"]
pub type SW_BYB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_CBCC` reader - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
pub type SW_CBCC_R = crate::BitReader;
#[doc = "Field `SW_CBCC` writer - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
pub type SW_CBCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_bya(&self) -> SW_BYA_R {
        SW_BYA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_byb(&self) -> SW_BYB_R {
        SW_BYB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn sw_cbcc(&self) -> SW_CBCC_R {
        SW_CBCC_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_bya(&mut self) -> SW_BYA_W<SW_BYP_SEL_SPEC> {
        SW_BYA_W::new(self, 12)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_byb(&mut self) -> SW_BYB_W<SW_BYP_SEL_SPEC> {
        SW_BYB_W::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    #[must_use]
    pub fn sw_cbcc(&mut self) -> SW_CBCC_W<SW_BYP_SEL_SPEC> {
        SW_CBCC_W::new(self, 20)
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
#[doc = "AMUXBUS bypass switches Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_byp_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_byp_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_BYP_SEL_SPEC;
impl crate::RegisterSpec for SW_BYP_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_byp_sel::R`](R) reader structure"]
impl crate::Readable for SW_BYP_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_byp_sel::W`](W) writer structure"]
impl crate::Writable for SW_BYP_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_BYP_SEL to value 0"]
impl crate::Resettable for SW_BYP_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
