#[doc = "Register `CRC_CTL` reader"]
pub type R = crate::R<CRC_CTL_SPEC>;
#[doc = "Register `CRC_CTL` writer"]
pub type W = crate::W<CRC_CTL_SPEC>;
#[doc = "Field `DATA_REVERSE` reader - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
pub type DATA_REVERSE_R = crate::BitReader;
#[doc = "Field `DATA_REVERSE` writer - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
pub type DATA_REVERSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REM_REVERSE` reader - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
pub type REM_REVERSE_R = crate::BitReader;
#[doc = "Field `REM_REVERSE` writer - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
pub type REM_REVERSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
    #[inline(always)]
    pub fn data_reverse(&self) -> DATA_REVERSE_R {
        DATA_REVERSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
    #[inline(always)]
    pub fn rem_reverse(&self) -> REM_REVERSE_R {
        REM_REVERSE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
    #[inline(always)]
    #[must_use]
    pub fn data_reverse(&mut self) -> DATA_REVERSE_W<CRC_CTL_SPEC> {
        DATA_REVERSE_W::new(self, 0)
    }
    #[doc = "Bit 8 - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
    #[inline(always)]
    #[must_use]
    pub fn rem_reverse(&mut self) -> REM_REVERSE_W<CRC_CTL_SPEC> {
        REM_REVERSE_W::new(self, 8)
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
#[doc = "CRC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_CTL_SPEC;
impl crate::RegisterSpec for CRC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_ctl::R`](R) reader structure"]
impl crate::Readable for CRC_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crc_ctl::W`](W) writer structure"]
impl crate::Writable for CRC_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRC_CTL to value 0"]
impl crate::Resettable for CRC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
