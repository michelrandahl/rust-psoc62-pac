#[doc = "Register `ANA_CTL1` reader"]
pub type R = crate::R<ANA_CTL1_SPEC>;
#[doc = "Register `ANA_CTL1` writer"]
pub type W = crate::W<ANA_CTL1_SPEC>;
#[doc = "Field `NDAC_MAX` reader - Ndac Max Value.Trimming of negative pump output Voltage."]
pub type NDAC_MAX_R = crate::FieldReader;
#[doc = "Field `NDAC_MAX` writer - Ndac Max Value.Trimming of negative pump output Voltage."]
pub type NDAC_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NDAC_STEP` reader - Ndac step increment"]
pub type NDAC_STEP_R = crate::FieldReader;
#[doc = "Field `NDAC_STEP` writer - Ndac step increment"]
pub type NDAC_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PDAC_MAX` reader - Pdac Max Value.Trimming of positive pump output Voltage:"]
pub type PDAC_MAX_R = crate::FieldReader;
#[doc = "Field `PDAC_MAX` writer - Pdac Max Value.Trimming of positive pump output Voltage:"]
pub type PDAC_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PDAC_STEP` reader - Pdac step increment"]
pub type PDAC_STEP_R = crate::FieldReader;
#[doc = "Field `PDAC_STEP` writer - Pdac step increment"]
pub type PDAC_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NPDAC_STEP_TIME` reader - Ndac/Pdac step duration: (1uS .. 255uS) * 8 When = 0 N/PDAC_MAX control the pumps"]
pub type NPDAC_STEP_TIME_R = crate::FieldReader;
#[doc = "Field `NPDAC_STEP_TIME` writer - Ndac/Pdac step duration: (1uS .. 255uS) * 8 When = 0 N/PDAC_MAX control the pumps"]
pub type NPDAC_STEP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NPDAC_ZERO_TIME` reader - Ndac/Pdac LO duration: (1uS .. 255uS) * 8 When 0, N/PDAC don't return to 0"]
pub type NPDAC_ZERO_TIME_R = crate::FieldReader;
#[doc = "Field `NPDAC_ZERO_TIME` writer - Ndac/Pdac LO duration: (1uS .. 255uS) * 8 When 0, N/PDAC don't return to 0"]
pub type NPDAC_ZERO_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Ndac Max Value.Trimming of negative pump output Voltage."]
    #[inline(always)]
    pub fn ndac_max(&self) -> NDAC_MAX_R {
        NDAC_MAX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Ndac step increment"]
    #[inline(always)]
    pub fn ndac_step(&self) -> NDAC_STEP_R {
        NDAC_STEP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pdac Max Value.Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub fn pdac_max(&self) -> PDAC_MAX_R {
        PDAC_MAX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pdac step increment"]
    #[inline(always)]
    pub fn pdac_step(&self) -> PDAC_STEP_R {
        PDAC_STEP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Ndac/Pdac step duration: (1uS .. 255uS) * 8 When = 0 N/PDAC_MAX control the pumps"]
    #[inline(always)]
    pub fn npdac_step_time(&self) -> NPDAC_STEP_TIME_R {
        NPDAC_STEP_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Ndac/Pdac LO duration: (1uS .. 255uS) * 8 When 0, N/PDAC don't return to 0"]
    #[inline(always)]
    pub fn npdac_zero_time(&self) -> NPDAC_ZERO_TIME_R {
        NPDAC_ZERO_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Ndac Max Value.Trimming of negative pump output Voltage."]
    #[inline(always)]
    #[must_use]
    pub fn ndac_max(&mut self) -> NDAC_MAX_W<ANA_CTL1_SPEC> {
        NDAC_MAX_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Ndac step increment"]
    #[inline(always)]
    #[must_use]
    pub fn ndac_step(&mut self) -> NDAC_STEP_W<ANA_CTL1_SPEC> {
        NDAC_STEP_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Pdac Max Value.Trimming of positive pump output Voltage:"]
    #[inline(always)]
    #[must_use]
    pub fn pdac_max(&mut self) -> PDAC_MAX_W<ANA_CTL1_SPEC> {
        PDAC_MAX_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Pdac step increment"]
    #[inline(always)]
    #[must_use]
    pub fn pdac_step(&mut self) -> PDAC_STEP_W<ANA_CTL1_SPEC> {
        PDAC_STEP_W::new(self, 12)
    }
    #[doc = "Bits 16:23 - Ndac/Pdac step duration: (1uS .. 255uS) * 8 When = 0 N/PDAC_MAX control the pumps"]
    #[inline(always)]
    #[must_use]
    pub fn npdac_step_time(&mut self) -> NPDAC_STEP_TIME_W<ANA_CTL1_SPEC> {
        NPDAC_STEP_TIME_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Ndac/Pdac LO duration: (1uS .. 255uS) * 8 When 0, N/PDAC don't return to 0"]
    #[inline(always)]
    #[must_use]
    pub fn npdac_zero_time(&mut self) -> NPDAC_ZERO_TIME_W<ANA_CTL1_SPEC> {
        NPDAC_ZERO_TIME_W::new(self, 24)
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
#[doc = "Analog control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_CTL1_SPEC;
impl crate::RegisterSpec for ANA_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_ctl1::R`](R) reader structure"]
impl crate::Readable for ANA_CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_ctl1::W`](W) writer structure"]
impl crate::Writable for ANA_CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANA_CTL1 to value 0x0d32_fafa"]
impl crate::Resettable for ANA_CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d32_fafa;
}
