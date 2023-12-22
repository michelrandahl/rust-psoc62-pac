#[doc = "Register `CRYPTO_BUFF_CTL` reader"]
pub type R = crate::R<CRYPTO_BUFF_CTL_SPEC>;
#[doc = "Register `CRYPTO_BUFF_CTL` writer"]
pub type W = crate::W<CRYPTO_BUFF_CTL_SPEC>;
#[doc = "Field `PREF_EN` reader - Prefetch enable: 0: Disabled. 1: Enabled. A prefetch will be done when there is read 'hit' on the last 32-bit word of the buffer. For eCT work Flash, prefetch will not be done."]
pub type PREF_EN_R = crate::BitReader;
#[doc = "Field `PREF_EN` writer - Prefetch enable: 0: Disabled. 1: Enabled. A prefetch will be done when there is read 'hit' on the last 32-bit word of the buffer. For eCT work Flash, prefetch will not be done."]
pub type PREF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Prefetch enable: 0: Disabled. 1: Enabled. A prefetch will be done when there is read 'hit' on the last 32-bit word of the buffer. For eCT work Flash, prefetch will not be done."]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Prefetch enable: 0: Disabled. 1: Enabled. A prefetch will be done when there is read 'hit' on the last 32-bit word of the buffer. For eCT work Flash, prefetch will not be done."]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PREF_EN_W<CRYPTO_BUFF_CTL_SPEC> {
        PREF_EN_W::new(self, 30)
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
#[doc = "Cryptography buffer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_buff_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_buff_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYPTO_BUFF_CTL_SPEC;
impl crate::RegisterSpec for CRYPTO_BUFF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_buff_ctl::R`](R) reader structure"]
impl crate::Readable for CRYPTO_BUFF_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crypto_buff_ctl::W`](W) writer structure"]
impl crate::Writable for CRYPTO_BUFF_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYPTO_BUFF_CTL to value 0x4000_0000"]
impl crate::Resettable for CRYPTO_BUFF_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
