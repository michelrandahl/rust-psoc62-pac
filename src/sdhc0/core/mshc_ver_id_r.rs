#[doc = "Register `MSHC_VER_ID_R` reader"]
pub type R = crate::R<MSHC_VER_ID_R_SPEC>;
#[doc = "Field `MSHC_VER_ID` reader - Current release number This field indicates the Synopsys DesignWare Cores DWC_mshc/DWC_mshc_lite current release number that is read by an application. For example, release number '1.60a' is represented in ASCII as 0x313630. Lower 8 bits read from this register can be ignored by the application. An application reading this register in conjunction with the MSHC_VER_TYPE_R register, gathers details of the current release."]
pub type MSHC_VER_ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current release number This field indicates the Synopsys DesignWare Cores DWC_mshc/DWC_mshc_lite current release number that is read by an application. For example, release number '1.60a' is represented in ASCII as 0x313630. Lower 8 bits read from this register can be ignored by the application. An application reading this register in conjunction with the MSHC_VER_TYPE_R register, gathers details of the current release."]
    #[inline(always)]
    pub fn mshc_ver_id(&self) -> MSHC_VER_ID_R {
        MSHC_VER_ID_R::new(self.bits)
    }
}
#[doc = "MSHC version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mshc_ver_id_r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSHC_VER_ID_R_SPEC;
impl crate::RegisterSpec for MSHC_VER_ID_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mshc_ver_id_r::R`](R) reader structure"]
impl crate::Readable for MSHC_VER_ID_R_SPEC {}
#[doc = "`reset()` method sets MSHC_VER_ID_R to value 0x3137_302a"]
impl crate::Resettable for MSHC_VER_ID_R_SPEC {
    const RESET_VALUE: Self::Ux = 0x3137_302a;
}
