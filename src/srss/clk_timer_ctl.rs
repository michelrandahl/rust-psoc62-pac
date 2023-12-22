#[doc = "Register `CLK_TIMER_CTL` reader"]
pub type R = crate::R<CLK_TIMER_CTL_SPEC>;
#[doc = "Register `CLK_TIMER_CTL` writer"]
pub type W = crate::W<CLK_TIMER_CTL_SPEC>;
#[doc = "Field `TIMER_SEL` reader - Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV."]
pub type TIMER_SEL_R = crate::BitReader<TIMER_SEL_A>;
#[doc = "Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER_SEL_A {
    #[doc = "0: IMO - Internal Main Oscillator"]
    IMO = 0,
    #[doc = "1: Select the output of the predivider configured by TIMER_HF0_DIV."]
    HF0_DIV = 1,
}
impl From<TIMER_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_SEL_A {
        match self.bits {
            false => TIMER_SEL_A::IMO,
            true => TIMER_SEL_A::HF0_DIV,
        }
    }
    #[doc = "IMO - Internal Main Oscillator"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == TIMER_SEL_A::IMO
    }
    #[doc = "Select the output of the predivider configured by TIMER_HF0_DIV."]
    #[inline(always)]
    pub fn is_hf0_div(&self) -> bool {
        *self == TIMER_SEL_A::HF0_DIV
    }
}
#[doc = "Field `TIMER_SEL` writer - Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV."]
pub type TIMER_SEL_W<'a, REG> = crate::BitWriter<'a, REG, TIMER_SEL_A>;
impl<'a, REG> TIMER_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IMO - Internal Main Oscillator"]
    #[inline(always)]
    pub fn imo(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SEL_A::IMO)
    }
    #[doc = "Select the output of the predivider configured by TIMER_HF0_DIV."]
    #[inline(always)]
    pub fn hf0_div(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SEL_A::HF0_DIV)
    }
}
#[doc = "Field `TIMER_HF0_DIV` reader - Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock."]
pub type TIMER_HF0_DIV_R = crate::FieldReader<TIMER_HF0_DIV_A>;
#[doc = "Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_HF0_DIV_A {
    #[doc = "0: Transparent mode, feed through selected clock source w/o dividing or correcting duty cycle."]
    NO_DIV = 0,
    #[doc = "1: Divide HFCLK0 by 2."]
    DIV_BY_2 = 1,
    #[doc = "2: Divide HFCLK0 by 4."]
    DIV_BY_4 = 2,
    #[doc = "3: Divide HFCLK0 by 8."]
    DIV_BY_8 = 3,
}
impl From<TIMER_HF0_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_HF0_DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_HF0_DIV_A {
    type Ux = u8;
}
impl TIMER_HF0_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_HF0_DIV_A {
        match self.bits {
            0 => TIMER_HF0_DIV_A::NO_DIV,
            1 => TIMER_HF0_DIV_A::DIV_BY_2,
            2 => TIMER_HF0_DIV_A::DIV_BY_4,
            3 => TIMER_HF0_DIV_A::DIV_BY_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Transparent mode, feed through selected clock source w/o dividing or correcting duty cycle."]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == TIMER_HF0_DIV_A::NO_DIV
    }
    #[doc = "Divide HFCLK0 by 2."]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == TIMER_HF0_DIV_A::DIV_BY_2
    }
    #[doc = "Divide HFCLK0 by 4."]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == TIMER_HF0_DIV_A::DIV_BY_4
    }
    #[doc = "Divide HFCLK0 by 8."]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == TIMER_HF0_DIV_A::DIV_BY_8
    }
}
#[doc = "Field `TIMER_HF0_DIV` writer - Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock."]
pub type TIMER_HF0_DIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TIMER_HF0_DIV_A>;
impl<'a, REG> TIMER_HF0_DIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transparent mode, feed through selected clock source w/o dividing or correcting duty cycle."]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_HF0_DIV_A::NO_DIV)
    }
    #[doc = "Divide HFCLK0 by 2."]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_HF0_DIV_A::DIV_BY_2)
    }
    #[doc = "Divide HFCLK0 by 4."]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_HF0_DIV_A::DIV_BY_4)
    }
    #[doc = "Divide HFCLK0 by 8."]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_HF0_DIV_A::DIV_BY_8)
    }
}
#[doc = "Field `TIMER_DIV` reader - Divide selected timer clock source by (1+TIMER_DIV). The output of this divider is TIMERCLK Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while the timer is enabled."]
pub type TIMER_DIV_R = crate::FieldReader;
#[doc = "Field `TIMER_DIV` writer - Divide selected timer clock source by (1+TIMER_DIV). The output of this divider is TIMERCLK Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while the timer is enabled."]
pub type TIMER_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENABLE` reader - Enable for TIMERCLK. 0: TIMERCLK is off 1: TIMERCLK is enabled"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable for TIMERCLK. 0: TIMERCLK is off 1: TIMERCLK is enabled"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV."]
    #[inline(always)]
    pub fn timer_sel(&self) -> TIMER_SEL_R {
        TIMER_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock."]
    #[inline(always)]
    pub fn timer_hf0_div(&self) -> TIMER_HF0_DIV_R {
        TIMER_HF0_DIV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Divide selected timer clock source by (1+TIMER_DIV). The output of this divider is TIMERCLK Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while the timer is enabled."]
    #[inline(always)]
    pub fn timer_div(&self) -> TIMER_DIV_R {
        TIMER_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Enable for TIMERCLK. 0: TIMERCLK is off 1: TIMERCLK is enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV."]
    #[inline(always)]
    #[must_use]
    pub fn timer_sel(&mut self) -> TIMER_SEL_W<CLK_TIMER_CTL_SPEC> {
        TIMER_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock."]
    #[inline(always)]
    #[must_use]
    pub fn timer_hf0_div(&mut self) -> TIMER_HF0_DIV_W<CLK_TIMER_CTL_SPEC> {
        TIMER_HF0_DIV_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Divide selected timer clock source by (1+TIMER_DIV). The output of this divider is TIMERCLK Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while the timer is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn timer_div(&mut self) -> TIMER_DIV_W<CLK_TIMER_CTL_SPEC> {
        TIMER_DIV_W::new(self, 16)
    }
    #[doc = "Bit 31 - Enable for TIMERCLK. 0: TIMERCLK is off 1: TIMERCLK is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CLK_TIMER_CTL_SPEC> {
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
#[doc = "Timer Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_timer_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_timer_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_TIMER_CTL_SPEC;
impl crate::RegisterSpec for CLK_TIMER_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_timer_ctl::R`](R) reader structure"]
impl crate::Readable for CLK_TIMER_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_timer_ctl::W`](W) writer structure"]
impl crate::Writable for CLK_TIMER_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_TIMER_CTL to value 0x0007_0000"]
impl crate::Resettable for CLK_TIMER_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0000;
}
