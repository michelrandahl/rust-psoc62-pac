#[doc = "Register `CRYPTO_INPUT3` reader"]
pub type R = crate::R<CRYPTO_INPUT3_SPEC>;
#[doc = "Register `CRYPTO_INPUT3` writer"]
pub type W = crate::W<CRYPTO_INPUT3_SPEC>;
#[doc = "Field `INPUT` reader - Four Bytes of the plaintext PT\\[127:96\\]
= CRYPTO_INPUT3.INPUT\\[31:0\\]."]
pub type INPUT_R = crate::FieldReader<u32>;
#[doc = "Field `INPUT` writer - Four Bytes of the plaintext PT\\[127:96\\]
= CRYPTO_INPUT3.INPUT\\[31:0\\]."]
pub type INPUT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Four Bytes of the plaintext PT\\[127:96\\]
= CRYPTO_INPUT3.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the plaintext PT\\[127:96\\]
= CRYPTO_INPUT3.INPUT\\[31:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn input(&mut self) -> INPUT_W<CRYPTO_INPUT3_SPEC> {
        INPUT_W::new(self, 0)
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
#[doc = "Cryptography input 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_input3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_input3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYPTO_INPUT3_SPEC;
impl crate::RegisterSpec for CRYPTO_INPUT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_input3::R`](R) reader structure"]
impl crate::Readable for CRYPTO_INPUT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crypto_input3::W`](W) writer structure"]
impl crate::Writable for CRYPTO_INPUT3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYPTO_INPUT3 to value 0"]
impl crate::Resettable for CRYPTO_INPUT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
