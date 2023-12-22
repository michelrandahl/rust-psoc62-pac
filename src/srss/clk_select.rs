#[doc = "Register `CLK_SELECT` reader"]
pub type R = crate::R<CLK_SELECT_SPEC>;
#[doc = "Register `CLK_SELECT` writer"]
pub type W = crate::W<CLK_SELECT_SPEC>;
#[doc = "Field `LFCLK_SEL` reader - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
pub type LFCLK_SEL_R = crate::FieldReader<LFCLK_SEL_A>;
#[doc = "Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFCLK_SEL_A {
    #[doc = "0: ILO - Internal Low-speed Oscillator"]
    ILO = 0,
    #[doc = "1: WCO - Watch-Crystal Oscillator. Requires Backup domain to be present and properly configured (including external watch crystal, if used)."]
    WCO = 1,
    #[doc = "2: ALTLF - Alternate Low-Frequency Clock. Capability is product-specific"]
    ALTLF = 2,
    #[doc = "3: PILO - Precision ILO. If present, it works in DEEPSLEEP and higher modes. Does not work in HIBERNATE mode."]
    PILO = 3,
}
impl From<LFCLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LFCLK_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFCLK_SEL_A {
    type Ux = u8;
}
impl LFCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFCLK_SEL_A {
        match self.bits {
            0 => LFCLK_SEL_A::ILO,
            1 => LFCLK_SEL_A::WCO,
            2 => LFCLK_SEL_A::ALTLF,
            3 => LFCLK_SEL_A::PILO,
            _ => unreachable!(),
        }
    }
    #[doc = "ILO - Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn is_ilo(&self) -> bool {
        *self == LFCLK_SEL_A::ILO
    }
    #[doc = "WCO - Watch-Crystal Oscillator. Requires Backup domain to be present and properly configured (including external watch crystal, if used)."]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == LFCLK_SEL_A::WCO
    }
    #[doc = "ALTLF - Alternate Low-Frequency Clock. Capability is product-specific"]
    #[inline(always)]
    pub fn is_altlf(&self) -> bool {
        *self == LFCLK_SEL_A::ALTLF
    }
    #[doc = "PILO - Precision ILO. If present, it works in DEEPSLEEP and higher modes. Does not work in HIBERNATE mode."]
    #[inline(always)]
    pub fn is_pilo(&self) -> bool {
        *self == LFCLK_SEL_A::PILO
    }
}
#[doc = "Field `LFCLK_SEL` writer - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
pub type LFCLK_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LFCLK_SEL_A>;
impl<'a, REG> LFCLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ILO - Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn ilo(self) -> &'a mut crate::W<REG> {
        self.variant(LFCLK_SEL_A::ILO)
    }
    #[doc = "WCO - Watch-Crystal Oscillator. Requires Backup domain to be present and properly configured (including external watch crystal, if used)."]
    #[inline(always)]
    pub fn wco(self) -> &'a mut crate::W<REG> {
        self.variant(LFCLK_SEL_A::WCO)
    }
    #[doc = "ALTLF - Alternate Low-Frequency Clock. Capability is product-specific"]
    #[inline(always)]
    pub fn altlf(self) -> &'a mut crate::W<REG> {
        self.variant(LFCLK_SEL_A::ALTLF)
    }
    #[doc = "PILO - Precision ILO. If present, it works in DEEPSLEEP and higher modes. Does not work in HIBERNATE mode."]
    #[inline(always)]
    pub fn pilo(self) -> &'a mut crate::W<REG> {
        self.variant(LFCLK_SEL_A::PILO)
    }
}
#[doc = "Field `PUMP_SEL` reader - Selects clock PATH&lt;k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
pub type PUMP_SEL_R = crate::FieldReader;
#[doc = "Field `PUMP_SEL` writer - Selects clock PATH&lt;k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
pub type PUMP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PUMP_DIV` reader - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
pub type PUMP_DIV_R = crate::FieldReader<PUMP_DIV_A>;
#[doc = "Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUMP_DIV_A {
    #[doc = "0: Transparent mode, feed through selected clock source w/o dividing."]
    NO_DIV = 0,
    #[doc = "1: Divide selected clock source by 2"]
    DIV_BY_2 = 1,
    #[doc = "2: Divide selected clock source by 4"]
    DIV_BY_4 = 2,
    #[doc = "3: Divide selected clock source by 8"]
    DIV_BY_8 = 3,
    #[doc = "4: Divide selected clock source by 16"]
    DIV_BY_16 = 4,
}
impl From<PUMP_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PUMP_DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUMP_DIV_A {
    type Ux = u8;
}
impl PUMP_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUMP_DIV_A> {
        match self.bits {
            0 => Some(PUMP_DIV_A::NO_DIV),
            1 => Some(PUMP_DIV_A::DIV_BY_2),
            2 => Some(PUMP_DIV_A::DIV_BY_4),
            3 => Some(PUMP_DIV_A::DIV_BY_8),
            4 => Some(PUMP_DIV_A::DIV_BY_16),
            _ => None,
        }
    }
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == PUMP_DIV_A::NO_DIV
    }
    #[doc = "Divide selected clock source by 2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == PUMP_DIV_A::DIV_BY_2
    }
    #[doc = "Divide selected clock source by 4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == PUMP_DIV_A::DIV_BY_4
    }
    #[doc = "Divide selected clock source by 8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == PUMP_DIV_A::DIV_BY_8
    }
    #[doc = "Divide selected clock source by 16"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == PUMP_DIV_A::DIV_BY_16
    }
}
#[doc = "Field `PUMP_DIV` writer - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
pub type PUMP_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PUMP_DIV_A>;
impl<'a, REG> PUMP_DIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut crate::W<REG> {
        self.variant(PUMP_DIV_A::NO_DIV)
    }
    #[doc = "Divide selected clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(PUMP_DIV_A::DIV_BY_2)
    }
    #[doc = "Divide selected clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(PUMP_DIV_A::DIV_BY_4)
    }
    #[doc = "Divide selected clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(PUMP_DIV_A::DIV_BY_8)
    }
    #[doc = "Divide selected clock source by 16"]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut crate::W<REG> {
        self.variant(PUMP_DIV_A::DIV_BY_16)
    }
}
#[doc = "Field `PUMP_ENABLE` reader - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
pub type PUMP_ENABLE_R = crate::BitReader;
#[doc = "Field `PUMP_ENABLE` writer - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
pub type PUMP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
    #[inline(always)]
    pub fn lfclk_sel(&self) -> LFCLK_SEL_R {
        LFCLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - Selects clock PATH&lt;k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
    #[inline(always)]
    pub fn pump_sel(&self) -> PUMP_SEL_R {
        PUMP_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
    #[inline(always)]
    pub fn pump_div(&self) -> PUMP_DIV_R {
        PUMP_DIV_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
    #[inline(always)]
    pub fn pump_enable(&self) -> PUMP_ENABLE_R {
        PUMP_ENABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
    #[inline(always)]
    #[must_use]
    pub fn lfclk_sel(&mut self) -> LFCLK_SEL_W<CLK_SELECT_SPEC> {
        LFCLK_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Selects clock PATH&lt;k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
    #[inline(always)]
    #[must_use]
    pub fn pump_sel(&mut self) -> PUMP_SEL_W<CLK_SELECT_SPEC> {
        PUMP_SEL_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
    #[inline(always)]
    #[must_use]
    pub fn pump_div(&mut self) -> PUMP_DIV_W<CLK_SELECT_SPEC> {
        PUMP_DIV_W::new(self, 12)
    }
    #[doc = "Bit 15 - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
    #[inline(always)]
    #[must_use]
    pub fn pump_enable(&mut self) -> PUMP_ENABLE_W<CLK_SELECT_SPEC> {
        PUMP_ENABLE_W::new(self, 15)
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
#[doc = "Clock selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_SELECT_SPEC;
impl crate::RegisterSpec for CLK_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_select::R`](R) reader structure"]
impl crate::Readable for CLK_SELECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_select::W`](W) writer structure"]
impl crate::Writable for CLK_SELECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_SELECT to value 0"]
impl crate::Resettable for CLK_SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
