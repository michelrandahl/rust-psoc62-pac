#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `CSD_SENSE` reader - Only for Debug/test purpose this internal signal (sensor clock) status can be read by CPU"]
pub type CSD_SENSE_R = crate::BitReader;
#[doc = "Field `HSCMP_OUT` reader - Only for Debug/test purpose the output status of CSD comparator can be read by CPU"]
pub type HSCMP_OUT_R = crate::BitReader<HSCMP_OUT_A>;
#[doc = "Only for Debug/test purpose the output status of CSD comparator can be read by CPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSCMP_OUT_A {
    #[doc = "0: N/A"]
    C_LT_VREF = 0,
    #[doc = "1: N/A"]
    C_GT_VREF = 1,
}
impl From<HSCMP_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: HSCMP_OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl HSCMP_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSCMP_OUT_A {
        match self.bits {
            false => HSCMP_OUT_A::C_LT_VREF,
            true => HSCMP_OUT_A::C_GT_VREF,
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_c_lt_vref(&self) -> bool {
        *self == HSCMP_OUT_A::C_LT_VREF
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_c_gt_vref(&self) -> bool {
        *self == HSCMP_OUT_A::C_GT_VREF
    }
}
#[doc = "Field `CSDCMP_OUT` reader - Only for Debug/test purpose the output status of CSD modulator can be read by CPU"]
pub type CSDCMP_OUT_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Only for Debug/test purpose this internal signal (sensor clock) status can be read by CPU"]
    #[inline(always)]
    pub fn csd_sense(&self) -> CSD_SENSE_R {
        CSD_SENSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Only for Debug/test purpose the output status of CSD comparator can be read by CPU"]
    #[inline(always)]
    pub fn hscmp_out(&self) -> HSCMP_OUT_R {
        HSCMP_OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Only for Debug/test purpose the output status of CSD modulator can be read by CPU"]
    #[inline(always)]
    pub fn csdcmp_out(&self) -> CSDCMP_OUT_R {
        CSDCMP_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
