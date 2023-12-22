#[doc = "Register `CM4_INT5_STATUS` reader"]
pub type R = crate::R<CM4_INT5_STATUS_SPEC>;
#[doc = "Field `SYSTEM_INT_IDX` reader - Lowest CM4 activated system interrupt index for CPU interrupt 5. See description of CM0_INT0_STATUS."]
pub type SYSTEM_INT_IDX_R = crate::FieldReader<u16>;
#[doc = "Field `SYSTEM_INT_VALID` reader - See description of CM0_INT0_STATUS."]
pub type SYSTEM_INT_VALID_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - Lowest CM4 activated system interrupt index for CPU interrupt 5. See description of CM0_INT0_STATUS."]
    #[inline(always)]
    pub fn system_int_idx(&self) -> SYSTEM_INT_IDX_R {
        SYSTEM_INT_IDX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - See description of CM0_INT0_STATUS."]
    #[inline(always)]
    pub fn system_int_valid(&self) -> SYSTEM_INT_VALID_R {
        SYSTEM_INT_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "CM4 interrupt 5 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_int5_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM4_INT5_STATUS_SPEC;
impl crate::RegisterSpec for CM4_INT5_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_int5_status::R`](R) reader structure"]
impl crate::Readable for CM4_INT5_STATUS_SPEC {}
#[doc = "`reset()` method sets CM4_INT5_STATUS to value 0"]
impl crate::Resettable for CM4_INT5_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
