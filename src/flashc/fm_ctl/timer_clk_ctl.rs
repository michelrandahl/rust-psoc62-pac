#[doc = "Register `TIMER_CLK_CTL` reader"]
pub type R = crate::R<TIMER_CLK_CTL_SPEC>;
#[doc = "Register `TIMER_CLK_CTL` writer"]
pub type W = crate::W<TIMER_CLK_CTL_SPEC>;
#[doc = "Field `TIMER_CLOCK_FREQ` reader - Clk_t frequency divider to provide the 1MHz reference clock for the Regif Timer. Equal to the frequency in MHz of the timer clock 'clk_t'. Example: if 'clk_t' has a frequency of 4 MHz then this field value is '4' Max clk_t frequency = 100MHz. This field is updated at runtime with the 'SW_TIMER_CLOCK_FREQ ' value from the HV parameters table"]
pub type TIMER_CLOCK_FREQ_R = crate::FieldReader;
#[doc = "Field `TIMER_CLOCK_FREQ` writer - Clk_t frequency divider to provide the 1MHz reference clock for the Regif Timer. Equal to the frequency in MHz of the timer clock 'clk_t'. Example: if 'clk_t' has a frequency of 4 MHz then this field value is '4' Max clk_t frequency = 100MHz. This field is updated at runtime with the 'SW_TIMER_CLOCK_FREQ ' value from the HV parameters table"]
pub type TIMER_CLOCK_FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_PRG_PEON` reader - PROG&amp;PRE_PROG: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_PRG_PEON_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_PRG_PEON` writer - PROG&amp;PRE_PROG: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_PRG_PEON_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_PRG_PEOFF` reader - PROG&amp;PRE_PROG: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_PRG_PEOFF_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_PRG_PEOFF` writer - PROG&amp;PRE_PROG: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_PRG_PEOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_PRG_SEQ01` reader - PROG&amp;PRE_PROG: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_PRG_SEQ01_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_PRG_SEQ01` writer - PROG&amp;PRE_PROG: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_PRG_SEQ01_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clk_t frequency divider to provide the 1MHz reference clock for the Regif Timer. Equal to the frequency in MHz of the timer clock 'clk_t'. Example: if 'clk_t' has a frequency of 4 MHz then this field value is '4' Max clk_t frequency = 100MHz. This field is updated at runtime with the 'SW_TIMER_CLOCK_FREQ ' value from the HV parameters table"]
    #[inline(always)]
    pub fn timer_clock_freq(&self) -> TIMER_CLOCK_FREQ_R {
        TIMER_CLOCK_FREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PROG&amp;PRE_PROG: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_peon(&self) -> RGRANT_DELAY_PRG_PEON_R {
        RGRANT_DELAY_PRG_PEON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PROG&amp;PRE_PROG: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_peoff(&self) -> RGRANT_DELAY_PRG_PEOFF_R {
        RGRANT_DELAY_PRG_PEOFF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - PROG&amp;PRE_PROG: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_seq01(&self) -> RGRANT_DELAY_PRG_SEQ01_R {
        RGRANT_DELAY_PRG_SEQ01_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clk_t frequency divider to provide the 1MHz reference clock for the Regif Timer. Equal to the frequency in MHz of the timer clock 'clk_t'. Example: if 'clk_t' has a frequency of 4 MHz then this field value is '4' Max clk_t frequency = 100MHz. This field is updated at runtime with the 'SW_TIMER_CLOCK_FREQ ' value from the HV parameters table"]
    #[inline(always)]
    #[must_use]
    pub fn timer_clock_freq(&mut self) -> TIMER_CLOCK_FREQ_W<TIMER_CLK_CTL_SPEC> {
        TIMER_CLOCK_FREQ_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - PROG&amp;PRE_PROG: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_prg_peon(&mut self) -> RGRANT_DELAY_PRG_PEON_W<TIMER_CLK_CTL_SPEC> {
        RGRANT_DELAY_PRG_PEON_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - PROG&amp;PRE_PROG: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_prg_peoff(&mut self) -> RGRANT_DELAY_PRG_PEOFF_W<TIMER_CLK_CTL_SPEC> {
        RGRANT_DELAY_PRG_PEOFF_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - PROG&amp;PRE_PROG: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_prg_seq01(&mut self) -> RGRANT_DELAY_PRG_SEQ01_W<TIMER_CLK_CTL_SPEC> {
        RGRANT_DELAY_PRG_SEQ01_W::new(self, 24)
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
#[doc = "Timer prescaler (clk_t to timer clock frequency divider)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_clk_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_clk_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_CLK_CTL_SPEC;
impl crate::RegisterSpec for TIMER_CLK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_clk_ctl::R`](R) reader structure"]
impl crate::Readable for TIMER_CLK_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_clk_ctl::W`](W) writer structure"]
impl crate::Writable for TIMER_CLK_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER_CLK_CTL to value 0x08"]
impl crate::Resettable for TIMER_CLK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
