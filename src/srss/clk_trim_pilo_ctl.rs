#[doc = "Register `CLK_TRIM_PILO_CTL` reader"]
pub type R = crate::R<CLK_TRIM_PILO_CTL_SPEC>;
#[doc = "Register `CLK_TRIM_PILO_CTL` writer"]
pub type W = crate::W<CLK_TRIM_PILO_CTL_SPEC>;
#[doc = "Field `PILO_CFREQ` reader - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
pub type PILO_CFREQ_R = crate::FieldReader;
#[doc = "Field `PILO_CFREQ` writer - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
pub type PILO_CFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PILO_OSC_TRIM` reader - Trim for current in oscillator block."]
pub type PILO_OSC_TRIM_R = crate::FieldReader;
#[doc = "Field `PILO_OSC_TRIM` writer - Trim for current in oscillator block."]
pub type PILO_OSC_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PILO_COMP_TRIM` reader - Trim for comparator bias current."]
pub type PILO_COMP_TRIM_R = crate::FieldReader;
#[doc = "Field `PILO_COMP_TRIM` writer - Trim for comparator bias current."]
pub type PILO_COMP_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PILO_NBIAS_TRIM` reader - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
pub type PILO_NBIAS_TRIM_R = crate::FieldReader;
#[doc = "Field `PILO_NBIAS_TRIM` writer - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
pub type PILO_NBIAS_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PILO_RES_TRIM` reader - Trim for beta-multiplier branch current"]
pub type PILO_RES_TRIM_R = crate::FieldReader;
#[doc = "Field `PILO_RES_TRIM` writer - Trim for beta-multiplier branch current"]
pub type PILO_RES_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PILO_ISLOPE_TRIM` reader - Trim for beta-multiplier current slope"]
pub type PILO_ISLOPE_TRIM_R = crate::FieldReader;
#[doc = "Field `PILO_ISLOPE_TRIM` writer - Trim for beta-multiplier current slope"]
pub type PILO_ISLOPE_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PILO_VTDIFF_TRIM` reader - Trim for VT-DIFF output (internal power supply)"]
pub type PILO_VTDIFF_TRIM_R = crate::FieldReader;
#[doc = "Field `PILO_VTDIFF_TRIM` writer - Trim for VT-DIFF output (internal power supply)"]
pub type PILO_VTDIFF_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
    #[inline(always)]
    pub fn pilo_cfreq(&self) -> PILO_CFREQ_R {
        PILO_CFREQ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 12:14 - Trim for current in oscillator block."]
    #[inline(always)]
    pub fn pilo_osc_trim(&self) -> PILO_OSC_TRIM_R {
        PILO_OSC_TRIM_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Trim for comparator bias current."]
    #[inline(always)]
    pub fn pilo_comp_trim(&self) -> PILO_COMP_TRIM_R {
        PILO_COMP_TRIM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
    #[inline(always)]
    pub fn pilo_nbias_trim(&self) -> PILO_NBIAS_TRIM_R {
        PILO_NBIAS_TRIM_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:24 - Trim for beta-multiplier branch current"]
    #[inline(always)]
    pub fn pilo_res_trim(&self) -> PILO_RES_TRIM_R {
        PILO_RES_TRIM_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 26:27 - Trim for beta-multiplier current slope"]
    #[inline(always)]
    pub fn pilo_islope_trim(&self) -> PILO_ISLOPE_TRIM_R {
        PILO_ISLOPE_TRIM_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:30 - Trim for VT-DIFF output (internal power supply)"]
    #[inline(always)]
    pub fn pilo_vtdiff_trim(&self) -> PILO_VTDIFF_TRIM_R {
        PILO_VTDIFF_TRIM_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_cfreq(&mut self) -> PILO_CFREQ_W<CLK_TRIM_PILO_CTL_SPEC> {
        PILO_CFREQ_W::new(self, 0)
    }
    #[doc = "Bits 12:14 - Trim for current in oscillator block."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_osc_trim(&mut self) -> PILO_OSC_TRIM_W<CLK_TRIM_PILO_CTL_SPEC> {
        PILO_OSC_TRIM_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Trim for comparator bias current."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_comp_trim(&mut self) -> PILO_COMP_TRIM_W<CLK_TRIM_PILO_CTL_SPEC> {
        PILO_COMP_TRIM_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_nbias_trim(&mut self) -> PILO_NBIAS_TRIM_W<CLK_TRIM_PILO_CTL_SPEC> {
        PILO_NBIAS_TRIM_W::new(self, 18)
    }
    #[doc = "Bits 20:24 - Trim for beta-multiplier branch current"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_res_trim(&mut self) -> PILO_RES_TRIM_W<CLK_TRIM_PILO_CTL_SPEC> {
        PILO_RES_TRIM_W::new(self, 20)
    }
    #[doc = "Bits 26:27 - Trim for beta-multiplier current slope"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_islope_trim(&mut self) -> PILO_ISLOPE_TRIM_W<CLK_TRIM_PILO_CTL_SPEC> {
        PILO_ISLOPE_TRIM_W::new(self, 26)
    }
    #[doc = "Bits 28:30 - Trim for VT-DIFF output (internal power supply)"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_vtdiff_trim(&mut self) -> PILO_VTDIFF_TRIM_W<CLK_TRIM_PILO_CTL_SPEC> {
        PILO_VTDIFF_TRIM_W::new(self, 28)
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
#[doc = "PILO Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_pilo_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_pilo_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_TRIM_PILO_CTL_SPEC;
impl crate::RegisterSpec for CLK_TRIM_PILO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_trim_pilo_ctl::R`](R) reader structure"]
impl crate::Readable for CLK_TRIM_PILO_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_trim_pilo_ctl::W`](W) writer structure"]
impl crate::Writable for CLK_TRIM_PILO_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_PILO_CTL to value 0x0108_500f"]
impl crate::Resettable for CLK_TRIM_PILO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0108_500f;
}
