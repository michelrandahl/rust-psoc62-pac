#[doc = "Register `CLK_FLL_CONFIG` reader"]
pub type R = crate::R<CLK_FLL_CONFIG_SPEC>;
#[doc = "Register `CLK_FLL_CONFIG` writer"]
pub type W = crate::W<CLK_FLL_CONFIG_SPEC>;
#[doc = "Field `FLL_MULT` reader - Multiplier to determine CCO frequency in multiples of the frequency of the selected reference clock (Fref). Ffll = (FLL_MULT) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV+1)"]
pub type FLL_MULT_R = crate::FieldReader<u32>;
#[doc = "Field `FLL_MULT` writer - Multiplier to determine CCO frequency in multiples of the frequency of the selected reference clock (Fref). Ffll = (FLL_MULT) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV+1)"]
pub type FLL_MULT_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `FLL_OUTPUT_DIV` reader - Control bits for Output divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: no division 1: divide by 2"]
pub type FLL_OUTPUT_DIV_R = crate::BitReader;
#[doc = "Field `FLL_OUTPUT_DIV` writer - Control bits for Output divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: no division 1: divide by 2"]
pub type FLL_OUTPUT_DIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLL_ENABLE` reader - Master enable for FLL. The FLL requires firmware sequencing when enabling, disabling, and entering/exiting DEEPSLEEP. To enable the FLL, first enable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=1 and wait until CLK_FLL_STATUS.CCO_READY==1. Next, ensure the reference clock has stabilized and CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF. Next, write FLL_ENABLE=1 and wait until CLK_FLL_STATUS.LOCKED==1. Finally, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. It takes seven reference clock cycles plus four FLL output cycles to switch to the FLL output. Do not disable the FLL before this time completes. To disable the FLL, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF and (optionally) read the same register to ensure the write completes. Then, wait at least seven FLL reference clock cycles before disabling it with FLL_ENABLE=0. Lastly, disable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=0. Before entering DEEPSLEEP, either disable the FLL using above sequence or use the following procedure to deselect/select it before/after DEEPSLEEP. Before entering DEEPSLEEP, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF to change the FLL to use its reference clock. After DEEPSLEEP wakeup, wait until CLK_FLL_STATUS.LOCKED==1 and then write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. 0: Block is powered off 1: Block is powered on"]
pub type FLL_ENABLE_R = crate::BitReader;
#[doc = "Field `FLL_ENABLE` writer - Master enable for FLL. The FLL requires firmware sequencing when enabling, disabling, and entering/exiting DEEPSLEEP. To enable the FLL, first enable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=1 and wait until CLK_FLL_STATUS.CCO_READY==1. Next, ensure the reference clock has stabilized and CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF. Next, write FLL_ENABLE=1 and wait until CLK_FLL_STATUS.LOCKED==1. Finally, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. It takes seven reference clock cycles plus four FLL output cycles to switch to the FLL output. Do not disable the FLL before this time completes. To disable the FLL, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF and (optionally) read the same register to ensure the write completes. Then, wait at least seven FLL reference clock cycles before disabling it with FLL_ENABLE=0. Lastly, disable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=0. Before entering DEEPSLEEP, either disable the FLL using above sequence or use the following procedure to deselect/select it before/after DEEPSLEEP. Before entering DEEPSLEEP, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF to change the FLL to use its reference clock. After DEEPSLEEP wakeup, wait until CLK_FLL_STATUS.LOCKED==1 and then write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. 0: Block is powered off 1: Block is powered on"]
pub type FLL_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:17 - Multiplier to determine CCO frequency in multiples of the frequency of the selected reference clock (Fref). Ffll = (FLL_MULT) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV+1)"]
    #[inline(always)]
    pub fn fll_mult(&self) -> FLL_MULT_R {
        FLL_MULT_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bit 24 - Control bits for Output divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: no division 1: divide by 2"]
    #[inline(always)]
    pub fn fll_output_div(&self) -> FLL_OUTPUT_DIV_R {
        FLL_OUTPUT_DIV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable for FLL. The FLL requires firmware sequencing when enabling, disabling, and entering/exiting DEEPSLEEP. To enable the FLL, first enable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=1 and wait until CLK_FLL_STATUS.CCO_READY==1. Next, ensure the reference clock has stabilized and CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF. Next, write FLL_ENABLE=1 and wait until CLK_FLL_STATUS.LOCKED==1. Finally, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. It takes seven reference clock cycles plus four FLL output cycles to switch to the FLL output. Do not disable the FLL before this time completes. To disable the FLL, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF and (optionally) read the same register to ensure the write completes. Then, wait at least seven FLL reference clock cycles before disabling it with FLL_ENABLE=0. Lastly, disable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=0. Before entering DEEPSLEEP, either disable the FLL using above sequence or use the following procedure to deselect/select it before/after DEEPSLEEP. Before entering DEEPSLEEP, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF to change the FLL to use its reference clock. After DEEPSLEEP wakeup, wait until CLK_FLL_STATUS.LOCKED==1 and then write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    pub fn fll_enable(&self) -> FLL_ENABLE_R {
        FLL_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Multiplier to determine CCO frequency in multiples of the frequency of the selected reference clock (Fref). Ffll = (FLL_MULT) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV+1)"]
    #[inline(always)]
    #[must_use]
    pub fn fll_mult(&mut self) -> FLL_MULT_W<CLK_FLL_CONFIG_SPEC> {
        FLL_MULT_W::new(self, 0)
    }
    #[doc = "Bit 24 - Control bits for Output divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: no division 1: divide by 2"]
    #[inline(always)]
    #[must_use]
    pub fn fll_output_div(&mut self) -> FLL_OUTPUT_DIV_W<CLK_FLL_CONFIG_SPEC> {
        FLL_OUTPUT_DIV_W::new(self, 24)
    }
    #[doc = "Bit 31 - Master enable for FLL. The FLL requires firmware sequencing when enabling, disabling, and entering/exiting DEEPSLEEP. To enable the FLL, first enable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=1 and wait until CLK_FLL_STATUS.CCO_READY==1. Next, ensure the reference clock has stabilized and CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF. Next, write FLL_ENABLE=1 and wait until CLK_FLL_STATUS.LOCKED==1. Finally, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. It takes seven reference clock cycles plus four FLL output cycles to switch to the FLL output. Do not disable the FLL before this time completes. To disable the FLL, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF and (optionally) read the same register to ensure the write completes. Then, wait at least seven FLL reference clock cycles before disabling it with FLL_ENABLE=0. Lastly, disable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=0. Before entering DEEPSLEEP, either disable the FLL using above sequence or use the following procedure to deselect/select it before/after DEEPSLEEP. Before entering DEEPSLEEP, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF to change the FLL to use its reference clock. After DEEPSLEEP wakeup, wait until CLK_FLL_STATUS.LOCKED==1 and then write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    #[must_use]
    pub fn fll_enable(&mut self) -> FLL_ENABLE_W<CLK_FLL_CONFIG_SPEC> {
        FLL_ENABLE_W::new(self, 31)
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
#[doc = "FLL Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_fll_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_fll_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_FLL_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_FLL_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_fll_config::R`](R) reader structure"]
impl crate::Readable for CLK_FLL_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_fll_config::W`](W) writer structure"]
impl crate::Writable for CLK_FLL_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_FLL_CONFIG to value 0x0100_0000"]
impl crate::Resettable for CLK_FLL_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
