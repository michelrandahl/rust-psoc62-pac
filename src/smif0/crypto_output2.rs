#[doc = "Register `CRYPTO_OUTPUT2` reader"]
pub type R = crate::R<CRYPTO_OUTPUT2_SPEC>;
#[doc = "Register `CRYPTO_OUTPUT2` writer"]
pub type W = crate::W<CRYPTO_OUTPUT2_SPEC>;
#[doc = "Field `OUTPUT` reader - Four Bytes of the ciphertext CT\\[95:64\\]
= CRYPTO_OUTPUT2.OUTPUT\\[31:0\\]."]
pub type OUTPUT_R = crate::FieldReader<u32>;
#[doc = "Field `OUTPUT` writer - Four Bytes of the ciphertext CT\\[95:64\\]
= CRYPTO_OUTPUT2.OUTPUT\\[31:0\\]."]
pub type OUTPUT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Four Bytes of the ciphertext CT\\[95:64\\]
= CRYPTO_OUTPUT2.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn output(&self) -> OUTPUT_R {
        OUTPUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the ciphertext CT\\[95:64\\]
= CRYPTO_OUTPUT2.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn output(&mut self) -> OUTPUT_W<CRYPTO_OUTPUT2_SPEC> {
        OUTPUT_W::new(self, 0)
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
#[doc = "Cryptography output 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_output2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_output2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYPTO_OUTPUT2_SPEC;
impl crate::RegisterSpec for CRYPTO_OUTPUT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_output2::R`](R) reader structure"]
impl crate::Readable for CRYPTO_OUTPUT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crypto_output2::W`](W) writer structure"]
impl crate::Writable for CRYPTO_OUTPUT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYPTO_OUTPUT2 to value 0"]
impl crate::Resettable for CRYPTO_OUTPUT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
