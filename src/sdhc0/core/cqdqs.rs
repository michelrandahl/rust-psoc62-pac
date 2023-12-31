#[doc = "Register `CQDQS` reader"]
pub type R = crate::R<CQDQS_SPEC>;
#[doc = "Field `DQS` reader - Device Queue Status Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Device has marked task N as ready for execution - Bit-N(0): Task-N is not ready for execution. This task could be pending in device or not submitted. Host controller updates this register with response of the Device Queue Status command."]
pub type DQS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Device Queue Status Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Device has marked task N as ready for execution - Bit-N(0): Task-N is not ready for execution. This task could be pending in device or not submitted. Host controller updates this register with response of the Device Queue Status command."]
    #[inline(always)]
    pub fn dqs(&self) -> DQS_R {
        DQS_R::new(self.bits)
    }
}
#[doc = "Device queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqdqs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQDQS_SPEC;
impl crate::RegisterSpec for CQDQS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqdqs::R`](R) reader structure"]
impl crate::Readable for CQDQS_SPEC {}
#[doc = "`reset()` method sets CQDQS to value 0"]
impl crate::Resettable for CQDQS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
