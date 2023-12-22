#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `CNT_DURATION` reader - This field specifies if events (edges) or a duration of the monitor signal is counted. '0': Events are monitored. An edge detection is done. All edges of the selected monitor signal are counted. '1': A duration is monitored. No edge detection is done. The monitored signal is taken as a (high active) level signal for enabling the profiling counter. Note: All monitor signals which only can represent events are edge encoded in hardware, therefore a selection of CTL.CNT_DURATION=1 will not produce meaningful results."]
pub type CNT_DURATION_R = crate::BitReader;
#[doc = "Field `CNT_DURATION` writer - This field specifies if events (edges) or a duration of the monitor signal is counted. '0': Events are monitored. An edge detection is done. All edges of the selected monitor signal are counted. '1': A duration is monitored. No edge detection is done. The monitored signal is taken as a (high active) level signal for enabling the profiling counter. Note: All monitor signals which only can represent events are edge encoded in hardware, therefore a selection of CTL.CNT_DURATION=1 will not produce meaningful results."]
pub type CNT_DURATION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_CLK_SEL` reader - This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0."]
pub type REF_CLK_SEL_R = crate::FieldReader<REF_CLK_SEL_A>;
#[doc = "This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REF_CLK_SEL_A {
    #[doc = "0: Timer clock (divided or undivided high frequency clock, e.g. from IMO). Selection is done in SRSS register CLK_TIMER_CTL.TIMER_SEL."]
    CLK_TIMER = 0,
    #[doc = "1: IMO - Internal Main Oscillator"]
    CLK_IMO = 1,
    #[doc = "2: ECO - External-Crystal Oscillator"]
    CLK_ECO = 2,
    #[doc = "3: Low frequency clock (ILO, WCO or ALTLF). Selection is done in SRSS register CLK_SELECT.LFCLK_SEL."]
    CLK_LF = 3,
    #[doc = "4: High frequuency clock ('clk_hfx')."]
    CLK_HF = 4,
    #[doc = "5: Peripheral clock ('clk_peri')."]
    CLK_PERI = 5,
    #[doc = "6: N/A"]
    RSVD_6 = 6,
    #[doc = "7: N/A"]
    RSVD_7 = 7,
}
impl From<REF_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REF_CLK_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REF_CLK_SEL_A {
    type Ux = u8;
}
impl REF_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REF_CLK_SEL_A {
        match self.bits {
            0 => REF_CLK_SEL_A::CLK_TIMER,
            1 => REF_CLK_SEL_A::CLK_IMO,
            2 => REF_CLK_SEL_A::CLK_ECO,
            3 => REF_CLK_SEL_A::CLK_LF,
            4 => REF_CLK_SEL_A::CLK_HF,
            5 => REF_CLK_SEL_A::CLK_PERI,
            6 => REF_CLK_SEL_A::RSVD_6,
            7 => REF_CLK_SEL_A::RSVD_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer clock (divided or undivided high frequency clock, e.g. from IMO). Selection is done in SRSS register CLK_TIMER_CTL.TIMER_SEL."]
    #[inline(always)]
    pub fn is_clk_timer(&self) -> bool {
        *self == REF_CLK_SEL_A::CLK_TIMER
    }
    #[doc = "IMO - Internal Main Oscillator"]
    #[inline(always)]
    pub fn is_clk_imo(&self) -> bool {
        *self == REF_CLK_SEL_A::CLK_IMO
    }
    #[doc = "ECO - External-Crystal Oscillator"]
    #[inline(always)]
    pub fn is_clk_eco(&self) -> bool {
        *self == REF_CLK_SEL_A::CLK_ECO
    }
    #[doc = "Low frequency clock (ILO, WCO or ALTLF). Selection is done in SRSS register CLK_SELECT.LFCLK_SEL."]
    #[inline(always)]
    pub fn is_clk_lf(&self) -> bool {
        *self == REF_CLK_SEL_A::CLK_LF
    }
    #[doc = "High frequuency clock ('clk_hfx')."]
    #[inline(always)]
    pub fn is_clk_hf(&self) -> bool {
        *self == REF_CLK_SEL_A::CLK_HF
    }
    #[doc = "Peripheral clock ('clk_peri')."]
    #[inline(always)]
    pub fn is_clk_peri(&self) -> bool {
        *self == REF_CLK_SEL_A::CLK_PERI
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_rsvd_6(&self) -> bool {
        *self == REF_CLK_SEL_A::RSVD_6
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_rsvd_7(&self) -> bool {
        *self == REF_CLK_SEL_A::RSVD_7
    }
}
#[doc = "Field `REF_CLK_SEL` writer - This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0."]
pub type REF_CLK_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, REF_CLK_SEL_A>;
impl<'a, REG> REF_CLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer clock (divided or undivided high frequency clock, e.g. from IMO). Selection is done in SRSS register CLK_TIMER_CTL.TIMER_SEL."]
    #[inline(always)]
    pub fn clk_timer(self) -> &'a mut crate::W<REG> {
        self.variant(REF_CLK_SEL_A::CLK_TIMER)
    }
    #[doc = "IMO - Internal Main Oscillator"]
    #[inline(always)]
    pub fn clk_imo(self) -> &'a mut crate::W<REG> {
        self.variant(REF_CLK_SEL_A::CLK_IMO)
    }
    #[doc = "ECO - External-Crystal Oscillator"]
    #[inline(always)]
    pub fn clk_eco(self) -> &'a mut crate::W<REG> {
        self.variant(REF_CLK_SEL_A::CLK_ECO)
    }
    #[doc = "Low frequency clock (ILO, WCO or ALTLF). Selection is done in SRSS register CLK_SELECT.LFCLK_SEL."]
    #[inline(always)]
    pub fn clk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(REF_CLK_SEL_A::CLK_LF)
    }
    #[doc = "High frequuency clock ('clk_hfx')."]
    #[inline(always)]
    pub fn clk_hf(self) -> &'a mut crate::W<REG> {
        self.variant(REF_CLK_SEL_A::CLK_HF)
    }
    #[doc = "Peripheral clock ('clk_peri')."]
    #[inline(always)]
    pub fn clk_peri(self) -> &'a mut crate::W<REG> {
        self.variant(REF_CLK_SEL_A::CLK_PERI)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd_6(self) -> &'a mut crate::W<REG> {
        self.variant(REF_CLK_SEL_A::RSVD_6)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd_7(self) -> &'a mut crate::W<REG> {
        self.variant(REF_CLK_SEL_A::RSVD_7)
    }
}
#[doc = "Field `MON_SEL` reader - This field specifies the montior input signal to be observed by the profiling counter. The monitor signals are product specific, see product definition spreadsheet tab 'Monitor' for details."]
pub type MON_SEL_R = crate::FieldReader;
#[doc = "Field `MON_SEL` writer - This field specifies the montior input signal to be observed by the profiling counter. The monitor signals are product specific, see product definition spreadsheet tab 'Monitor' for details."]
pub type MON_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ENABLED` reader - Enables the profiling counter: '0': Disabled. '1': Enabled."]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - Enables the profiling counter: '0': Disabled. '1': Enabled."]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field specifies if events (edges) or a duration of the monitor signal is counted. '0': Events are monitored. An edge detection is done. All edges of the selected monitor signal are counted. '1': A duration is monitored. No edge detection is done. The monitored signal is taken as a (high active) level signal for enabling the profiling counter. Note: All monitor signals which only can represent events are edge encoded in hardware, therefore a selection of CTL.CNT_DURATION=1 will not produce meaningful results."]
    #[inline(always)]
    pub fn cnt_duration(&self) -> CNT_DURATION_R {
        CNT_DURATION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0."]
    #[inline(always)]
    pub fn ref_clk_sel(&self) -> REF_CLK_SEL_R {
        REF_CLK_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:22 - This field specifies the montior input signal to be observed by the profiling counter. The monitor signals are product specific, see product definition spreadsheet tab 'Monitor' for details."]
    #[inline(always)]
    pub fn mon_sel(&self) -> MON_SEL_R {
        MON_SEL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Enables the profiling counter: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field specifies if events (edges) or a duration of the monitor signal is counted. '0': Events are monitored. An edge detection is done. All edges of the selected monitor signal are counted. '1': A duration is monitored. No edge detection is done. The monitored signal is taken as a (high active) level signal for enabling the profiling counter. Note: All monitor signals which only can represent events are edge encoded in hardware, therefore a selection of CTL.CNT_DURATION=1 will not produce meaningful results."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_duration(&mut self) -> CNT_DURATION_W<CTL_SPEC> {
        CNT_DURATION_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0."]
    #[inline(always)]
    #[must_use]
    pub fn ref_clk_sel(&mut self) -> REF_CLK_SEL_W<CTL_SPEC> {
        REF_CLK_SEL_W::new(self, 4)
    }
    #[doc = "Bits 16:22 - This field specifies the montior input signal to be observed by the profiling counter. The monitor signals are product specific, see product definition spreadsheet tab 'Monitor' for details."]
    #[inline(always)]
    #[must_use]
    pub fn mon_sel(&mut self) -> MON_SEL_W<CTL_SPEC> {
        MON_SEL_W::new(self, 16)
    }
    #[doc = "Bit 31 - Enables the profiling counter: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<CTL_SPEC> {
        ENABLED_W::new(self, 31)
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
#[doc = "Profile counter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
