#[doc = "Register `CRYPTO_KEY1` writer"]
pub type W = crate::W<CRYPTO_KEY1_SPEC>;
#[doc = "Field `KEY` writer - Four Bytes of the key KEY\\[63:32\\]
= CRYPTO_KEY1.KEY\\[31:0\\]."]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the key KEY\\[63:32\\]
= CRYPTO_KEY1.KEY\\[31:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CRYPTO_KEY1_SPEC> {
        KEY_W::new(self, 0)
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
#[doc = "Cryptography key 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_key1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYPTO_KEY1_SPEC;
impl crate::RegisterSpec for CRYPTO_KEY1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crypto_key1::W`](W) writer structure"]
impl crate::Writable for CRYPTO_KEY1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYPTO_KEY1 to value 0"]
impl crate::Resettable for CRYPTO_KEY1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
