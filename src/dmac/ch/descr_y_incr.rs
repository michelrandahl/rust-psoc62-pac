#[doc = "Register `DESCR_Y_INCR` reader"]
pub type R = crate::R<DESCR_Y_INCR_SPEC>;
#[doc = "Field `SRC_Y` reader - N/A"]
pub type SRC_Y_R = crate::FieldReader<u16>;
#[doc = "Field `DST_Y` reader - N/A"]
pub type DST_Y_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn src_y(&self) -> SRC_Y_R {
        SRC_Y_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn dst_y(&self) -> DST_Y_R {
        DST_Y_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Channel descriptor Y increment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_y_incr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESCR_Y_INCR_SPEC;
impl crate::RegisterSpec for DESCR_Y_INCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_y_incr::R`](R) reader structure"]
impl crate::Readable for DESCR_Y_INCR_SPEC {}
#[doc = "`reset()` method sets DESCR_Y_INCR to value 0"]
impl crate::Resettable for DESCR_Y_INCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
