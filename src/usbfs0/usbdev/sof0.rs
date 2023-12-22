#[doc = "Register `SOF0` reader"]
pub type R = crate::R<SOF0_SPEC>;
#[doc = "Field `FRAME_NUMBER` reader - It has the lower 8 bits \\[7:0\\]
of the SOF frame number."]
pub type FRAME_NUMBER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It has the lower 8 bits \\[7:0\\]
of the SOF frame number."]
    #[inline(always)]
    pub fn frame_number(&self) -> FRAME_NUMBER_R {
        FRAME_NUMBER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Start Of Frame Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sof0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOF0_SPEC;
impl crate::RegisterSpec for SOF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sof0::R`](R) reader structure"]
impl crate::Readable for SOF0_SPEC {}
#[doc = "`reset()` method sets SOF0 to value 0"]
impl crate::Resettable for SOF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
