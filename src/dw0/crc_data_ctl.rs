#[doc = "Register `CRC_DATA_CTL` reader"]
pub type R = crate::R<CRC_DATA_CTL_SPEC>;
#[doc = "Register `CRC_DATA_CTL` writer"]
pub type W = crate::W<CRC_DATA_CTL_SPEC>;
#[doc = "Field `DATA_XOR` reader - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
pub type DATA_XOR_R = crate::FieldReader;
#[doc = "Field `DATA_XOR` writer - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
pub type DATA_XOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
    #[inline(always)]
    pub fn data_xor(&self) -> DATA_XOR_R {
        DATA_XOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
    #[inline(always)]
    #[must_use]
    pub fn data_xor(&mut self) -> DATA_XOR_W<CRC_DATA_CTL_SPEC> {
        DATA_XOR_W::new(self, 0)
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
#[doc = "CRC data control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_data_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_data_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_DATA_CTL_SPEC;
impl crate::RegisterSpec for CRC_DATA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_data_ctl::R`](R) reader structure"]
impl crate::Readable for CRC_DATA_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crc_data_ctl::W`](W) writer structure"]
impl crate::Writable for CRC_DATA_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRC_DATA_CTL to value 0"]
impl crate::Resettable for CRC_DATA_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
