#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `RTC_BUSY` reader - pending RTC write"]
pub type RTC_BUSY_R = crate::BitReader;
#[doc = "Field `WCO_OK` reader - Indicates that output has transitioned."]
pub type WCO_OK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - pending RTC write"]
    #[inline(always)]
    pub fn rtc_busy(&self) -> RTC_BUSY_R {
        RTC_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that output has transitioned."]
    #[inline(always)]
    pub fn wco_ok(&self) -> WCO_OK_R {
        WCO_OK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
