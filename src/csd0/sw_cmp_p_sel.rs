#[doc = "Register `SW_CMP_P_SEL` reader"]
pub type R = crate::R<SW_CMP_P_SEL_SPEC>;
#[doc = "Register `SW_CMP_P_SEL` writer"]
pub type W = crate::W<SW_CMP_P_SEL_SPEC>;
#[doc = "Field `SW_SFPM` reader - Select waveform for corresponding switch"]
pub type SW_SFPM_R = crate::FieldReader;
#[doc = "Field `SW_SFPM` writer - Select waveform for corresponding switch"]
pub type SW_SFPM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_SFPT` reader - Select waveform for corresponding switch"]
pub type SW_SFPT_R = crate::FieldReader;
#[doc = "Field `SW_SFPT` writer - Select waveform for corresponding switch"]
pub type SW_SFPT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_SFPS` reader - Select waveform for corresponding switch"]
pub type SW_SFPS_R = crate::FieldReader;
#[doc = "Field `SW_SFPS` writer - Select waveform for corresponding switch"]
pub type SW_SFPS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_SFMA` reader - Set corresponding switch"]
pub type SW_SFMA_R = crate::BitReader;
#[doc = "Field `SW_SFMA` writer - Set corresponding switch"]
pub type SW_SFMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SFMB` reader - Set corresponding switch"]
pub type SW_SFMB_R = crate::BitReader;
#[doc = "Field `SW_SFMB` writer - Set corresponding switch"]
pub type SW_SFMB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SFCA` reader - Set corresponding switch"]
pub type SW_SFCA_R = crate::BitReader;
#[doc = "Field `SW_SFCA` writer - Set corresponding switch"]
pub type SW_SFCA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SFCB` reader - Set corresponding switch"]
pub type SW_SFCB_R = crate::BitReader;
#[doc = "Field `SW_SFCB` writer - Set corresponding switch"]
pub type SW_SFCB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpm(&self) -> SW_SFPM_R {
        SW_SFPM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpt(&self) -> SW_SFPT_R {
        SW_SFPT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfps(&self) -> SW_SFPS_R {
        SW_SFPS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfma(&self) -> SW_SFMA_R {
        SW_SFMA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfmb(&self) -> SW_SFMB_R {
        SW_SFMB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfca(&self) -> SW_SFCA_R {
        SW_SFCA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfcb(&self) -> SW_SFCB_R {
        SW_SFCB_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfpm(&mut self) -> SW_SFPM_W<SW_CMP_P_SEL_SPEC> {
        SW_SFPM_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfpt(&mut self) -> SW_SFPT_W<SW_CMP_P_SEL_SPEC> {
        SW_SFPT_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfps(&mut self) -> SW_SFPS_W<SW_CMP_P_SEL_SPEC> {
        SW_SFPS_W::new(self, 8)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfma(&mut self) -> SW_SFMA_W<SW_CMP_P_SEL_SPEC> {
        SW_SFMA_W::new(self, 12)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfmb(&mut self) -> SW_SFMB_W<SW_CMP_P_SEL_SPEC> {
        SW_SFMB_W::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfca(&mut self) -> SW_SFCA_W<SW_CMP_P_SEL_SPEC> {
        SW_SFCA_W::new(self, 20)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfcb(&mut self) -> SW_SFCB_W<SW_CMP_P_SEL_SPEC> {
        SW_SFCB_W::new(self, 24)
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
#[doc = "CSDCMP Pos Switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_cmp_p_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_cmp_p_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_CMP_P_SEL_SPEC;
impl crate::RegisterSpec for SW_CMP_P_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_cmp_p_sel::R`](R) reader structure"]
impl crate::Readable for SW_CMP_P_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_cmp_p_sel::W`](W) writer structure"]
impl crate::Writable for SW_CMP_P_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_CMP_P_SEL to value 0"]
impl crate::Resettable for SW_CMP_P_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
