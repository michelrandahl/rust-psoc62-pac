#[doc = "Register `SW_HS_N_SEL` reader"]
pub type R = crate::R<SW_HS_N_SEL_SPEC>;
#[doc = "Register `SW_HS_N_SEL` writer"]
pub type W = crate::W<SW_HS_N_SEL_SPEC>;
#[doc = "Field `SW_HCCC` reader - Set corresponding switch"]
pub type SW_HCCC_R = crate::BitReader;
#[doc = "Field `SW_HCCC` writer - Set corresponding switch"]
pub type SW_HCCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HCCD` reader - Set corresponding switch"]
pub type SW_HCCD_R = crate::BitReader;
#[doc = "Field `SW_HCCD` writer - Set corresponding switch"]
pub type SW_HCCD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HCRH` reader - Select waveform for corresponding switch"]
pub type SW_HCRH_R = crate::FieldReader;
#[doc = "Field `SW_HCRH` writer - Select waveform for corresponding switch"]
pub type SW_HCRH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_HCRL` reader - Select waveform for corresponding switch"]
pub type SW_HCRL_R = crate::FieldReader;
#[doc = "Field `SW_HCRL` writer - Select waveform for corresponding switch"]
pub type SW_HCRL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccc(&self) -> SW_HCCC_R {
        SW_HCCC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccd(&self) -> SW_HCCD_R {
        SW_HCCD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrh(&self) -> SW_HCRH_R {
        SW_HCRH_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrl(&self) -> SW_HCRL_R {
        SW_HCRL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hccc(&mut self) -> SW_HCCC_W<SW_HS_N_SEL_SPEC> {
        SW_HCCC_W::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hccd(&mut self) -> SW_HCCD_W<SW_HS_N_SEL_SPEC> {
        SW_HCCD_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hcrh(&mut self) -> SW_HCRH_W<SW_HS_N_SEL_SPEC> {
        SW_HCRH_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hcrl(&mut self) -> SW_HCRL_W<SW_HS_N_SEL_SPEC> {
        SW_HCRL_W::new(self, 28)
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
#[doc = "HSCMP Neg input switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_hs_n_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_hs_n_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_HS_N_SEL_SPEC;
impl crate::RegisterSpec for SW_HS_N_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_hs_n_sel::R`](R) reader structure"]
impl crate::Readable for SW_HS_N_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_hs_n_sel::W`](W) writer structure"]
impl crate::Writable for SW_HS_N_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_HS_N_SEL to value 0"]
impl crate::Resettable for SW_HS_N_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
