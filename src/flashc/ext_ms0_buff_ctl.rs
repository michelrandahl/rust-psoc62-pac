#[doc = "Register `EXT_MS0_BUFF_CTL` reader"]
pub type R = crate::R<EXT_MS0_BUFF_CTL_SPEC>;
#[doc = "Register `EXT_MS0_BUFF_CTL` writer"]
pub type W = crate::W<EXT_MS0_BUFF_CTL_SPEC>;
#[doc = "Field `PREF_EN` reader - See CRYPTO_BUFF_CTL."]
pub type PREF_EN_R = crate::BitReader;
#[doc = "Field `PREF_EN` writer - See CRYPTO_BUFF_CTL."]
pub type PREF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PREF_EN_W<EXT_MS0_BUFF_CTL_SPEC> {
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
#[doc = "External master 0 buffer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_ms0_buff_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ms0_buff_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_MS0_BUFF_CTL_SPEC;
impl crate::RegisterSpec for EXT_MS0_BUFF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_ms0_buff_ctl::R`](R) reader structure"]
impl crate::Readable for EXT_MS0_BUFF_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_ms0_buff_ctl::W`](W) writer structure"]
impl crate::Writable for EXT_MS0_BUFF_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_MS0_BUFF_CTL to value 0x4000_0000"]
impl crate::Resettable for EXT_MS0_BUFF_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
