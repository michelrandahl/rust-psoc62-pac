#[doc = "Register `DESCR_STATUS` reader"]
pub type R = crate::R<DESCR_STATUS_SPEC>;
#[doc = "Field `VALID` reader - Indicates whether the descriptor information present in DESCR_CTL, DESCR_SRC, DESCR_DST, DESCR_X_SIZE, DESCR_X_INCR, DESCR_Y_SIZE, DESCR_Y_INCR, DESCR_NEXT status registers is valid or not."]
pub type VALID_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - Indicates whether the descriptor information present in DESCR_CTL, DESCR_SRC, DESCR_DST, DESCR_X_SIZE, DESCR_X_INCR, DESCR_Y_SIZE, DESCR_Y_INCR, DESCR_NEXT status registers is valid or not."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel descriptor status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESCR_STATUS_SPEC;
impl crate::RegisterSpec for DESCR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_status::R`](R) reader structure"]
impl crate::Readable for DESCR_STATUS_SPEC {}
#[doc = "`reset()` method sets DESCR_STATUS to value 0"]
impl crate::Resettable for DESCR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
