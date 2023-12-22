#[doc = "Register `SW_SHIELD_SEL` reader"]
pub type R = crate::R<SW_SHIELD_SEL_SPEC>;
#[doc = "Register `SW_SHIELD_SEL` writer"]
pub type W = crate::W<SW_SHIELD_SEL_SPEC>;
#[doc = "Field `SW_HCAV` reader - N/A"]
pub type SW_HCAV_R = crate::FieldReader;
#[doc = "Field `SW_HCAV` writer - N/A"]
pub type SW_HCAV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_HCAG` reader - Select waveform for corresponding switch"]
pub type SW_HCAG_R = crate::FieldReader;
#[doc = "Field `SW_HCAG` writer - Select waveform for corresponding switch"]
pub type SW_HCAG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_HCBV` reader - N/A"]
pub type SW_HCBV_R = crate::FieldReader;
#[doc = "Field `SW_HCBV` writer - N/A"]
pub type SW_HCBV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_HCBG` reader - Select waveform for corresponding switch, using csd_shield as base"]
pub type SW_HCBG_R = crate::FieldReader;
#[doc = "Field `SW_HCBG` writer - Select waveform for corresponding switch, using csd_shield as base"]
pub type SW_HCBG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_HCCV` reader - Set corresponding switch"]
pub type SW_HCCV_R = crate::BitReader;
#[doc = "Field `SW_HCCV` writer - Set corresponding switch"]
pub type SW_HCCV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HCCG` reader - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
pub type SW_HCCG_R = crate::BitReader;
#[doc = "Field `SW_HCCG` writer - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
pub type SW_HCCG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn sw_hcav(&self) -> SW_HCAV_R {
        SW_HCAV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcag(&self) -> SW_HCAG_R {
        SW_HCAG_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - N/A"]
    #[inline(always)]
    pub fn sw_hcbv(&self) -> SW_HCBV_R {
        SW_HCBV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch, using csd_shield as base"]
    #[inline(always)]
    pub fn sw_hcbg(&self) -> SW_HCBG_R {
        SW_HCBG_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccv(&self) -> SW_HCCV_R {
        SW_HCCV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn sw_hccg(&self) -> SW_HCCG_R {
        SW_HCCG_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hcav(&mut self) -> SW_HCAV_W<SW_SHIELD_SEL_SPEC> {
        SW_HCAV_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hcag(&mut self) -> SW_HCAG_W<SW_SHIELD_SEL_SPEC> {
        SW_HCAG_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hcbv(&mut self) -> SW_HCBV_W<SW_SHIELD_SEL_SPEC> {
        SW_HCBV_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch, using csd_shield as base"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hcbg(&mut self) -> SW_HCBG_W<SW_SHIELD_SEL_SPEC> {
        SW_HCBG_W::new(self, 12)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hccv(&mut self) -> SW_HCCV_W<SW_SHIELD_SEL_SPEC> {
        SW_HCCV_W::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    #[must_use]
    pub fn sw_hccg(&mut self) -> SW_HCCG_W<SW_SHIELD_SEL_SPEC> {
        SW_HCCG_W::new(self, 20)
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
#[doc = "Shielding switches Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_shield_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_shield_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_SHIELD_SEL_SPEC;
impl crate::RegisterSpec for SW_SHIELD_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_shield_sel::R`](R) reader structure"]
impl crate::Readable for SW_SHIELD_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_shield_sel::W`](W) writer structure"]
impl crate::Writable for SW_SHIELD_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_SHIELD_SEL to value 0"]
impl crate::Resettable for SW_SHIELD_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
