#[doc = "Register `SW_AMUXBUF_SEL` reader"]
pub type R = crate::R<SW_AMUXBUF_SEL_SPEC>;
#[doc = "Register `SW_AMUXBUF_SEL` writer"]
pub type W = crate::W<SW_AMUXBUF_SEL_SPEC>;
#[doc = "Field `SW_IRBY` reader - Set corresponding switch"]
pub type SW_IRBY_R = crate::BitReader;
#[doc = "Field `SW_IRBY` writer - Set corresponding switch"]
pub type SW_IRBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_IRLB` reader - Set corresponding switch"]
pub type SW_IRLB_R = crate::BitReader;
#[doc = "Field `SW_IRLB` writer - Set corresponding switch"]
pub type SW_IRLB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_ICA` reader - Set corresponding switch"]
pub type SW_ICA_R = crate::BitReader;
#[doc = "Field `SW_ICA` writer - Set corresponding switch"]
pub type SW_ICA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_ICB` reader - Select waveform for corresponding switch"]
pub type SW_ICB_R = crate::FieldReader;
#[doc = "Field `SW_ICB` writer - Select waveform for corresponding switch"]
pub type SW_ICB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_IRLI` reader - Set corresponding switch"]
pub type SW_IRLI_R = crate::BitReader;
#[doc = "Field `SW_IRLI` writer - Set corresponding switch"]
pub type SW_IRLI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_IRH` reader - Set corresponding switch"]
pub type SW_IRH_R = crate::BitReader;
#[doc = "Field `SW_IRH` writer - Set corresponding switch"]
pub type SW_IRH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_IRL` reader - Set corresponding switch"]
pub type SW_IRL_R = crate::BitReader;
#[doc = "Field `SW_IRL` writer - Set corresponding switch"]
pub type SW_IRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irby(&self) -> SW_IRBY_R {
        SW_IRBY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irlb(&self) -> SW_IRLB_R {
        SW_IRLB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ica(&self) -> SW_ICA_R {
        SW_ICA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_icb(&self) -> SW_ICB_R {
        SW_ICB_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irli(&self) -> SW_IRLI_R {
        SW_IRLI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irh(&self) -> SW_IRH_R {
        SW_IRH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irl(&self) -> SW_IRL_R {
        SW_IRL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_irby(&mut self) -> SW_IRBY_W<SW_AMUXBUF_SEL_SPEC> {
        SW_IRBY_W::new(self, 4)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_irlb(&mut self) -> SW_IRLB_W<SW_AMUXBUF_SEL_SPEC> {
        SW_IRLB_W::new(self, 8)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ica(&mut self) -> SW_ICA_W<SW_AMUXBUF_SEL_SPEC> {
        SW_ICA_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_icb(&mut self) -> SW_ICB_W<SW_AMUXBUF_SEL_SPEC> {
        SW_ICB_W::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_irli(&mut self) -> SW_IRLI_W<SW_AMUXBUF_SEL_SPEC> {
        SW_IRLI_W::new(self, 20)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_irh(&mut self) -> SW_IRH_W<SW_AMUXBUF_SEL_SPEC> {
        SW_IRH_W::new(self, 24)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_irl(&mut self) -> SW_IRL_W<SW_AMUXBUF_SEL_SPEC> {
        SW_IRL_W::new(self, 28)
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
#[doc = "Amuxbuffer switches Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_amuxbuf_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_amuxbuf_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_AMUXBUF_SEL_SPEC;
impl crate::RegisterSpec for SW_AMUXBUF_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_amuxbuf_sel::R`](R) reader structure"]
impl crate::Readable for SW_AMUXBUF_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_amuxbuf_sel::W`](W) writer structure"]
impl crate::Writable for SW_AMUXBUF_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_AMUXBUF_SEL to value 0"]
impl crate::Resettable for SW_AMUXBUF_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
