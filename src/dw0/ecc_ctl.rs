#[doc = "Register `ECC_CTL` reader"]
pub type R = crate::R<ECC_CTL_SPEC>;
#[doc = "Register `ECC_CTL` writer"]
pub type W = crate::W<ECC_CTL_SPEC>;
#[doc = "Field `WORD_ADDR` reader - Specifies the word address where an error will be injected. - On a write transfer to this SRAM word address and when CTL.ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
pub type WORD_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `WORD_ADDR` writer - Specifies the word address where an error will be injected. - On a write transfer to this SRAM word address and when CTL.ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
pub type WORD_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PARITY` reader - ECC parity to use for ECC error injection at address WORD_ADDR."]
pub type PARITY_R = crate::FieldReader;
#[doc = "Field `PARITY` writer - ECC parity to use for ECC error injection at address WORD_ADDR."]
pub type PARITY_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:9 - Specifies the word address where an error will be injected. - On a write transfer to this SRAM word address and when CTL.ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    pub fn word_addr(&self) -> WORD_ADDR_R {
        WORD_ADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 25:31 - ECC parity to use for ECC error injection at address WORD_ADDR."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Specifies the word address where an error will be injected. - On a write transfer to this SRAM word address and when CTL.ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    #[must_use]
    pub fn word_addr(&mut self) -> WORD_ADDR_W<ECC_CTL_SPEC> {
        WORD_ADDR_W::new(self, 0)
    }
    #[doc = "Bits 25:31 - ECC parity to use for ECC error injection at address WORD_ADDR."]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<ECC_CTL_SPEC> {
        PARITY_W::new(self, 25)
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
#[doc = "`reset()` method sets ECC_CTL to value 0"]
impl crate::Resettable for ECC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
