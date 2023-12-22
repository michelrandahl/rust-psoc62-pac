#[doc = "Register `CRC_REM_CTL` reader"]
pub type R = crate::R<CRC_REM_CTL_SPEC>;
#[doc = "Register `CRC_REM_CTL` writer"]
pub type W = crate::W<CRC_REM_CTL_SPEC>;
#[doc = "Field `REM_XOR` reader - Specifies a mask with which the CRC_LFSR_CTL.LFSR32 register is XOR'd to produce a remainder. The XOR is performed before remainder reversal."]
pub type REM_XOR_R = crate::FieldReader<u32>;
#[doc = "Field `REM_XOR` writer - Specifies a mask with which the CRC_LFSR_CTL.LFSR32 register is XOR'd to produce a remainder. The XOR is performed before remainder reversal."]
pub type REM_XOR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies a mask with which the CRC_LFSR_CTL.LFSR32 register is XOR'd to produce a remainder. The XOR is performed before remainder reversal."]
    #[inline(always)]
    pub fn rem_xor(&self) -> REM_XOR_R {
        REM_XOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies a mask with which the CRC_LFSR_CTL.LFSR32 register is XOR'd to produce a remainder. The XOR is performed before remainder reversal."]
    #[inline(always)]
    #[must_use]
    pub fn rem_xor(&mut self) -> REM_XOR_W<CRC_REM_CTL_SPEC> {
        REM_XOR_W::new(self, 0)
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
#[doc = "CRC remainder control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_rem_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_rem_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_REM_CTL_SPEC;
impl crate::RegisterSpec for CRC_REM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_rem_ctl::R`](R) reader structure"]
impl crate::Readable for CRC_REM_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crc_rem_ctl::W`](W) writer structure"]
impl crate::Writable for CRC_REM_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRC_REM_CTL to value 0"]
impl crate::Resettable for CRC_REM_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
