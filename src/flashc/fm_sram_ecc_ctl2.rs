#[doc = "Register `FM_SRAM_ECC_CTL2` reader"]
pub type R = crate::R<FM_SRAM_ECC_CTL2_SPEC>;
#[doc = "Field `CORRECTED_DATA` reader - 32-bit corrected data output of the ECC syndrome logic."]
pub type CORRECTED_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit corrected data output of the ECC syndrome logic."]
    #[inline(always)]
    pub fn corrected_data(&self) -> CORRECTED_DATA_R {
        CORRECTED_DATA_R::new(self.bits)
    }
}
#[doc = "eCT Flash SRAM ECC control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_sram_ecc_ctl2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FM_SRAM_ECC_CTL2_SPEC;
impl crate::RegisterSpec for FM_SRAM_ECC_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_sram_ecc_ctl2::R`](R) reader structure"]
impl crate::Readable for FM_SRAM_ECC_CTL2_SPEC {}
#[doc = "`reset()` method sets FM_SRAM_ECC_CTL2 to value 0"]
impl crate::Resettable for FM_SRAM_ECC_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
