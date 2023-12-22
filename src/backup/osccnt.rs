#[doc = "Register `OSCCNT` reader"]
pub type R = crate::R<OSCCNT_SPEC>;
#[doc = "Field `CNT32KHZ` reader - 32kHz oscillator count (msb=128Hz), calibration can cause bit 6 to skip. Reset when RTC_TIME.RTC_SEC fields is written."]
pub type CNT32KHZ_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 32kHz oscillator count (msb=128Hz), calibration can cause bit 6 to skip. Reset when RTC_TIME.RTC_SEC fields is written."]
    #[inline(always)]
    pub fn cnt32khz(&self) -> CNT32KHZ_R {
        CNT32KHZ_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "32kHz oscillator counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCCNT_SPEC;
impl crate::RegisterSpec for OSCCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osccnt::R`](R) reader structure"]
impl crate::Readable for OSCCNT_SPEC {}
#[doc = "`reset()` method sets OSCCNT to value 0"]
impl crate::Resettable for OSCCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
