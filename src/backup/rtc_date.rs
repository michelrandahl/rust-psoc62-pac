#[doc = "Register `RTC_DATE` reader"]
pub type R = crate::R<RTC_DATE_SPEC>;
#[doc = "Register `RTC_DATE` writer"]
pub type W = crate::W<RTC_DATE_SPEC>;
#[doc = "Field `RTC_DATE` reader - Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
pub type RTC_DATE_R = crate::FieldReader;
#[doc = "Field `RTC_DATE` writer - Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
pub type RTC_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RTC_MON` reader - Calendar Month in BCD, 1-12"]
pub type RTC_MON_R = crate::FieldReader;
#[doc = "Field `RTC_MON` writer - Calendar Month in BCD, 1-12"]
pub type RTC_MON_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RTC_YEAR` reader - Calendar year in BCD, 0-99"]
pub type RTC_YEAR_R = crate::FieldReader;
#[doc = "Field `RTC_YEAR` writer - Calendar year in BCD, 0-99"]
pub type RTC_YEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
    #[inline(always)]
    pub fn rtc_date(&self) -> RTC_DATE_R {
        RTC_DATE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Calendar Month in BCD, 1-12"]
    #[inline(always)]
    pub fn rtc_mon(&self) -> RTC_MON_R {
        RTC_MON_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Calendar year in BCD, 0-99"]
    #[inline(always)]
    pub fn rtc_year(&self) -> RTC_YEAR_R {
        RTC_YEAR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_date(&mut self) -> RTC_DATE_W<RTC_DATE_SPEC> {
        RTC_DATE_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Calendar Month in BCD, 1-12"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mon(&mut self) -> RTC_MON_W<RTC_DATE_SPEC> {
        RTC_MON_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Calendar year in BCD, 0-99"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_year(&mut self) -> RTC_YEAR_W<RTC_DATE_SPEC> {
        RTC_YEAR_W::new(self, 16)
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
#[doc = "Calendar Day of Month, Month, Year\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_DATE_SPEC;
impl crate::RegisterSpec for RTC_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_date::R`](R) reader structure"]
impl crate::Readable for RTC_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_date::W`](W) writer structure"]
impl crate::Writable for RTC_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_DATE to value 0"]
impl crate::Resettable for RTC_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
