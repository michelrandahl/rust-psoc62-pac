#[doc = "Register `SW_CMP_N_SEL` reader"]
pub type R = crate::R<SW_CMP_N_SEL_SPEC>;
#[doc = "Register `SW_CMP_N_SEL` writer"]
pub type W = crate::W<SW_CMP_N_SEL_SPEC>;
#[doc = "Field `SW_SCRH` reader - Select waveform for corresponding switch"]
pub type SW_SCRH_R = crate::FieldReader;
#[doc = "Field `SW_SCRH` writer - Select waveform for corresponding switch"]
pub type SW_SCRH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_SCRL` reader - Select waveform for corresponding switch"]
pub type SW_SCRL_R = crate::FieldReader;
#[doc = "Field `SW_SCRL` writer - Select waveform for corresponding switch"]
pub type SW_SCRL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_scrh(&self) -> SW_SCRH_R {
        SW_SCRH_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_scrl(&self) -> SW_SCRL_R {
        SW_SCRL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_scrh(&mut self) -> SW_SCRH_W<SW_CMP_N_SEL_SPEC> {
        SW_SCRH_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_scrl(&mut self) -> SW_SCRL_W<SW_CMP_N_SEL_SPEC> {
        SW_SCRL_W::new(self, 28)
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
#[doc = "CSDCMP Neg Switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_cmp_n_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_cmp_n_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_CMP_N_SEL_SPEC;
impl crate::RegisterSpec for SW_CMP_N_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_cmp_n_sel::R`](R) reader structure"]
impl crate::Readable for SW_CMP_N_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_cmp_n_sel::W`](W) writer structure"]
impl crate::Writable for SW_CMP_N_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_CMP_N_SEL to value 0"]
impl crate::Resettable for SW_CMP_N_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
