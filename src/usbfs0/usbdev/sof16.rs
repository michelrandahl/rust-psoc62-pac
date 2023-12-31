#[doc = "Register `SOF16` reader"]
pub type R = crate::R<SOF16_SPEC>;
#[doc = "Field `FRAME_NUMBER16` reader - The frame number (11b)"]
pub type FRAME_NUMBER16_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - The frame number (11b)"]
    #[inline(always)]
    pub fn frame_number16(&self) -> FRAME_NUMBER16_R {
        FRAME_NUMBER16_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "Start Of Frame Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sof16::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOF16_SPEC;
impl crate::RegisterSpec for SOF16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sof16::R`](R) reader structure"]
impl crate::Readable for SOF16_SPEC {}
#[doc = "`reset()` method sets SOF16 to value 0"]
impl crate::Resettable for SOF16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
