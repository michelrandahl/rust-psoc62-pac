#[doc = "Register `CLOCK_CTL` reader"]
pub type R = crate::R<CLOCK_CTL_SPEC>;
#[doc = "Register `CLOCK_CTL` writer"]
pub type W = crate::W<CLOCK_CTL_SPEC>;
#[doc = "Field `CLK_CLOCK_DIV` reader - PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
pub type CLK_CLOCK_DIV_R = crate::FieldReader<CLK_CLOCK_DIV_A>;
#[doc = "PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_CLOCK_DIV_A {
    #[doc = "0: Divide by 1"]
    DIVBY1 = 0,
    #[doc = "1: Divide by 2 (no 50 percent duty cycle)"]
    DIVBY2 = 1,
    #[doc = "2: Divide by 3 (no 50 percent duty cycle)"]
    DIVBY3 = 2,
    #[doc = "3: Divide by 4 (no 50 percent duty cycle)"]
    DIVBY4 = 3,
}
impl From<CLK_CLOCK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_CLOCK_DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLK_CLOCK_DIV_A {
    type Ux = u8;
}
impl CLK_CLOCK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_CLOCK_DIV_A {
        match self.bits {
            0 => CLK_CLOCK_DIV_A::DIVBY1,
            1 => CLK_CLOCK_DIV_A::DIVBY2,
            2 => CLK_CLOCK_DIV_A::DIVBY3,
            3 => CLK_CLOCK_DIV_A::DIVBY4,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_divby1(&self) -> bool {
        *self == CLK_CLOCK_DIV_A::DIVBY1
    }
    #[doc = "Divide by 2 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn is_divby2(&self) -> bool {
        *self == CLK_CLOCK_DIV_A::DIVBY2
    }
    #[doc = "Divide by 3 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn is_divby3(&self) -> bool {
        *self == CLK_CLOCK_DIV_A::DIVBY3
    }
    #[doc = "Divide by 4 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn is_divby4(&self) -> bool {
        *self == CLK_CLOCK_DIV_A::DIVBY4
    }
}
#[doc = "Field `CLK_CLOCK_DIV` writer - PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
pub type CLK_CLOCK_DIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CLK_CLOCK_DIV_A>;
impl<'a, REG> CLK_CLOCK_DIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divby1(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_CLOCK_DIV_A::DIVBY1)
    }
    #[doc = "Divide by 2 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby2(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_CLOCK_DIV_A::DIVBY2)
    }
    #[doc = "Divide by 3 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby3(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_CLOCK_DIV_A::DIVBY3)
    }
    #[doc = "Divide by 4 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby4(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_CLOCK_DIV_A::DIVBY4)
    }
}
#[doc = "Field `MCLKQ_CLOCK_DIV` reader - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
pub type MCLKQ_CLOCK_DIV_R = crate::FieldReader<MCLKQ_CLOCK_DIV_A>;
#[doc = "MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCLKQ_CLOCK_DIV_A {
    #[doc = "0: Divide by 1"]
    DIVBY1 = 0,
    #[doc = "1: Divide by 2 (no 50 percent duty cycle)"]
    DIVBY2 = 1,
    #[doc = "2: Divide by 3 (no 50 percent duty cycle)"]
    DIVBY3 = 2,
    #[doc = "3: Divide by 4 (no 50 percent duty cycle)"]
    DIVBY4 = 3,
}
impl From<MCLKQ_CLOCK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MCLKQ_CLOCK_DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCLKQ_CLOCK_DIV_A {
    type Ux = u8;
}
impl MCLKQ_CLOCK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCLKQ_CLOCK_DIV_A {
        match self.bits {
            0 => MCLKQ_CLOCK_DIV_A::DIVBY1,
            1 => MCLKQ_CLOCK_DIV_A::DIVBY2,
            2 => MCLKQ_CLOCK_DIV_A::DIVBY3,
            3 => MCLKQ_CLOCK_DIV_A::DIVBY4,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_divby1(&self) -> bool {
        *self == MCLKQ_CLOCK_DIV_A::DIVBY1
    }
    #[doc = "Divide by 2 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn is_divby2(&self) -> bool {
        *self == MCLKQ_CLOCK_DIV_A::DIVBY2
    }
    #[doc = "Divide by 3 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn is_divby3(&self) -> bool {
        *self == MCLKQ_CLOCK_DIV_A::DIVBY3
    }
    #[doc = "Divide by 4 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn is_divby4(&self) -> bool {
        *self == MCLKQ_CLOCK_DIV_A::DIVBY4
    }
}
#[doc = "Field `MCLKQ_CLOCK_DIV` writer - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
pub type MCLKQ_CLOCK_DIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MCLKQ_CLOCK_DIV_A>;
impl<'a, REG> MCLKQ_CLOCK_DIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divby1(self) -> &'a mut crate::W<REG> {
        self.variant(MCLKQ_CLOCK_DIV_A::DIVBY1)
    }
    #[doc = "Divide by 2 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby2(self) -> &'a mut crate::W<REG> {
        self.variant(MCLKQ_CLOCK_DIV_A::DIVBY2)
    }
    #[doc = "Divide by 3 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby3(self) -> &'a mut crate::W<REG> {
        self.variant(MCLKQ_CLOCK_DIV_A::DIVBY3)
    }
    #[doc = "Divide by 4 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby4(self) -> &'a mut crate::W<REG> {
        self.variant(MCLKQ_CLOCK_DIV_A::DIVBY4)
    }
}
#[doc = "Field `CKO_CLOCK_DIV` reader - PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
pub type CKO_CLOCK_DIV_R = crate::FieldReader;
#[doc = "Field `CKO_CLOCK_DIV` writer - PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
pub type CKO_CLOCK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SINC_RATE` reader - SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
pub type SINC_RATE_R = crate::FieldReader;
#[doc = "Field `SINC_RATE` writer - SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
pub type SINC_RATE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:1 - PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
    #[inline(always)]
    pub fn clk_clock_div(&self) -> CLK_CLOCK_DIV_R {
        CLK_CLOCK_DIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
    #[inline(always)]
    pub fn mclkq_clock_div(&self) -> MCLKQ_CLOCK_DIV_R {
        MCLKQ_CLOCK_DIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
    #[inline(always)]
    pub fn cko_clock_div(&self) -> CKO_CLOCK_DIV_R {
        CKO_CLOCK_DIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
    #[inline(always)]
    pub fn sinc_rate(&self) -> SINC_RATE_R {
        SINC_RATE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
    #[inline(always)]
    #[must_use]
    pub fn clk_clock_div(&mut self) -> CLK_CLOCK_DIV_W<CLOCK_CTL_SPEC> {
        CLK_CLOCK_DIV_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
    #[inline(always)]
    #[must_use]
    pub fn mclkq_clock_div(&mut self) -> MCLKQ_CLOCK_DIV_W<CLOCK_CTL_SPEC> {
        MCLKQ_CLOCK_DIV_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
    #[inline(always)]
    #[must_use]
    pub fn cko_clock_div(&mut self) -> CKO_CLOCK_DIV_W<CLOCK_CTL_SPEC> {
        CKO_CLOCK_DIV_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
    #[inline(always)]
    #[must_use]
    pub fn sinc_rate(&mut self) -> SINC_RATE_W<CLOCK_CTL_SPEC> {
        SINC_RATE_W::new(self, 16)
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
#[doc = "`reset()` method sets CLOCK_CTL to value 0x0020_0310"]
impl crate::Resettable for CLOCK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0310;
}
