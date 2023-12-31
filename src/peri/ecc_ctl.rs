#[doc = "Register `ECC_CTL` reader"]
pub type R = crate::R<ECC_CTL_SPEC>;
#[doc = "Register `ECC_CTL` writer"]
pub type W = crate::W<ECC_CTL_SPEC>;
#[doc = "Field `WORD_ADDR` reader - Specifies the word address where the parity is injected. - On a 32-bit write access to this SRAM address and when ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
pub type WORD_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `WORD_ADDR` writer - Specifies the word address where the parity is injected. - On a 32-bit write access to this SRAM address and when ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
pub type WORD_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ECC_EN` reader - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type ECC_EN_R = crate::BitReader;
#[doc = "Field `ECC_EN` writer - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_INJ_EN` reader - Enable error injection for PERI protection structure SRAM. When '1', the parity (PARITY) is used when a write is done to the WORD_ADDR word address of the SRAM."]
pub type ECC_INJ_EN_R = crate::BitReader;
#[doc = "Field `ECC_INJ_EN` writer - Enable error injection for PERI protection structure SRAM. When '1', the parity (PARITY) is used when a write is done to the WORD_ADDR word address of the SRAM."]
pub type ECC_INJ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY` reader - ECC parity to use for ECC error injection at address WORD_ADDR."]
pub type PARITY_R = crate::FieldReader;
#[doc = "Field `PARITY` writer - ECC parity to use for ECC error injection at address WORD_ADDR."]
pub type PARITY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:10 - Specifies the word address where the parity is injected. - On a 32-bit write access to this SRAM address and when ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    pub fn word_addr(&self) -> WORD_ADDR_R {
        WORD_ADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&self) -> ECC_EN_R {
        ECC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable error injection for PERI protection structure SRAM. When '1', the parity (PARITY) is used when a write is done to the WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> ECC_INJ_EN_R {
        ECC_INJ_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:31 - ECC parity to use for ECC error injection at address WORD_ADDR."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Specifies the word address where the parity is injected. - On a 32-bit write access to this SRAM address and when ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    #[must_use]
    pub fn word_addr(&mut self) -> WORD_ADDR_W<ECC_CTL_SPEC> {
        WORD_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> ECC_EN_W<ECC_CTL_SPEC> {
        ECC_EN_W::new(self, 16)
    }
    #[doc = "Bit 18 - Enable error injection for PERI protection structure SRAM. When '1', the parity (PARITY) is used when a write is done to the WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_en(&mut self) -> ECC_INJ_EN_W<ECC_CTL_SPEC> {
        ECC_INJ_EN_W::new(self, 18)
    }
    #[doc = "Bits 24:31 - ECC parity to use for ECC error injection at address WORD_ADDR."]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<ECC_CTL_SPEC> {
        PARITY_W::new(self, 24)
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
#[doc = "ECC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECC_CTL_SPEC;
impl crate::RegisterSpec for ECC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_ctl::R`](R) reader structure"]
impl crate::Readable for ECC_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecc_ctl::W`](W) writer structure"]
impl crate::Writable for ECC_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC_CTL to value 0x0001_0000"]
impl crate::Resettable for ECC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
