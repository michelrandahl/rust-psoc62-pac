#[doc = "Register `SENSE_DUTY` reader"]
pub type R = crate::R<SENSE_DUTY_SPEC>;
#[doc = "Register `SENSE_DUTY` writer"]
pub type W = crate::W<SENSE_DUTY_SPEC>;
#[doc = "Field `SENSE_WIDTH` reader - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT &lt;&lt; LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
pub type SENSE_WIDTH_R = crate::FieldReader<u16>;
#[doc = "Field `SENSE_WIDTH` writer - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT &lt;&lt; LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
pub type SENSE_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SENSE_POL` reader - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
pub type SENSE_POL_R = crate::BitReader;
#[doc = "Field `SENSE_POL` writer - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
pub type SENSE_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERLAP_PHI1` reader - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
pub type OVERLAP_PHI1_R = crate::BitReader;
#[doc = "Field `OVERLAP_PHI1` writer - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
pub type OVERLAP_PHI1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERLAP_PHI2` reader - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
pub type OVERLAP_PHI2_R = crate::BitReader;
#[doc = "Field `OVERLAP_PHI2` writer - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
pub type OVERLAP_PHI2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT &lt;&lt; LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
    #[inline(always)]
    pub fn sense_width(&self) -> SENSE_WIDTH_R {
        SENSE_WIDTH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
    #[inline(always)]
    pub fn sense_pol(&self) -> SENSE_POL_R {
        SENSE_POL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
    #[inline(always)]
    pub fn overlap_phi1(&self) -> OVERLAP_PHI1_R {
        OVERLAP_PHI1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
    #[inline(always)]
    pub fn overlap_phi2(&self) -> OVERLAP_PHI2_R {
        OVERLAP_PHI2_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT &lt;&lt; LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
    #[inline(always)]
    #[must_use]
    pub fn sense_width(&mut self) -> SENSE_WIDTH_W<SENSE_DUTY_SPEC> {
        SENSE_WIDTH_W::new(self, 0)
    }
    #[doc = "Bit 16 - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
    #[inline(always)]
    #[must_use]
    pub fn sense_pol(&mut self) -> SENSE_POL_W<SENSE_DUTY_SPEC> {
        SENSE_POL_W::new(self, 16)
    }
    #[doc = "Bit 18 - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
    #[inline(always)]
    #[must_use]
    pub fn overlap_phi1(&mut self) -> OVERLAP_PHI1_W<SENSE_DUTY_SPEC> {
        OVERLAP_PHI1_W::new(self, 18)
    }
    #[doc = "Bit 19 - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
    #[inline(always)]
    #[must_use]
    pub fn overlap_phi2(&mut self) -> OVERLAP_PHI2_W<SENSE_DUTY_SPEC> {
        OVERLAP_PHI2_W::new(self, 19)
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
#[doc = "Sense clock duty cycle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sense_duty::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sense_duty::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SENSE_DUTY_SPEC;
impl crate::RegisterSpec for SENSE_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sense_duty::R`](R) reader structure"]
impl crate::Readable for SENSE_DUTY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sense_duty::W`](W) writer structure"]
impl crate::Writable for SENSE_DUTY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SENSE_DUTY to value 0"]
impl crate::Resettable for SENSE_DUTY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
