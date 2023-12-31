#[doc = "Register `ACT_DESCR_NEXT_PTR` reader"]
pub type R = crate::R<ACT_DESCR_NEXT_PTR_SPEC>;
#[doc = "Field `ADDR` reader - Copy of DESCR_NEXT_PTR of the currently active descriptor."]
pub type ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 2:31 - Copy of DESCR_NEXT_PTR of the currently active descriptor."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[doc = "Active descriptor next pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`act_descr_next_ptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACT_DESCR_NEXT_PTR_SPEC;
impl crate::RegisterSpec for ACT_DESCR_NEXT_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_descr_next_ptr::R`](R) reader structure"]
impl crate::Readable for ACT_DESCR_NEXT_PTR_SPEC {}
#[doc = "`reset()` method sets ACT_DESCR_NEXT_PTR to value 0"]
impl crate::Resettable for ACT_DESCR_NEXT_PTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
