#[doc = "Register `SOF1` reader"]
pub type R = crate::R<SOF1_SPEC>;
#[doc = "Field `FRAME_NUMBER_MSB` reader - It has the upper 3 bits \\[10:8\\]
of the SOF frame number."]
pub type FRAME_NUMBER_MSB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - It has the upper 3 bits \\[10:8\\]
of the SOF frame number."]
    #[inline(always)]
    pub fn frame_number_msb(&self) -> FRAME_NUMBER_MSB_R {
        FRAME_NUMBER_MSB_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Start Of Frame Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sof1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOF1_SPEC;
impl crate::RegisterSpec for SOF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sof1::R`](R) reader structure"]
impl crate::Readable for SOF1_SPEC {}
#[doc = "`reset()` method sets SOF1 to value 0"]
impl crate::Resettable for SOF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
