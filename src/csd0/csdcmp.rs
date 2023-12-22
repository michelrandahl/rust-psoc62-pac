#[doc = "Register `CSDCMP` reader"]
pub type R = crate::R<CSDCMP_SPEC>;
#[doc = "Register `CSDCMP` writer"]
pub type W = crate::W<CSDCMP_SPEC>;
#[doc = "Field `CSDCMP_EN` reader - CSD Comparator Enable"]
pub type CSDCMP_EN_R = crate::BitReader<CSDCMP_EN_A>;
#[doc = "CSD Comparator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSDCMP_EN_A {
    #[doc = "0: Disable comparator, output is zero"]
    OFF = 0,
    #[doc = "1: On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    ON = 1,
}
impl From<CSDCMP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CSDCMP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CSDCMP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSDCMP_EN_A {
        match self.bits {
            false => CSDCMP_EN_A::OFF,
            true => CSDCMP_EN_A::ON,
        }
    }
    #[doc = "Disable comparator, output is zero"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CSDCMP_EN_A::OFF
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CSDCMP_EN_A::ON
    }
}
#[doc = "Field `CSDCMP_EN` writer - CSD Comparator Enable"]
pub type CSDCMP_EN_W<'a, REG> = crate::BitWriter<'a, REG, CSDCMP_EN_A>;
impl<'a, REG> CSDCMP_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable comparator, output is zero"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CSDCMP_EN_A::OFF)
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CSDCMP_EN_A::ON)
    }
}
#[doc = "Field `POLARITY_SEL` reader - Select which IDAC polarity to use to detect CSDCMP triggering"]
pub type POLARITY_SEL_R = crate::FieldReader<POLARITY_SEL_A>;
#[doc = "Select which IDAC polarity to use to detect CSDCMP triggering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POLARITY_SEL_A {
    #[doc = "0: Use idaca_pol (firmware setting with CSX and optionally DSI mixed in) to determine the direction, this is the most common use-case, used for normal CSD and normal CSX"]
    IDACA_POL = 0,
    #[doc = "1: Use idacb_pol (firmware setting with optional DSI mixed in) to determine the direction, this is only used for normal CSD if IDACB is used i.s.o. IDACA (not common)"]
    IDACB_POL = 1,
    #[doc = "2: Use the expression (csd_sense ? idaca_pol : idacb_pol) to determine the direction, this is only useful for the CSX with DUAL_IDAC use-case"]
    DUAL_POL = 2,
}
impl From<POLARITY_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: POLARITY_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POLARITY_SEL_A {
    type Ux = u8;
}
impl POLARITY_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<POLARITY_SEL_A> {
        match self.bits {
            0 => Some(POLARITY_SEL_A::IDACA_POL),
            1 => Some(POLARITY_SEL_A::IDACB_POL),
            2 => Some(POLARITY_SEL_A::DUAL_POL),
            _ => None,
        }
    }
    #[doc = "Use idaca_pol (firmware setting with CSX and optionally DSI mixed in) to determine the direction, this is the most common use-case, used for normal CSD and normal CSX"]
    #[inline(always)]
    pub fn is_idaca_pol(&self) -> bool {
        *self == POLARITY_SEL_A::IDACA_POL
    }
    #[doc = "Use idacb_pol (firmware setting with optional DSI mixed in) to determine the direction, this is only used for normal CSD if IDACB is used i.s.o. IDACA (not common)"]
    #[inline(always)]
    pub fn is_idacb_pol(&self) -> bool {
        *self == POLARITY_SEL_A::IDACB_POL
    }
    #[doc = "Use the expression (csd_sense ? idaca_pol : idacb_pol) to determine the direction, this is only useful for the CSX with DUAL_IDAC use-case"]
    #[inline(always)]
    pub fn is_dual_pol(&self) -> bool {
        *self == POLARITY_SEL_A::DUAL_POL
    }
}
#[doc = "Field `POLARITY_SEL` writer - Select which IDAC polarity to use to detect CSDCMP triggering"]
pub type POLARITY_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, POLARITY_SEL_A>;
impl<'a, REG> POLARITY_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use idaca_pol (firmware setting with CSX and optionally DSI mixed in) to determine the direction, this is the most common use-case, used for normal CSD and normal CSX"]
    #[inline(always)]
    pub fn idaca_pol(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY_SEL_A::IDACA_POL)
    }
    #[doc = "Use idacb_pol (firmware setting with optional DSI mixed in) to determine the direction, this is only used for normal CSD if IDACB is used i.s.o. IDACA (not common)"]
    #[inline(always)]
    pub fn idacb_pol(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY_SEL_A::IDACB_POL)
    }
    #[doc = "Use the expression (csd_sense ? idaca_pol : idacb_pol) to determine the direction, this is only useful for the CSX with DUAL_IDAC use-case"]
    #[inline(always)]
    pub fn dual_pol(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY_SEL_A::DUAL_POL)
    }
}
#[doc = "Field `CMP_PHASE` reader - Select in what phase(s) the comparator is active, typically set to match the BAL_MODE of the used IDAC. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap)."]
pub type CMP_PHASE_R = crate::FieldReader<CMP_PHASE_A>;
#[doc = "Select in what phase(s) the comparator is active, typically set to match the BAL_MODE of the used IDAC. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP_PHASE_A {
    #[doc = "0: Comparator is active from start of Phi2 and kept active into Phi1. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    FULL = 0,
    #[doc = "1: Comparator is active during Phi1 only. Currently no known use-case."]
    PHI1 = 1,
    #[doc = "2: Comparator is active during Phi2 only. Intended usage: CSD Low EMI."]
    PHI2 = 2,
    #[doc = "3: Comparator is activated at the start of both Phi1 and Phi2 (non-overlap should be enabled). Intended usage: CSX, or Full-Wave."]
    PHI1_2 = 3,
}
impl From<CMP_PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP_PHASE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMP_PHASE_A {
    type Ux = u8;
}
impl CMP_PHASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMP_PHASE_A {
        match self.bits {
            0 => CMP_PHASE_A::FULL,
            1 => CMP_PHASE_A::PHI1,
            2 => CMP_PHASE_A::PHI2,
            3 => CMP_PHASE_A::PHI1_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Comparator is active from start of Phi2 and kept active into Phi1. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == CMP_PHASE_A::FULL
    }
    #[doc = "Comparator is active during Phi1 only. Currently no known use-case."]
    #[inline(always)]
    pub fn is_phi1(&self) -> bool {
        *self == CMP_PHASE_A::PHI1
    }
    #[doc = "Comparator is active during Phi2 only. Intended usage: CSD Low EMI."]
    #[inline(always)]
    pub fn is_phi2(&self) -> bool {
        *self == CMP_PHASE_A::PHI2
    }
    #[doc = "Comparator is activated at the start of both Phi1 and Phi2 (non-overlap should be enabled). Intended usage: CSX, or Full-Wave."]
    #[inline(always)]
    pub fn is_phi1_2(&self) -> bool {
        *self == CMP_PHASE_A::PHI1_2
    }
}
#[doc = "Field `CMP_PHASE` writer - Select in what phase(s) the comparator is active, typically set to match the BAL_MODE of the used IDAC. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap)."]
pub type CMP_PHASE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CMP_PHASE_A>;
impl<'a, REG> CMP_PHASE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comparator is active from start of Phi2 and kept active into Phi1. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(CMP_PHASE_A::FULL)
    }
    #[doc = "Comparator is active during Phi1 only. Currently no known use-case."]
    #[inline(always)]
    pub fn phi1(self) -> &'a mut crate::W<REG> {
        self.variant(CMP_PHASE_A::PHI1)
    }
    #[doc = "Comparator is active during Phi2 only. Intended usage: CSD Low EMI."]
    #[inline(always)]
    pub fn phi2(self) -> &'a mut crate::W<REG> {
        self.variant(CMP_PHASE_A::PHI2)
    }
    #[doc = "Comparator is activated at the start of both Phi1 and Phi2 (non-overlap should be enabled). Intended usage: CSX, or Full-Wave."]
    #[inline(always)]
    pub fn phi1_2(self) -> &'a mut crate::W<REG> {
        self.variant(CMP_PHASE_A::PHI1_2)
    }
}
#[doc = "Field `CMP_MODE` reader - Select which signal to output on dsi_sample_out."]
pub type CMP_MODE_R = crate::BitReader<CMP_MODE_A>;
#[doc = "Select which signal to output on dsi_sample_out.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP_MODE_A {
    #[doc = "0: CSD mode: output the filtered sample signal on dsi_sample_out"]
    CSD = 0,
    #[doc = "1: General Purpose mode: output the unfiltered sample unfiltered comparator output, either asynchronous or flopped."]
    GP = 1,
}
impl From<CMP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: CMP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMP_MODE_A {
        match self.bits {
            false => CMP_MODE_A::CSD,
            true => CMP_MODE_A::GP,
        }
    }
    #[doc = "CSD mode: output the filtered sample signal on dsi_sample_out"]
    #[inline(always)]
    pub fn is_csd(&self) -> bool {
        *self == CMP_MODE_A::CSD
    }
    #[doc = "General Purpose mode: output the unfiltered sample unfiltered comparator output, either asynchronous or flopped."]
    #[inline(always)]
    pub fn is_gp(&self) -> bool {
        *self == CMP_MODE_A::GP
    }
}
#[doc = "Field `CMP_MODE` writer - Select which signal to output on dsi_sample_out."]
pub type CMP_MODE_W<'a, REG> = crate::BitWriter<'a, REG, CMP_MODE_A>;
impl<'a, REG> CMP_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSD mode: output the filtered sample signal on dsi_sample_out"]
    #[inline(always)]
    pub fn csd(self) -> &'a mut crate::W<REG> {
        self.variant(CMP_MODE_A::CSD)
    }
    #[doc = "General Purpose mode: output the unfiltered sample unfiltered comparator output, either asynchronous or flopped."]
    #[inline(always)]
    pub fn gp(self) -> &'a mut crate::W<REG> {
        self.variant(CMP_MODE_A::GP)
    }
}
#[doc = "Field `FEEDBACK_MODE` reader - This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out."]
pub type FEEDBACK_MODE_R = crate::BitReader<FEEDBACK_MODE_A>;
#[doc = "This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEEDBACK_MODE_A {
    #[doc = "0: Use feedback from sampling flip-flop (used in most modes)."]
    FLOP = 0,
    #[doc = "1: Use feedback from comparator directly (used in single Cmod mutual cap sensing only)"]
    COMP = 1,
}
impl From<FEEDBACK_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FEEDBACK_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl FEEDBACK_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FEEDBACK_MODE_A {
        match self.bits {
            false => FEEDBACK_MODE_A::FLOP,
            true => FEEDBACK_MODE_A::COMP,
        }
    }
    #[doc = "Use feedback from sampling flip-flop (used in most modes)."]
    #[inline(always)]
    pub fn is_flop(&self) -> bool {
        *self == FEEDBACK_MODE_A::FLOP
    }
    #[doc = "Use feedback from comparator directly (used in single Cmod mutual cap sensing only)"]
    #[inline(always)]
    pub fn is_comp(&self) -> bool {
        *self == FEEDBACK_MODE_A::COMP
    }
}
#[doc = "Field `FEEDBACK_MODE` writer - This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out."]
pub type FEEDBACK_MODE_W<'a, REG> = crate::BitWriter<'a, REG, FEEDBACK_MODE_A>;
impl<'a, REG> FEEDBACK_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use feedback from sampling flip-flop (used in most modes)."]
    #[inline(always)]
    pub fn flop(self) -> &'a mut crate::W<REG> {
        self.variant(FEEDBACK_MODE_A::FLOP)
    }
    #[doc = "Use feedback from comparator directly (used in single Cmod mutual cap sensing only)"]
    #[inline(always)]
    pub fn comp(self) -> &'a mut crate::W<REG> {
        self.variant(FEEDBACK_MODE_A::COMP)
    }
}
#[doc = "Field `AZ_EN` reader - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
pub type AZ_EN_R = crate::BitReader;
#[doc = "Field `AZ_EN` writer - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
pub type AZ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CSD Comparator Enable"]
    #[inline(always)]
    pub fn csdcmp_en(&self) -> CSDCMP_EN_R {
        CSDCMP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Select which IDAC polarity to use to detect CSDCMP triggering"]
    #[inline(always)]
    pub fn polarity_sel(&self) -> POLARITY_SEL_R {
        POLARITY_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Select in what phase(s) the comparator is active, typically set to match the BAL_MODE of the used IDAC. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap)."]
    #[inline(always)]
    pub fn cmp_phase(&self) -> CMP_PHASE_R {
        CMP_PHASE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 28 - Select which signal to output on dsi_sample_out."]
    #[inline(always)]
    pub fn cmp_mode(&self) -> CMP_MODE_R {
        CMP_MODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out."]
    #[inline(always)]
    pub fn feedback_mode(&self) -> FEEDBACK_MODE_R {
        FEEDBACK_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub fn az_en(&self) -> AZ_EN_R {
        AZ_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSD Comparator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csdcmp_en(&mut self) -> CSDCMP_EN_W<CSDCMP_SPEC> {
        CSDCMP_EN_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Select which IDAC polarity to use to detect CSDCMP triggering"]
    #[inline(always)]
    #[must_use]
    pub fn polarity_sel(&mut self) -> POLARITY_SEL_W<CSDCMP_SPEC> {
        POLARITY_SEL_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Select in what phase(s) the comparator is active, typically set to match the BAL_MODE of the used IDAC. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap)."]
    #[inline(always)]
    #[must_use]
    pub fn cmp_phase(&mut self) -> CMP_PHASE_W<CSDCMP_SPEC> {
        CMP_PHASE_W::new(self, 8)
    }
    #[doc = "Bit 28 - Select which signal to output on dsi_sample_out."]
    #[inline(always)]
    #[must_use]
    pub fn cmp_mode(&mut self) -> CMP_MODE_W<CSDCMP_SPEC> {
        CMP_MODE_W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out."]
    #[inline(always)]
    #[must_use]
    pub fn feedback_mode(&mut self) -> FEEDBACK_MODE_W<CSDCMP_SPEC> {
        FEEDBACK_MODE_W::new(self, 29)
    }
    #[doc = "Bit 31 - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    #[must_use]
    pub fn az_en(&mut self) -> AZ_EN_W<CSDCMP_SPEC> {
        AZ_EN_W::new(self, 31)
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
#[doc = "CSD Comparator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csdcmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csdcmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSDCMP_SPEC;
impl crate::RegisterSpec for CSDCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csdcmp::R`](R) reader structure"]
impl crate::Readable for CSDCMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csdcmp::W`](W) writer structure"]
impl crate::Writable for CSDCMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSDCMP to value 0"]
impl crate::Resettable for CSDCMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
