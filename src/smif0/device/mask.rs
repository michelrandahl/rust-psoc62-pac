#[doc = "Register `MASK` reader"]
pub type R = crate::R<MASK_SPEC>;
#[doc = "Register `MASK` writer"]
pub type W = crate::W<MASK_SPEC>;
#[doc = "Field `MASK` reader - Specifies the size of the device region. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\]
with the address as specified in ADDR.ADDR: Address A is in the device when (A\\[31:8\\]
&amp; MASK\\[31:8\\]) == ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\]
= 0xff. Note: a transfer request that is not in any device region results in an AHB-Lite bus error."]
pub type MASK_R = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - Specifies the size of the device region. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\]
with the address as specified in ADDR.ADDR: Address A is in the device when (A\\[31:8\\]
&amp; MASK\\[31:8\\]) == ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\]
= 0xff. Note: a transfer request that is not in any device region results in an AHB-Lite bus error."]
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - Specifies the size of the device region. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\]
with the address as specified in ADDR.ADDR: Address A is in the device when (A\\[31:8\\]
&amp; MASK\\[31:8\\]) == ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\]
= 0xff. Note: a transfer request that is not in any device region results in an AHB-Lite bus error."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Specifies the size of the device region. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\]
with the address as specified in ADDR.ADDR: Address A is in the device when (A\\[31:8\\]
&amp; MASK\\[31:8\\]) == ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\]
= 0xff. Note: a transfer request that is not in any device region results in an AHB-Lite bus error."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<MASK_SPEC> {
        MASK_W::new(self, 8)
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
#[doc = "Device region mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask::R`](R) reader structure"]
impl crate::Readable for MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mask::W`](W) writer structure"]
impl crate::Writable for MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::Resettable for MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
