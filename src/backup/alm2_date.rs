#[doc = "Register `ALM2_DATE` reader"]
pub type R = crate::R<ALM2_DATE_SPEC>;
#[doc = "Register `ALM2_DATE` writer"]
pub type W = crate::W<ALM2_DATE_SPEC>;
#[doc = "Field `ALM_DATE` reader - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
pub type ALM_DATE_R = crate::FieldReader;
#[doc = "Field `ALM_DATE` writer - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
pub type ALM_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ALM_DATE_EN` reader - Alarm Day of the Month enable: 0=ignore, 1=match"]
pub type ALM_DATE_EN_R = crate::BitReader;
#[doc = "Field `ALM_DATE_EN` writer - Alarm Day of the Month enable: 0=ignore, 1=match"]
pub type ALM_DATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALM_MON` reader - Alarm Month in BCD, 1-12"]
pub type ALM_MON_R = crate::FieldReader;
#[doc = "Field `ALM_MON` writer - Alarm Month in BCD, 1-12"]
pub type ALM_MON_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ALM_MON_EN` reader - Alarm Month enable: 0=ignore, 1=match"]
pub type ALM_MON_EN_R = crate::BitReader;
#[doc = "Field `ALM_MON_EN` writer - Alarm Month enable: 0=ignore, 1=match"]
pub type ALM_MON_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALM_EN` reader - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. If none of the date and time fields are enabled, then this alarm triggers once every second."]
pub type ALM_EN_R = crate::BitReader;
#[doc = "Field `ALM_EN` writer - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. If none of the date and time fields are enabled, then this alarm triggers once every second."]
pub type ALM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    pub fn alm_date(&self) -> ALM_DATE_R {
        ALM_DATE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_date_en(&self) -> ALM_DATE_EN_R {
        ALM_DATE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Alarm Month in BCD, 1-12"]
    #[inline(always)]
    pub fn alm_mon(&self) -> ALM_MON_R {
        ALM_MON_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_mon_en(&self) -> ALM_MON_EN_R {
        ALM_MON_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    pub fn alm_en(&self) -> ALM_EN_R {
        ALM_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    #[must_use]
    pub fn alm_date(&mut self) -> ALM_DATE_W<ALM2_DATE_SPEC> {
        ALM_DATE_W::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    #[must_use]
    pub fn alm_date_en(&mut self) -> ALM_DATE_EN_W<ALM2_DATE_SPEC> {
        ALM_DATE_EN_W::new(self, 7)
    }
    #[doc = "Bits 8:12 - Alarm Month in BCD, 1-12"]
    #[inline(always)]
    #[must_use]
    pub fn alm_mon(&mut self) -> ALM_MON_W<ALM2_DATE_SPEC> {
        ALM_MON_W::new(self, 8)
    }
    #[doc = "Bit 15 - Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    #[must_use]
    pub fn alm_mon_en(&mut self) -> ALM_MON_EN_W<ALM2_DATE_SPEC> {
        ALM_MON_EN_W::new(self, 15)
    }
    #[doc = "Bit 31 - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    #[must_use]
    pub fn alm_en(&mut self) -> ALM_EN_W<ALM2_DATE_SPEC> {
        ALM_EN_W::new(self, 31)
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
#[doc = "Alarm 2 Day of Month, Month\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alm2_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alm2_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALM2_DATE_SPEC;
impl crate::RegisterSpec for ALM2_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alm2_date::R`](R) reader structure"]
impl crate::Readable for ALM2_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alm2_date::W`](W) writer structure"]
impl crate::Writable for ALM2_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALM2_DATE to value 0x0101"]
impl crate::Resettable for ALM2_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101;
}
