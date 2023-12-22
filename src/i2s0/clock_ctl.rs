#[doc = "Register `CLOCK_CTL` reader"]
pub type R = crate::R<CLOCK_CTL_SPEC>;
#[doc = "Register `CLOCK_CTL` writer"]
pub type W = crate::W<CLOCK_CTL_SPEC>;
#[doc = "Field `CLOCK_DIV` reader - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
pub type CLOCK_DIV_R = crate::FieldReader;
#[doc = "Field `CLOCK_DIV` writer - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
pub type CLOCK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLOCK_SEL` reader - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
pub type CLOCK_SEL_R = crate::BitReader;
#[doc = "Field `CLOCK_SEL` writer - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
pub type CLOCK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
    #[inline(always)]
    pub fn clock_div(&self) -> CLOCK_DIV_R {
        CLOCK_DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
    #[inline(always)]
    #[must_use]
    pub fn clock_div(&mut self) -> CLOCK_DIV_W<CLOCK_CTL_SPEC> {
        CLOCK_DIV_W::new(self, 0)
    }
    #[doc = "Bit 8 - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
    #[inline(always)]
    #[must_use]
    pub fn clock_sel(&mut self) -> CLOCK_SEL_W<CLOCK_CTL_SPEC> {
        CLOCK_SEL_W::new(self, 8)
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
#[doc = "Clock control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCK_CTL_SPEC;
impl crate::RegisterSpec for CLOCK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_ctl::R`](R) reader structure"]
impl crate::Readable for CLOCK_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clock_ctl::W`](W) writer structure"]
impl crate::Writable for CLOCK_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCK_CTL to value 0"]
impl crate::Resettable for CLOCK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
