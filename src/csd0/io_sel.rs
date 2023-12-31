#[doc = "Register `IO_SEL` reader"]
pub type R = crate::R<IO_SEL_SPEC>;
#[doc = "Register `IO_SEL` writer"]
pub type W = crate::W<IO_SEL_SPEC>;
#[doc = "Field `CSD_TX_OUT` reader - Select waveform for csd_tx_out output signal"]
pub type CSD_TX_OUT_R = crate::FieldReader;
#[doc = "Field `CSD_TX_OUT` writer - Select waveform for csd_tx_out output signal"]
pub type CSD_TX_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSD_TX_OUT_EN` reader - Select waveform for csd_tx_out_en output signal"]
pub type CSD_TX_OUT_EN_R = crate::FieldReader;
#[doc = "Field `CSD_TX_OUT_EN` writer - Select waveform for csd_tx_out_en output signal"]
pub type CSD_TX_OUT_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSD_TX_AMUXB_EN` reader - Select waveform for csd_tx_amuxb_en output signal"]
pub type CSD_TX_AMUXB_EN_R = crate::FieldReader;
#[doc = "Field `CSD_TX_AMUXB_EN` writer - Select waveform for csd_tx_amuxb_en output signal"]
pub type CSD_TX_AMUXB_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSD_TX_N_OUT` reader - Select waveform for csd_tx_n_out output signal"]
pub type CSD_TX_N_OUT_R = crate::FieldReader;
#[doc = "Field `CSD_TX_N_OUT` writer - Select waveform for csd_tx_n_out output signal"]
pub type CSD_TX_N_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSD_TX_N_OUT_EN` reader - Select waveform for csd_tx_n_out_en output signal"]
pub type CSD_TX_N_OUT_EN_R = crate::FieldReader;
#[doc = "Field `CSD_TX_N_OUT_EN` writer - Select waveform for csd_tx_n_out_en output signal"]
pub type CSD_TX_N_OUT_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSD_TX_N_AMUXA_EN` reader - Select waveform for csd_tx_n_amuxa_en output signal"]
pub type CSD_TX_N_AMUXA_EN_R = crate::FieldReader;
#[doc = "Field `CSD_TX_N_AMUXA_EN` writer - Select waveform for csd_tx_n_amuxa_en output signal"]
pub type CSD_TX_N_AMUXA_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Select waveform for csd_tx_out output signal"]
    #[inline(always)]
    pub fn csd_tx_out(&self) -> CSD_TX_OUT_R {
        CSD_TX_OUT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select waveform for csd_tx_out_en output signal"]
    #[inline(always)]
    pub fn csd_tx_out_en(&self) -> CSD_TX_OUT_EN_R {
        CSD_TX_OUT_EN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Select waveform for csd_tx_amuxb_en output signal"]
    #[inline(always)]
    pub fn csd_tx_amuxb_en(&self) -> CSD_TX_AMUXB_EN_R {
        CSD_TX_AMUXB_EN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Select waveform for csd_tx_n_out output signal"]
    #[inline(always)]
    pub fn csd_tx_n_out(&self) -> CSD_TX_N_OUT_R {
        CSD_TX_N_OUT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Select waveform for csd_tx_n_out_en output signal"]
    #[inline(always)]
    pub fn csd_tx_n_out_en(&self) -> CSD_TX_N_OUT_EN_R {
        CSD_TX_N_OUT_EN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Select waveform for csd_tx_n_amuxa_en output signal"]
    #[inline(always)]
    pub fn csd_tx_n_amuxa_en(&self) -> CSD_TX_N_AMUXA_EN_R {
        CSD_TX_N_AMUXA_EN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select waveform for csd_tx_out output signal"]
    #[inline(always)]
    #[must_use]
    pub fn csd_tx_out(&mut self) -> CSD_TX_OUT_W<IO_SEL_SPEC> {
        CSD_TX_OUT_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Select waveform for csd_tx_out_en output signal"]
    #[inline(always)]
    #[must_use]
    pub fn csd_tx_out_en(&mut self) -> CSD_TX_OUT_EN_W<IO_SEL_SPEC> {
        CSD_TX_OUT_EN_W::new(self, 4)
    }
    #[doc = "Bits 12:15 - Select waveform for csd_tx_amuxb_en output signal"]
    #[inline(always)]
    #[must_use]
    pub fn csd_tx_amuxb_en(&mut self) -> CSD_TX_AMUXB_EN_W<IO_SEL_SPEC> {
        CSD_TX_AMUXB_EN_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Select waveform for csd_tx_n_out output signal"]
    #[inline(always)]
    #[must_use]
    pub fn csd_tx_n_out(&mut self) -> CSD_TX_N_OUT_W<IO_SEL_SPEC> {
        CSD_TX_N_OUT_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Select waveform for csd_tx_n_out_en output signal"]
    #[inline(always)]
    #[must_use]
    pub fn csd_tx_n_out_en(&mut self) -> CSD_TX_N_OUT_EN_W<IO_SEL_SPEC> {
        CSD_TX_N_OUT_EN_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Select waveform for csd_tx_n_amuxa_en output signal"]
    #[inline(always)]
    #[must_use]
    pub fn csd_tx_n_amuxa_en(&mut self) -> CSD_TX_N_AMUXA_EN_W<IO_SEL_SPEC> {
        CSD_TX_N_AMUXA_EN_W::new(self, 24)
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
#[doc = "IO output control Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IO_SEL_SPEC;
impl crate::RegisterSpec for IO_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_sel::R`](R) reader structure"]
impl crate::Readable for IO_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`io_sel::W`](W) writer structure"]
impl crate::Writable for IO_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IO_SEL to value 0"]
impl crate::Resettable for IO_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
