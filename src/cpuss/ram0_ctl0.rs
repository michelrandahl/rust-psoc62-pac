#[doc = "Register `RAM0_CTL0` reader"]
pub type R = crate::R<RAM0_CTL0_SPEC>;
#[doc = "Register `RAM0_CTL0` writer"]
pub type W = crate::W<RAM0_CTL0_SPEC>;
#[doc = "Field `SLOW_WS` reader - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type SLOW_WS_R = crate::FieldReader;
#[doc = "Field `SLOW_WS` writer - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type SLOW_WS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FAST_WS` reader - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type FAST_WS_R = crate::FieldReader;
#[doc = "Field `FAST_WS` writer - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type FAST_WS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECC_EN` reader - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type ECC_EN_R = crate::BitReader;
#[doc = "Field `ECC_EN` writer - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_AUTO_CORRECT` reader - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
pub type ECC_AUTO_CORRECT_R = crate::BitReader;
#[doc = "Field `ECC_AUTO_CORRECT` writer - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
pub type ECC_AUTO_CORRECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_INJ_EN` reader - Enable error injection for system SRAM 0. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of system SRAM 0."]
pub type ECC_INJ_EN_R = crate::BitReader;
#[doc = "Field `ECC_INJ_EN` writer - Enable error injection for system SRAM 0. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of system SRAM 0."]
pub type ECC_INJ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn slow_ws(&self) -> SLOW_WS_R {
        SLOW_WS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn fast_ws(&self) -> FAST_WS_R {
        FAST_WS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&self) -> ECC_EN_R {
        ECC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
    #[inline(always)]
    pub fn ecc_auto_correct(&self) -> ECC_AUTO_CORRECT_R {
        ECC_AUTO_CORRECT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable error injection for system SRAM 0. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of system SRAM 0."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> ECC_INJ_EN_R {
        ECC_INJ_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    #[must_use]
    pub fn slow_ws(&mut self) -> SLOW_WS_W<RAM0_CTL0_SPEC> {
        SLOW_WS_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    #[must_use]
    pub fn fast_ws(&mut self) -> FAST_WS_W<RAM0_CTL0_SPEC> {
        FAST_WS_W::new(self, 8)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> ECC_EN_W<RAM0_CTL0_SPEC> {
        ECC_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_auto_correct(&mut self) -> ECC_AUTO_CORRECT_W<RAM0_CTL0_SPEC> {
        ECC_AUTO_CORRECT_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable error injection for system SRAM 0. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of system SRAM 0."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_en(&mut self) -> ECC_INJ_EN_W<RAM0_CTL0_SPEC> {
        ECC_INJ_EN_W::new(self, 18)
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
#[doc = "RAM 0 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram0_ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram0_ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM0_CTL0_SPEC;
impl crate::RegisterSpec for RAM0_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram0_ctl0::R`](R) reader structure"]
impl crate::Readable for RAM0_CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ram0_ctl0::W`](W) writer structure"]
impl crate::Writable for RAM0_CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAM0_CTL0 to value 0x0003_0001"]
impl crate::Resettable for RAM0_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0001;
}
