#[doc = "Register `DESCR_NEXT` reader"]
pub type R = crate::R<DESCR_NEXT_SPEC>;
#[doc = "Field `PTR` reader - N/A"]
pub type PTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 2:31 - N/A"]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[doc = "Channel descriptor next pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_next::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESCR_NEXT_SPEC;
impl crate::RegisterSpec for DESCR_NEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_next::R`](R) reader structure"]
impl crate::Readable for DESCR_NEXT_SPEC {}
#[doc = "`reset()` method sets DESCR_NEXT to value 0"]
impl crate::Resettable for DESCR_NEXT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
