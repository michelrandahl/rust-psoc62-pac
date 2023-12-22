#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `ECC_EN` reader - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type ECC_EN_R = crate::BitReader;
#[doc = "Field `ECC_EN` writer - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_INJ_EN` reader - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
pub type ECC_INJ_EN_R = crate::BitReader;
#[doc = "Field `ECC_INJ_EN` writer - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
pub type ECC_INJ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - IP enable: '0': Disabled. Disabling the IP activates the IP's Active logic reset: Active logic and non-retention MMIO registers are reset (retention MMIO registers are not affected). '1': Enabled."]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - IP enable: '0': Disabled. Disabling the IP activates the IP's Active logic reset: Active logic and non-retention MMIO registers are reset (retention MMIO registers are not affected). '1': Enabled."]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&self) -> ECC_EN_R {
        ECC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> ECC_INJ_EN_R {
        ECC_INJ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. Disabling the IP activates the IP's Active logic reset: Active logic and non-retention MMIO registers are reset (retention MMIO registers are not affected). '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> ECC_EN_W<CTL_SPEC> {
        ECC_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_en(&mut self) -> ECC_INJ_EN_W<CTL_SPEC> {
        ECC_INJ_EN_W::new(self, 1)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. Disabling the IP activates the IP's Active logic reset: Active logic and non-retention MMIO registers are reset (retention MMIO registers are not affected). '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<CTL_SPEC> {
        ENABLED_W::new(self, 31)
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
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x01"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
