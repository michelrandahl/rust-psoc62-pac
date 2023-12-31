#[doc = "Register `FM_SRAM_ECC_CTL0` reader"]
pub type R = crate::R<FM_SRAM_ECC_CTL0_SPEC>;
#[doc = "Register `FM_SRAM_ECC_CTL0` writer"]
pub type W = crate::W<FM_SRAM_ECC_CTL0_SPEC>;
#[doc = "Field `ECC_INJ_DATA` reader - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
pub type ECC_INJ_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `ECC_INJ_DATA` writer - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
pub type ECC_INJ_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    pub fn ecc_inj_data(&self) -> ECC_INJ_DATA_R {
        ECC_INJ_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_data(&mut self) -> ECC_INJ_DATA_W<FM_SRAM_ECC_CTL0_SPEC> {
        ECC_INJ_DATA_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "eCT Flash SRAM ECC control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_sram_ecc_ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm_sram_ecc_ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FM_SRAM_ECC_CTL0_SPEC;
impl crate::RegisterSpec for FM_SRAM_ECC_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_sram_ecc_ctl0::R`](R) reader structure"]
impl crate::Readable for FM_SRAM_ECC_CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fm_sram_ecc_ctl0::W`](W) writer structure"]
impl crate::Writable for FM_SRAM_ECC_CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FM_SRAM_ECC_CTL0 to value 0"]
impl crate::Resettable for FM_SRAM_ECC_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
