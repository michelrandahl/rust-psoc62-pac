#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `IREF_SEL` reader - N/A"]
pub type IREF_SEL_R = crate::BitReader<IREF_SEL_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREF_SEL_A {
    #[doc = "0: N/A"]
    IREF_SRSS = 0,
    #[doc = "1: N/A"]
    IREF_PASS = 1,
}
impl From<IREF_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: IREF_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl IREF_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IREF_SEL_A {
        match self.bits {
            false => IREF_SEL_A::IREF_SRSS,
            true => IREF_SEL_A::IREF_PASS,
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_iref_srss(&self) -> bool {
        *self == IREF_SEL_A::IREF_SRSS
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_iref_pass(&self) -> bool {
        *self == IREF_SEL_A::IREF_PASS
    }
}
#[doc = "Field `IREF_SEL` writer - N/A"]
pub type IREF_SEL_W<'a, REG> = crate::BitWriter<'a, REG, IREF_SEL_A>;
impl<'a, REG> IREF_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "N/A"]
    #[inline(always)]
    pub fn iref_srss(self) -> &'a mut crate::W<REG> {
        self.variant(IREF_SEL_A::IREF_SRSS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn iref_pass(self) -> &'a mut crate::W<REG> {
        self.variant(IREF_SEL_A::IREF_PASS)
    }
}
#[doc = "Field `FILTER_DELAY` reader - Enables the digital filtering on the CSD comparator"]
pub type FILTER_DELAY_R = crate::FieldReader;
#[doc = "Field `FILTER_DELAY` writer - Enables the digital filtering on the CSD comparator"]
pub type FILTER_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SHIELD_DELAY` reader - Configures the delay between shield clock and sensor clock"]
pub type SHIELD_DELAY_R = crate::FieldReader<SHIELD_DELAY_A>;
#[doc = "Configures the delay between shield clock and sensor clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHIELD_DELAY_A {
    #[doc = "0: Delay line is off; sensor clock = shield clock"]
    OFF = 0,
    #[doc = "1: shield clock is delayed by 5ns delay w.r.t sensor clock"]
    D5NS = 1,
    #[doc = "2: shield clock is delayed by 10ns delay w.r.t sensor clock"]
    D10NS = 2,
    #[doc = "3: shield clock is delayed by 20ns delay w.r.t sensor clock"]
    D20NS = 3,
}
impl From<SHIELD_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: SHIELD_DELAY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SHIELD_DELAY_A {
    type Ux = u8;
}
impl SHIELD_DELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SHIELD_DELAY_A {
        match self.bits {
            0 => SHIELD_DELAY_A::OFF,
            1 => SHIELD_DELAY_A::D5NS,
            2 => SHIELD_DELAY_A::D10NS,
            3 => SHIELD_DELAY_A::D20NS,
            _ => unreachable!(),
        }
    }
    #[doc = "Delay line is off; sensor clock = shield clock"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SHIELD_DELAY_A::OFF
    }
    #[doc = "shield clock is delayed by 5ns delay w.r.t sensor clock"]
    #[inline(always)]
    pub fn is_d5ns(&self) -> bool {
        *self == SHIELD_DELAY_A::D5NS
    }
    #[doc = "shield clock is delayed by 10ns delay w.r.t sensor clock"]
    #[inline(always)]
    pub fn is_d10ns(&self) -> bool {
        *self == SHIELD_DELAY_A::D10NS
    }
    #[doc = "shield clock is delayed by 20ns delay w.r.t sensor clock"]
    #[inline(always)]
    pub fn is_d20ns(&self) -> bool {
        *self == SHIELD_DELAY_A::D20NS
    }
}
#[doc = "Field `SHIELD_DELAY` writer - Configures the delay between shield clock and sensor clock"]
pub type SHIELD_DELAY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SHIELD_DELAY_A>;
impl<'a, REG> SHIELD_DELAY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Delay line is off; sensor clock = shield clock"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(SHIELD_DELAY_A::OFF)
    }
    #[doc = "shield clock is delayed by 5ns delay w.r.t sensor clock"]
    #[inline(always)]
    pub fn d5ns(self) -> &'a mut crate::W<REG> {
        self.variant(SHIELD_DELAY_A::D5NS)
    }
    #[doc = "shield clock is delayed by 10ns delay w.r.t sensor clock"]
    #[inline(always)]
    pub fn d10ns(self) -> &'a mut crate::W<REG> {
        self.variant(SHIELD_DELAY_A::D10NS)
    }
    #[doc = "shield clock is delayed by 20ns delay w.r.t sensor clock"]
    #[inline(always)]
    pub fn d20ns(self) -> &'a mut crate::W<REG> {
        self.variant(SHIELD_DELAY_A::D20NS)
    }
}
#[doc = "Field `SENSE_EN` reader - Enables the sensor and shield clocks, CSD modulator output and turns on the IDAC compensation current as selected by CSD_IDAC."]
pub type SENSE_EN_R = crate::BitReader;
#[doc = "Field `SENSE_EN` writer - Enables the sensor and shield clocks, CSD modulator output and turns on the IDAC compensation current as selected by CSD_IDAC."]
pub type SENSE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL_WAVE` reader - N/A"]
pub type FULL_WAVE_R = crate::BitReader<FULL_WAVE_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FULL_WAVE_A {
    #[doc = "0: Half Wave mode"]
    HALFWAVE = 0,
    #[doc = "1: Full Wave mode"]
    FULLWAVE = 1,
}
impl From<FULL_WAVE_A> for bool {
    #[inline(always)]
    fn from(variant: FULL_WAVE_A) -> Self {
        variant as u8 != 0
    }
}
impl FULL_WAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FULL_WAVE_A {
        match self.bits {
            false => FULL_WAVE_A::HALFWAVE,
            true => FULL_WAVE_A::FULLWAVE,
        }
    }
    #[doc = "Half Wave mode"]
    #[inline(always)]
    pub fn is_halfwave(&self) -> bool {
        *self == FULL_WAVE_A::HALFWAVE
    }
    #[doc = "Full Wave mode"]
    #[inline(always)]
    pub fn is_fullwave(&self) -> bool {
        *self == FULL_WAVE_A::FULLWAVE
    }
}
#[doc = "Field `FULL_WAVE` writer - N/A"]
pub type FULL_WAVE_W<'a, REG> = crate::BitWriter<'a, REG, FULL_WAVE_A>;
impl<'a, REG> FULL_WAVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half Wave mode"]
    #[inline(always)]
    pub fn halfwave(self) -> &'a mut crate::W<REG> {
        self.variant(FULL_WAVE_A::HALFWAVE)
    }
    #[doc = "Full Wave mode"]
    #[inline(always)]
    pub fn fullwave(self) -> &'a mut crate::W<REG> {
        self.variant(FULL_WAVE_A::FULLWAVE)
    }
}
#[doc = "Field `MUTUAL_CAP` reader - N/A"]
pub type MUTUAL_CAP_R = crate::BitReader<MUTUAL_CAP_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUTUAL_CAP_A {
    #[doc = "0: Self-cap mode"]
    SELFCAP = 0,
    #[doc = "1: Mutual-cap mode"]
    MUTUALCAP = 1,
}
impl From<MUTUAL_CAP_A> for bool {
    #[inline(always)]
    fn from(variant: MUTUAL_CAP_A) -> Self {
        variant as u8 != 0
    }
}
impl MUTUAL_CAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MUTUAL_CAP_A {
        match self.bits {
            false => MUTUAL_CAP_A::SELFCAP,
            true => MUTUAL_CAP_A::MUTUALCAP,
        }
    }
    #[doc = "Self-cap mode"]
    #[inline(always)]
    pub fn is_selfcap(&self) -> bool {
        *self == MUTUAL_CAP_A::SELFCAP
    }
    #[doc = "Mutual-cap mode"]
    #[inline(always)]
    pub fn is_mutualcap(&self) -> bool {
        *self == MUTUAL_CAP_A::MUTUALCAP
    }
}
#[doc = "Field `MUTUAL_CAP` writer - N/A"]
pub type MUTUAL_CAP_W<'a, REG> = crate::BitWriter<'a, REG, MUTUAL_CAP_A>;
impl<'a, REG> MUTUAL_CAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Self-cap mode"]
    #[inline(always)]
    pub fn selfcap(self) -> &'a mut crate::W<REG> {
        self.variant(MUTUAL_CAP_A::SELFCAP)
    }
    #[doc = "Mutual-cap mode"]
    #[inline(always)]
    pub fn mutualcap(self) -> &'a mut crate::W<REG> {
        self.variant(MUTUAL_CAP_A::MUTUALCAP)
    }
}
#[doc = "Field `CSX_DUAL_CNT` reader - N/A"]
pub type CSX_DUAL_CNT_R = crate::BitReader<CSX_DUAL_CNT_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSX_DUAL_CNT_A {
    #[doc = "0: N/A"]
    ONE = 0,
    #[doc = "1: N/A"]
    TWO = 1,
}
impl From<CSX_DUAL_CNT_A> for bool {
    #[inline(always)]
    fn from(variant: CSX_DUAL_CNT_A) -> Self {
        variant as u8 != 0
    }
}
impl CSX_DUAL_CNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSX_DUAL_CNT_A {
        match self.bits {
            false => CSX_DUAL_CNT_A::ONE,
            true => CSX_DUAL_CNT_A::TWO,
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSX_DUAL_CNT_A::ONE
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSX_DUAL_CNT_A::TWO
    }
}
#[doc = "Field `CSX_DUAL_CNT` writer - N/A"]
pub type CSX_DUAL_CNT_W<'a, REG> = crate::BitWriter<'a, REG, CSX_DUAL_CNT_A>;
impl<'a, REG> CSX_DUAL_CNT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "N/A"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(CSX_DUAL_CNT_A::ONE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(CSX_DUAL_CNT_A::TWO)
    }
}
#[doc = "Field `DSI_COUNT_SEL` reader - N/A"]
pub type DSI_COUNT_SEL_R = crate::BitReader<DSI_COUNT_SEL_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSI_COUNT_SEL_A {
    #[doc = "0: N/A"]
    CSD_RESULT = 0,
    #[doc = "1: N/A"]
    ADC_RESULT = 1,
}
impl From<DSI_COUNT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DSI_COUNT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DSI_COUNT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DSI_COUNT_SEL_A {
        match self.bits {
            false => DSI_COUNT_SEL_A::CSD_RESULT,
            true => DSI_COUNT_SEL_A::ADC_RESULT,
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_csd_result(&self) -> bool {
        *self == DSI_COUNT_SEL_A::CSD_RESULT
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_adc_result(&self) -> bool {
        *self == DSI_COUNT_SEL_A::ADC_RESULT
    }
}
#[doc = "Field `DSI_COUNT_SEL` writer - N/A"]
pub type DSI_COUNT_SEL_W<'a, REG> = crate::BitWriter<'a, REG, DSI_COUNT_SEL_A>;
impl<'a, REG> DSI_COUNT_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "N/A"]
    #[inline(always)]
    pub fn csd_result(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_COUNT_SEL_A::CSD_RESULT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn adc_result(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_COUNT_SEL_A::ADC_RESULT)
    }
}
#[doc = "Field `DSI_SAMPLE_EN` reader - DSI_SAMPLE_EN = 1 -> COUNTER will count the samples generated by DSI DSI_SAMPLE_EN = 0 -> COUNTER will count the samples generated by CSD modulator"]
pub type DSI_SAMPLE_EN_R = crate::BitReader;
#[doc = "Field `DSI_SAMPLE_EN` writer - DSI_SAMPLE_EN = 1 -> COUNTER will count the samples generated by DSI DSI_SAMPLE_EN = 0 -> COUNTER will count the samples generated by CSD modulator"]
pub type DSI_SAMPLE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLE_SYNC` reader - N/A"]
pub type SAMPLE_SYNC_R = crate::BitReader;
#[doc = "Field `SAMPLE_SYNC` writer - N/A"]
pub type SAMPLE_SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_SENSE_EN` reader - DSI_SENSE_EN = 1-> sensor clock is driven directly by DSI DSI_SENSE_EN = 0-> sensor clock is driven by PRS/divide-by-2/DIRECT_CLOCK"]
pub type DSI_SENSE_EN_R = crate::BitReader;
#[doc = "Field `DSI_SENSE_EN` writer - DSI_SENSE_EN = 1-> sensor clock is driven directly by DSI DSI_SENSE_EN = 0-> sensor clock is driven by PRS/divide-by-2/DIRECT_CLOCK"]
pub type DSI_SENSE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_MODE` reader - N/A"]
pub type LP_MODE_R = crate::BitReader;
#[doc = "Field `LP_MODE` writer - N/A"]
pub type LP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - N/A"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - N/A"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn iref_sel(&self) -> IREF_SEL_R {
        IREF_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:8 - Enables the digital filtering on the CSD comparator"]
    #[inline(always)]
    pub fn filter_delay(&self) -> FILTER_DELAY_R {
        FILTER_DELAY_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - Configures the delay between shield clock and sensor clock"]
    #[inline(always)]
    pub fn shield_delay(&self) -> SHIELD_DELAY_R {
        SHIELD_DELAY_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Enables the sensor and shield clocks, CSD modulator output and turns on the IDAC compensation current as selected by CSD_IDAC."]
    #[inline(always)]
    pub fn sense_en(&self) -> SENSE_EN_R {
        SENSE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn full_wave(&self) -> FULL_WAVE_R {
        FULL_WAVE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn mutual_cap(&self) -> MUTUAL_CAP_R {
        MUTUAL_CAP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    pub fn csx_dual_cnt(&self) -> CSX_DUAL_CNT_R {
        CSX_DUAL_CNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn dsi_count_sel(&self) -> DSI_COUNT_SEL_R {
        DSI_COUNT_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DSI_SAMPLE_EN = 1 -> COUNTER will count the samples generated by DSI DSI_SAMPLE_EN = 0 -> COUNTER will count the samples generated by CSD modulator"]
    #[inline(always)]
    pub fn dsi_sample_en(&self) -> DSI_SAMPLE_EN_R {
        DSI_SAMPLE_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn sample_sync(&self) -> SAMPLE_SYNC_R {
        SAMPLE_SYNC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DSI_SENSE_EN = 1-> sensor clock is driven directly by DSI DSI_SENSE_EN = 0-> sensor clock is driven by PRS/divide-by-2/DIRECT_CLOCK"]
    #[inline(always)]
    pub fn dsi_sense_en(&self) -> DSI_SENSE_EN_R {
        DSI_SENSE_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn lp_mode(&self) -> LP_MODE_R {
        LP_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn iref_sel(&mut self) -> IREF_SEL_W<CONFIG_SPEC> {
        IREF_SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:8 - Enables the digital filtering on the CSD comparator"]
    #[inline(always)]
    #[must_use]
    pub fn filter_delay(&mut self) -> FILTER_DELAY_W<CONFIG_SPEC> {
        FILTER_DELAY_W::new(self, 4)
    }
    #[doc = "Bits 10:11 - Configures the delay between shield clock and sensor clock"]
    #[inline(always)]
    #[must_use]
    pub fn shield_delay(&mut self) -> SHIELD_DELAY_W<CONFIG_SPEC> {
        SHIELD_DELAY_W::new(self, 10)
    }
    #[doc = "Bit 12 - Enables the sensor and shield clocks, CSD modulator output and turns on the IDAC compensation current as selected by CSD_IDAC."]
    #[inline(always)]
    #[must_use]
    pub fn sense_en(&mut self) -> SENSE_EN_W<CONFIG_SPEC> {
        SENSE_EN_W::new(self, 12)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn full_wave(&mut self) -> FULL_WAVE_W<CONFIG_SPEC> {
        FULL_WAVE_W::new(self, 17)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn mutual_cap(&mut self) -> MUTUAL_CAP_W<CONFIG_SPEC> {
        MUTUAL_CAP_W::new(self, 18)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn csx_dual_cnt(&mut self) -> CSX_DUAL_CNT_W<CONFIG_SPEC> {
        CSX_DUAL_CNT_W::new(self, 19)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_count_sel(&mut self) -> DSI_COUNT_SEL_W<CONFIG_SPEC> {
        DSI_COUNT_SEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - DSI_SAMPLE_EN = 1 -> COUNTER will count the samples generated by DSI DSI_SAMPLE_EN = 0 -> COUNTER will count the samples generated by CSD modulator"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_sample_en(&mut self) -> DSI_SAMPLE_EN_W<CONFIG_SPEC> {
        DSI_SAMPLE_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sample_sync(&mut self) -> SAMPLE_SYNC_W<CONFIG_SPEC> {
        SAMPLE_SYNC_W::new(self, 26)
    }
    #[doc = "Bit 27 - DSI_SENSE_EN = 1-> sensor clock is driven directly by DSI DSI_SENSE_EN = 0-> sensor clock is driven by PRS/divide-by-2/DIRECT_CLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_sense_en(&mut self) -> DSI_SENSE_EN_W<CONFIG_SPEC> {
        DSI_SENSE_EN_W::new(self, 27)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn lp_mode(&mut self) -> LP_MODE_W<CONFIG_SPEC> {
        LP_MODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CONFIG_SPEC> {
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
#[doc = "Configuration and Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x0400_0000"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0000;
}
