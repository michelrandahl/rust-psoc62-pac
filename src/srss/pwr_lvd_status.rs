#[doc = "Register `PWR_LVD_STATUS` reader"]
pub type R = crate::R<PWR_LVD_STATUS_SPEC>;
#[doc = "Field `HVLVD1_OK` reader - HVLVD1 output. 0: below voltage threshold 1: above voltage threshold"]
pub type HVLVD1_OK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HVLVD1 output. 0: below voltage threshold 1: above voltage threshold"]
    #[inline(always)]
    pub fn hvlvd1_ok(&self) -> HVLVD1_OK_R {
        HVLVD1_OK_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Low Voltage Detector (LVD) Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_lvd_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_LVD_STATUS_SPEC;
impl crate::RegisterSpec for PWR_LVD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_lvd_status::R`](R) reader structure"]
impl crate::Readable for PWR_LVD_STATUS_SPEC {}
#[doc = "`reset()` method sets PWR_LVD_STATUS to value 0"]
impl crate::Resettable for PWR_LVD_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
