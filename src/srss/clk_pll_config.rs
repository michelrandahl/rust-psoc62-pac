#[doc = "Register `CLK_PLL_CONFIG[%s]` reader"]
pub type R = crate::R<CLK_PLL_CONFIG_SPEC>;
#[doc = "Register `CLK_PLL_CONFIG[%s]` writer"]
pub type W = crate::W<CLK_PLL_CONFIG_SPEC>;
#[doc = "Field `FEEDBACK_DIV` reader - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
pub type FEEDBACK_DIV_R = crate::FieldReader;
#[doc = "Field `FEEDBACK_DIV` writer - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
pub type FEEDBACK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `REFERENCE_DIV` reader - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
pub type REFERENCE_DIV_R = crate::FieldReader;
#[doc = "Field `REFERENCE_DIV` writer - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
pub type REFERENCE_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OUTPUT_DIV` reader - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
pub type OUTPUT_DIV_R = crate::FieldReader;
#[doc = "Field `OUTPUT_DIV` writer - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
pub type OUTPUT_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLL_LF_MODE` reader - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\]
1: VCO frequency is \\[170MHz, 200MHz)"]
pub type PLL_LF_MODE_R = crate::BitReader;
#[doc = "Field `PLL_LF_MODE` writer - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\]
1: VCO frequency is \\[170MHz, 200MHz)"]
pub type PLL_LF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS_SEL` reader - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
pub type BYPASS_SEL_R = crate::FieldReader<BYPASS_SEL_A>;
#[doc = "Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BYPASS_SEL_A {
    #[doc = "0: Automatic using lock indicator. When unlocked, automatically selects PLL reference input (bypass mode). When locked, automatically selects PLL output."]
    AUTO = 0,
    #[doc = "1: Same as AUTO"]
    AUTO1 = 1,
    #[doc = "2: Select PLL reference input (bypass mode). Ignores lock indicator"]
    PLL_REF = 2,
    #[doc = "3: Select PLL output. Ignores lock indicator."]
    PLL_OUT = 3,
}
impl From<BYPASS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BYPASS_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BYPASS_SEL_A {
    type Ux = u8;
}
impl BYPASS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BYPASS_SEL_A {
        match self.bits {
            0 => BYPASS_SEL_A::AUTO,
            1 => BYPASS_SEL_A::AUTO1,
            2 => BYPASS_SEL_A::PLL_REF,
            3 => BYPASS_SEL_A::PLL_OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Automatic using lock indicator. When unlocked, automatically selects PLL reference input (bypass mode). When locked, automatically selects PLL output."]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == BYPASS_SEL_A::AUTO
    }
    #[doc = "Same as AUTO"]
    #[inline(always)]
    pub fn is_auto1(&self) -> bool {
        *self == BYPASS_SEL_A::AUTO1
    }
    #[doc = "Select PLL reference input (bypass mode). Ignores lock indicator"]
    #[inline(always)]
    pub fn is_pll_ref(&self) -> bool {
        *self == BYPASS_SEL_A::PLL_REF
    }
    #[doc = "Select PLL output. Ignores lock indicator."]
    #[inline(always)]
    pub fn is_pll_out(&self) -> bool {
        *self == BYPASS_SEL_A::PLL_OUT
    }
}
#[doc = "Field `BYPASS_SEL` writer - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
pub type BYPASS_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BYPASS_SEL_A>;
impl<'a, REG> BYPASS_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Automatic using lock indicator. When unlocked, automatically selects PLL reference input (bypass mode). When locked, automatically selects PLL output."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS_SEL_A::AUTO)
    }
    #[doc = "Same as AUTO"]
    #[inline(always)]
    pub fn auto1(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS_SEL_A::AUTO1)
    }
    #[doc = "Select PLL reference input (bypass mode). Ignores lock indicator"]
    #[inline(always)]
    pub fn pll_ref(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS_SEL_A::PLL_REF)
    }
    #[doc = "Select PLL output. Ignores lock indicator."]
    #[inline(always)]
    pub fn pll_out(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS_SEL_A::PLL_OUT)
    }
}
#[doc = "Field `ENABLE` reader - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn feedback_div(&self) -> FEEDBACK_DIV_R {
        FEEDBACK_DIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:12 - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn reference_div(&self) -> REFERENCE_DIV_R {
        REFERENCE_DIV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn output_div(&self) -> OUTPUT_DIV_R {
        OUTPUT_DIV_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 27 - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\]
1: VCO frequency is \\[170MHz, 200MHz)"]
    #[inline(always)]
    pub fn pll_lf_mode(&self) -> PLL_LF_MODE_R {
        PLL_LF_MODE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
    #[inline(always)]
    pub fn bypass_sel(&self) -> BYPASS_SEL_R {
        BYPASS_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
    #[inline(always)]
    #[must_use]
    pub fn feedback_div(&mut self) -> FEEDBACK_DIV_W<CLK_PLL_CONFIG_SPEC> {
        FEEDBACK_DIV_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
    #[inline(always)]
    #[must_use]
    pub fn reference_div(&mut self) -> REFERENCE_DIV_W<CLK_PLL_CONFIG_SPEC> {
        REFERENCE_DIV_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
    #[inline(always)]
    #[must_use]
    pub fn output_div(&mut self) -> OUTPUT_DIV_W<CLK_PLL_CONFIG_SPEC> {
        OUTPUT_DIV_W::new(self, 16)
    }
    #[doc = "Bit 27 - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\]
1: VCO frequency is \\[170MHz, 200MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lf_mode(&mut self) -> PLL_LF_MODE_W<CLK_PLL_CONFIG_SPEC> {
        PLL_LF_MODE_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_sel(&mut self) -> BYPASS_SEL_W<CLK_PLL_CONFIG_SPEC> {
        BYPASS_SEL_W::new(self, 28)
    }
    #[doc = "Bit 31 - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CLK_PLL_CONFIG_SPEC> {
        ENABLE_W::new(self, 31)
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
#[doc = "PLL Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_PLL_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_PLL_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_pll_config::R`](R) reader structure"]
impl crate::Readable for CLK_PLL_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_pll_config::W`](W) writer structure"]
impl crate::Writable for CLK_PLL_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_PLL_CONFIG[%s]
to value 0x0002_0116"]
impl crate::Resettable for CLK_PLL_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0116;
}
