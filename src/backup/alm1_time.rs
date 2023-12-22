#[doc = "Register `ALM1_TIME` reader"]
pub type R = crate::R<ALM1_TIME_SPEC>;
#[doc = "Register `ALM1_TIME` writer"]
pub type W = crate::W<ALM1_TIME_SPEC>;
#[doc = "Field `ALM_SEC` reader - Alarm seconds in BCD, 0-59"]
pub type ALM_SEC_R = crate::FieldReader;
#[doc = "Field `ALM_SEC` writer - Alarm seconds in BCD, 0-59"]
pub type ALM_SEC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ALM_SEC_EN` reader - Alarm second enable: 0=ignore, 1=match"]
pub type ALM_SEC_EN_R = crate::BitReader;
#[doc = "Field `ALM_SEC_EN` writer - Alarm second enable: 0=ignore, 1=match"]
pub type ALM_SEC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALM_MIN` reader - Alarm minutes in BCD, 0-59"]
pub type ALM_MIN_R = crate::FieldReader;
#[doc = "Field `ALM_MIN` writer - Alarm minutes in BCD, 0-59"]
pub type ALM_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ALM_MIN_EN` reader - Alarm minutes enable: 0=ignore, 1=match"]
pub type ALM_MIN_EN_R = crate::BitReader;
#[doc = "Field `ALM_MIN_EN` writer - Alarm minutes enable: 0=ignore, 1=match"]
pub type ALM_MIN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALM_HOUR` reader - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
pub type ALM_HOUR_R = crate::FieldReader;
#[doc = "Field `ALM_HOUR` writer - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
pub type ALM_HOUR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ALM_HOUR_EN` reader - Alarm hour enable: 0=ignore, 1=match"]
pub type ALM_HOUR_EN_R = crate::BitReader;
#[doc = "Field `ALM_HOUR_EN` writer - Alarm hour enable: 0=ignore, 1=match"]
pub type ALM_HOUR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALM_DAY` reader - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub type ALM_DAY_R = crate::FieldReader;
#[doc = "Field `ALM_DAY` writer - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub type ALM_DAY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ALM_DAY_EN` reader - Alarm Day of the Week enable: 0=ignore, 1=match"]
pub type ALM_DAY_EN_R = crate::BitReader;
#[doc = "Field `ALM_DAY_EN` writer - Alarm Day of the Week enable: 0=ignore, 1=match"]
pub type ALM_DAY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_sec(&self) -> ALM_SEC_R {
        ALM_SEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_sec_en(&self) -> ALM_SEC_EN_R {
        ALM_SEC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_min(&self) -> ALM_MIN_R {
        ALM_MIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_min_en(&self) -> ALM_MIN_EN_R {
        ALM_MIN_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    pub fn alm_hour(&self) -> ALM_HOUR_R {
        ALM_HOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_hour_en(&self) -> ALM_HOUR_EN_R {
        ALM_HOUR_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn alm_day(&self) -> ALM_DAY_R {
        ALM_DAY_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_day_en(&self) -> ALM_DAY_EN_R {
        ALM_DAY_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    #[must_use]
    pub fn alm_sec(&mut self) -> ALM_SEC_W<ALM1_TIME_SPEC> {
        ALM_SEC_W::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    #[must_use]
    pub fn alm_sec_en(&mut self) -> ALM_SEC_EN_W<ALM1_TIME_SPEC> {
        ALM_SEC_EN_W::new(self, 7)
    }
    #[doc = "Bits 8:14 - Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    #[must_use]
    pub fn alm_min(&mut self) -> ALM_MIN_W<ALM1_TIME_SPEC> {
        ALM_MIN_W::new(self, 8)
    }
    #[doc = "Bit 15 - Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    #[must_use]
    pub fn alm_min_en(&mut self) -> ALM_MIN_EN_W<ALM1_TIME_SPEC> {
        ALM_MIN_EN_W::new(self, 15)
    }
    #[doc = "Bits 16:21 - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    #[must_use]
    pub fn alm_hour(&mut self) -> ALM_HOUR_W<ALM1_TIME_SPEC> {
        ALM_HOUR_W::new(self, 16)
    }
    #[doc = "Bit 23 - Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    #[must_use]
    pub fn alm_hour_en(&mut self) -> ALM_HOUR_EN_W<ALM1_TIME_SPEC> {
        ALM_HOUR_EN_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    #[must_use]
    pub fn alm_day(&mut self) -> ALM_DAY_W<ALM1_TIME_SPEC> {
        ALM_DAY_W::new(self, 24)
    }
    #[doc = "Bit 31 - Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    #[must_use]
    pub fn alm_day_en(&mut self) -> ALM_DAY_EN_W<ALM1_TIME_SPEC> {
        ALM_DAY_EN_W::new(self, 31)
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
#[doc = "Alarm 1 Seconds, Minute, Hours, Day of Week\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alm1_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alm1_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALM1_TIME_SPEC;
impl crate::RegisterSpec for ALM1_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alm1_time::R`](R) reader structure"]
impl crate::Readable for ALM1_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alm1_time::W`](W) writer structure"]
impl crate::Writable for ALM1_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALM1_TIME to value 0x0100_0000"]
impl crate::Resettable for ALM1_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
