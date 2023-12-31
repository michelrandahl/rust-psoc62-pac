#[doc = "Register `SW_HS_P_SEL` reader"]
pub type R = crate::R<SW_HS_P_SEL_SPEC>;
#[doc = "Register `SW_HS_P_SEL` writer"]
pub type W = crate::W<SW_HS_P_SEL_SPEC>;
#[doc = "Field `SW_HMPM` reader - Set HMPM switch 0: static open 1: static closed"]
pub type SW_HMPM_R = crate::BitReader;
#[doc = "Field `SW_HMPM` writer - Set HMPM switch 0: static open 1: static closed"]
pub type SW_HMPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMPT` reader - Set corresponding switch"]
pub type SW_HMPT_R = crate::BitReader;
#[doc = "Field `SW_HMPT` writer - Set corresponding switch"]
pub type SW_HMPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMPS` reader - Set corresponding switch"]
pub type SW_HMPS_R = crate::BitReader;
#[doc = "Field `SW_HMPS` writer - Set corresponding switch"]
pub type SW_HMPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMMA` reader - Set corresponding switch"]
pub type SW_HMMA_R = crate::BitReader;
#[doc = "Field `SW_HMMA` writer - Set corresponding switch"]
pub type SW_HMMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMMB` reader - Set corresponding switch"]
pub type SW_HMMB_R = crate::BitReader;
#[doc = "Field `SW_HMMB` writer - Set corresponding switch"]
pub type SW_HMMB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMCA` reader - Set corresponding switch"]
pub type SW_HMCA_R = crate::BitReader;
#[doc = "Field `SW_HMCA` writer - Set corresponding switch"]
pub type SW_HMCA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMCB` reader - Set corresponding switch"]
pub type SW_HMCB_R = crate::BitReader;
#[doc = "Field `SW_HMCB` writer - Set corresponding switch"]
pub type SW_HMCB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMRH` reader - Set corresponding switch"]
pub type SW_HMRH_R = crate::BitReader;
#[doc = "Field `SW_HMRH` writer - Set corresponding switch"]
pub type SW_HMRH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set HMPM switch 0: static open 1: static closed"]
    #[inline(always)]
    pub fn sw_hmpm(&self) -> SW_HMPM_R {
        SW_HMPM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmpt(&self) -> SW_HMPT_R {
        SW_HMPT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmps(&self) -> SW_HMPS_R {
        SW_HMPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmma(&self) -> SW_HMMA_R {
        SW_HMMA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmmb(&self) -> SW_HMMB_R {
        SW_HMMB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmca(&self) -> SW_HMCA_R {
        SW_HMCA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmcb(&self) -> SW_HMCB_R {
        SW_HMCB_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmrh(&self) -> SW_HMRH_R {
        SW_HMRH_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set HMPM switch 0: static open 1: static closed"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmpm(&mut self) -> SW_HMPM_W<SW_HS_P_SEL_SPEC> {
        SW_HMPM_W::new(self, 0)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmpt(&mut self) -> SW_HMPT_W<SW_HS_P_SEL_SPEC> {
        SW_HMPT_W::new(self, 4)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmps(&mut self) -> SW_HMPS_W<SW_HS_P_SEL_SPEC> {
        SW_HMPS_W::new(self, 8)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmma(&mut self) -> SW_HMMA_W<SW_HS_P_SEL_SPEC> {
        SW_HMMA_W::new(self, 12)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmmb(&mut self) -> SW_HMMB_W<SW_HS_P_SEL_SPEC> {
        SW_HMMB_W::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmca(&mut self) -> SW_HMCA_W<SW_HS_P_SEL_SPEC> {
        SW_HMCA_W::new(self, 20)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmcb(&mut self) -> SW_HMCB_W<SW_HS_P_SEL_SPEC> {
        SW_HMCB_W::new(self, 24)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmrh(&mut self) -> SW_HMRH_W<SW_HS_P_SEL_SPEC> {
        SW_HMRH_W::new(self, 28)
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
#[doc = "HSCMP Pos input switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_hs_p_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_hs_p_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_HS_P_SEL_SPEC;
impl crate::RegisterSpec for SW_HS_P_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_hs_p_sel::R`](R) reader structure"]
impl crate::Readable for SW_HS_P_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_hs_p_sel::W`](W) writer structure"]
impl crate::Writable for SW_HS_P_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_HS_P_SEL to value 0"]
impl crate::Resettable for SW_HS_P_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
