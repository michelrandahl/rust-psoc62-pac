#[doc = "Register `DESCR_CTL` reader"]
pub type R = crate::R<DESCR_CTL_SPEC>;
#[doc = "Field `WAIT_FOR_DEACT` reader - N/A"]
pub type WAIT_FOR_DEACT_R = crate::FieldReader;
#[doc = "Field `INTR_TYPE` reader - N/A"]
pub type INTR_TYPE_R = crate::FieldReader;
#[doc = "Field `TR_OUT_TYPE` reader - N/A"]
pub type TR_OUT_TYPE_R = crate::FieldReader;
#[doc = "Field `TR_IN_TYPE` reader - N/A"]
pub type TR_IN_TYPE_R = crate::FieldReader;
#[doc = "Field `DATA_PREFETCH` reader - N/A"]
pub type DATA_PREFETCH_R = crate::BitReader;
#[doc = "Field `DATA_SIZE` reader - N/A"]
pub type DATA_SIZE_R = crate::FieldReader;
#[doc = "Field `CH_DISABLE` reader - N/A"]
pub type CH_DISABLE_R = crate::BitReader;
#[doc = "Field `SRC_TRANSFER_SIZE` reader - N/A"]
pub type SRC_TRANSFER_SIZE_R = crate::BitReader;
#[doc = "Field `DST_TRANSFER_SIZE` reader - N/A"]
pub type DST_TRANSFER_SIZE_R = crate::BitReader;
#[doc = "Field `DESCR_TYPE` reader - N/A"]
pub type DESCR_TYPE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn wait_for_deact(&self) -> WAIT_FOR_DEACT_R {
        WAIT_FOR_DEACT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - N/A"]
    #[inline(always)]
    pub fn intr_type(&self) -> INTR_TYPE_R {
        INTR_TYPE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - N/A"]
    #[inline(always)]
    pub fn tr_out_type(&self) -> TR_OUT_TYPE_R {
        TR_OUT_TYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - N/A"]
    #[inline(always)]
    pub fn tr_in_type(&self) -> TR_IN_TYPE_R {
        TR_IN_TYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn data_prefetch(&self) -> DATA_PREFETCH_R {
        DATA_PREFETCH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - N/A"]
    #[inline(always)]
    pub fn data_size(&self) -> DATA_SIZE_R {
        DATA_SIZE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn ch_disable(&self) -> CH_DISABLE_R {
        CH_DISABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn src_transfer_size(&self) -> SRC_TRANSFER_SIZE_R {
        SRC_TRANSFER_SIZE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - N/A"]
    #[inline(always)]
    pub fn dst_transfer_size(&self) -> DST_TRANSFER_SIZE_R {
        DST_TRANSFER_SIZE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - N/A"]
    #[inline(always)]
    pub fn descr_type(&self) -> DESCR_TYPE_R {
        DESCR_TYPE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Channel descriptor control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_ctl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESCR_CTL_SPEC;
impl crate::RegisterSpec for DESCR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_ctl::R`](R) reader structure"]
impl crate::Readable for DESCR_CTL_SPEC {}
#[doc = "`reset()` method sets DESCR_CTL to value 0"]
impl crate::Resettable for DESCR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
