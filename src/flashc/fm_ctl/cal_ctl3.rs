#[doc = "Register `CAL_CTL3` reader"]
pub type R = crate::R<CAL_CTL3_SPEC>;
#[doc = "Register `CAL_CTL3` writer"]
pub type W = crate::W<CAL_CTL3_SPEC>;
#[doc = "Field `OSC_TRIM_HV` reader - Flash macro pump clock trim control."]
pub type OSC_TRIM_HV_R = crate::FieldReader;
#[doc = "Field `OSC_TRIM_HV` writer - Flash macro pump clock trim control."]
pub type OSC_TRIM_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OSC_RANGE_TRIM_HV` reader - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
pub type OSC_RANGE_TRIM_HV_R = crate::BitReader;
#[doc = "Field `OSC_RANGE_TRIM_HV` writer - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
pub type OSC_RANGE_TRIM_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPROT_ACT_HV` reader - Forces VPROT in active mode all the time"]
pub type VPROT_ACT_HV_R = crate::BitReader;
#[doc = "Field `VPROT_ACT_HV` writer - Forces VPROT in active mode all the time"]
pub type VPROT_ACT_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPREF_TC_HV` reader - 0: Increases the IPREF Tempco by subtracting ICREF from IPREF - IPREF internal will be 0.5uA 1: Reduces the IPREF Tempco without subtracting ICREF from IPREF - IPREF internal will be 1uA"]
pub type IPREF_TC_HV_R = crate::BitReader;
#[doc = "Field `IPREF_TC_HV` writer - 0: Increases the IPREF Tempco by subtracting ICREF from IPREF - IPREF internal will be 0.5uA 1: Reduces the IPREF Tempco without subtracting ICREF from IPREF - IPREF internal will be 1uA"]
pub type IPREF_TC_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_SEL_HV` reader - Voltage reference: 0: internal bandgap reference 1: external voltage reference"]
pub type VREF_SEL_HV_R = crate::BitReader;
#[doc = "Field `VREF_SEL_HV` writer - Voltage reference: 0: internal bandgap reference 1: external voltage reference"]
pub type VREF_SEL_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREF_SEL_HV` reader - Current reference: 0: internal current reference 1: external current reference"]
pub type IREF_SEL_HV_R = crate::BitReader;
#[doc = "Field `IREF_SEL_HV` writer - Current reference: 0: internal current reference 1: external current reference"]
pub type IREF_SEL_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_ACT_HV` reader - 0: VBST regulator will operate in active/standby mode based on control signal. 1: Forces the VBST regulator in active mode all the time"]
pub type REG_ACT_HV_R = crate::BitReader;
#[doc = "Field `REG_ACT_HV` writer - 0: VBST regulator will operate in active/standby mode based on control signal. 1: Forces the VBST regulator in active mode all the time"]
pub type REG_ACT_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDIV_TRIM_HV` reader - FDIV_TRIM_HV\\[1:0\\]: Assuming oscillator frequency of 8MHz in standby. Following are the clock frequencies seen by doubler 00: F = 1MHz 01: F = 0.5MHz 10: F = 2MHz 11: F = 4MHz"]
pub type FDIV_TRIM_HV_R = crate::FieldReader;
#[doc = "Field `FDIV_TRIM_HV` writer - FDIV_TRIM_HV\\[1:0\\]: Assuming oscillator frequency of 8MHz in standby. Following are the clock frequencies seen by doubler 00: F = 1MHz 01: F = 0.5MHz 10: F = 2MHz 11: F = 4MHz"]
pub type FDIV_TRIM_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VDDHI_HV` reader - 0: vdd &lt; 2.3V 1: vdd >= 2.3V '0' setting can used for vdd > 2.3V also, but with a current penalty."]
pub type VDDHI_HV_R = crate::BitReader;
#[doc = "Field `VDDHI_HV` writer - 0: vdd &lt; 2.3V 1: vdd >= 2.3V '0' setting can used for vdd > 2.3V also, but with a current penalty."]
pub type VDDHI_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TURBO_PULSEW_HV` reader - Turbo pulse width trim (Typical) 00: 40 us 01: 20 us 10: 15 us 11: 8 us"]
pub type TURBO_PULSEW_HV_R = crate::FieldReader;
#[doc = "Field `TURBO_PULSEW_HV` writer - Turbo pulse width trim (Typical) 00: 40 us 01: 20 us 10: 15 us 11: 8 us"]
pub type TURBO_PULSEW_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BGLO_EN_HV` reader - 0: Normal (Automatic change over from HI to LO) 1: Force enable LO Bandgap"]
pub type BGLO_EN_HV_R = crate::BitReader;
#[doc = "Field `BGLO_EN_HV` writer - 0: Normal (Automatic change over from HI to LO) 1: Force enable LO Bandgap"]
pub type BGLO_EN_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGHI_EN_HV` reader - 0: Normal (Automatic change over from HI to LO) 1: Force enable HI Bandgap When both BGLO_EN_HV and BGHI_EN_HV are HIGH, only BGHI output is used and turbo_hv_n pulse is active"]
pub type BGHI_EN_HV_R = crate::BitReader;
#[doc = "Field `BGHI_EN_HV` writer - 0: Normal (Automatic change over from HI to LO) 1: Force enable HI Bandgap When both BGLO_EN_HV and BGHI_EN_HV are HIGH, only BGHI output is used and turbo_hv_n pulse is active"]
pub type BGHI_EN_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CL_ISO_DIS_HV` reader - 0: The internal logic controls the CL isolation 1: Forces CL bypass"]
pub type CL_ISO_DIS_HV_R = crate::BitReader;
#[doc = "Field `CL_ISO_DIS_HV` writer - 0: The internal logic controls the CL isolation 1: Forces CL bypass"]
pub type CL_ISO_DIS_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R_GRANT_EN_HV` reader - 0: r_grant handshake disabled, r_grant always 1. 1: r_grand handshake enabled"]
pub type R_GRANT_EN_HV_R = crate::BitReader;
#[doc = "Field `R_GRANT_EN_HV` writer - 0: r_grant handshake disabled, r_grant always 1. 1: r_grand handshake enabled"]
pub type R_GRANT_EN_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ULP_SW_HV` reader - LP&lt;-->ULP switch for trim signals: 0: LP 1: ULP"]
pub type LP_ULP_SW_HV_R = crate::BitReader;
#[doc = "Field `LP_ULP_SW_HV` writer - LP&lt;-->ULP switch for trim signals: 0: LP 1: ULP"]
pub type LP_ULP_SW_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Flash macro pump clock trim control."]
    #[inline(always)]
    pub fn osc_trim_hv(&self) -> OSC_TRIM_HV_R {
        OSC_TRIM_HV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
    #[inline(always)]
    pub fn osc_range_trim_hv(&self) -> OSC_RANGE_TRIM_HV_R {
        OSC_RANGE_TRIM_HV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Forces VPROT in active mode all the time"]
    #[inline(always)]
    pub fn vprot_act_hv(&self) -> VPROT_ACT_HV_R {
        VPROT_ACT_HV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: Increases the IPREF Tempco by subtracting ICREF from IPREF - IPREF internal will be 0.5uA 1: Reduces the IPREF Tempco without subtracting ICREF from IPREF - IPREF internal will be 1uA"]
    #[inline(always)]
    pub fn ipref_tc_hv(&self) -> IPREF_TC_HV_R {
        IPREF_TC_HV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Voltage reference: 0: internal bandgap reference 1: external voltage reference"]
    #[inline(always)]
    pub fn vref_sel_hv(&self) -> VREF_SEL_HV_R {
        VREF_SEL_HV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Current reference: 0: internal current reference 1: external current reference"]
    #[inline(always)]
    pub fn iref_sel_hv(&self) -> IREF_SEL_HV_R {
        IREF_SEL_HV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: VBST regulator will operate in active/standby mode based on control signal. 1: Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    pub fn reg_act_hv(&self) -> REG_ACT_HV_R {
        REG_ACT_HV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - FDIV_TRIM_HV\\[1:0\\]: Assuming oscillator frequency of 8MHz in standby. Following are the clock frequencies seen by doubler 00: F = 1MHz 01: F = 0.5MHz 10: F = 2MHz 11: F = 4MHz"]
    #[inline(always)]
    pub fn fdiv_trim_hv(&self) -> FDIV_TRIM_HV_R {
        FDIV_TRIM_HV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 0: vdd &lt; 2.3V 1: vdd >= 2.3V '0' setting can used for vdd > 2.3V also, but with a current penalty."]
    #[inline(always)]
    pub fn vddhi_hv(&self) -> VDDHI_HV_R {
        VDDHI_HV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Turbo pulse width trim (Typical) 00: 40 us 01: 20 us 10: 15 us 11: 8 us"]
    #[inline(always)]
    pub fn turbo_pulsew_hv(&self) -> TURBO_PULSEW_HV_R {
        TURBO_PULSEW_HV_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 0: Normal (Automatic change over from HI to LO) 1: Force enable LO Bandgap"]
    #[inline(always)]
    pub fn bglo_en_hv(&self) -> BGLO_EN_HV_R {
        BGLO_EN_HV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 0: Normal (Automatic change over from HI to LO) 1: Force enable HI Bandgap When both BGLO_EN_HV and BGHI_EN_HV are HIGH, only BGHI output is used and turbo_hv_n pulse is active"]
    #[inline(always)]
    pub fn bghi_en_hv(&self) -> BGHI_EN_HV_R {
        BGHI_EN_HV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: The internal logic controls the CL isolation 1: Forces CL bypass"]
    #[inline(always)]
    pub fn cl_iso_dis_hv(&self) -> CL_ISO_DIS_HV_R {
        CL_ISO_DIS_HV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 0: r_grant handshake disabled, r_grant always 1. 1: r_grand handshake enabled"]
    #[inline(always)]
    pub fn r_grant_en_hv(&self) -> R_GRANT_EN_HV_R {
        R_GRANT_EN_HV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LP&lt;-->ULP switch for trim signals: 0: LP 1: ULP"]
    #[inline(always)]
    pub fn lp_ulp_sw_hv(&self) -> LP_ULP_SW_HV_R {
        LP_ULP_SW_HV_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flash macro pump clock trim control."]
    #[inline(always)]
    #[must_use]
    pub fn osc_trim_hv(&mut self) -> OSC_TRIM_HV_W<CAL_CTL3_SPEC> {
        OSC_TRIM_HV_W::new(self, 0)
    }
    #[doc = "Bit 4 - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
    #[inline(always)]
    #[must_use]
    pub fn osc_range_trim_hv(&mut self) -> OSC_RANGE_TRIM_HV_W<CAL_CTL3_SPEC> {
        OSC_RANGE_TRIM_HV_W::new(self, 4)
    }
    #[doc = "Bit 5 - Forces VPROT in active mode all the time"]
    #[inline(always)]
    #[must_use]
    pub fn vprot_act_hv(&mut self) -> VPROT_ACT_HV_W<CAL_CTL3_SPEC> {
        VPROT_ACT_HV_W::new(self, 5)
    }
    #[doc = "Bit 6 - 0: Increases the IPREF Tempco by subtracting ICREF from IPREF - IPREF internal will be 0.5uA 1: Reduces the IPREF Tempco without subtracting ICREF from IPREF - IPREF internal will be 1uA"]
    #[inline(always)]
    #[must_use]
    pub fn ipref_tc_hv(&mut self) -> IPREF_TC_HV_W<CAL_CTL3_SPEC> {
        IPREF_TC_HV_W::new(self, 6)
    }
    #[doc = "Bit 7 - Voltage reference: 0: internal bandgap reference 1: external voltage reference"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel_hv(&mut self) -> VREF_SEL_HV_W<CAL_CTL3_SPEC> {
        VREF_SEL_HV_W::new(self, 7)
    }
    #[doc = "Bit 8 - Current reference: 0: internal current reference 1: external current reference"]
    #[inline(always)]
    #[must_use]
    pub fn iref_sel_hv(&mut self) -> IREF_SEL_HV_W<CAL_CTL3_SPEC> {
        IREF_SEL_HV_W::new(self, 8)
    }
    #[doc = "Bit 9 - 0: VBST regulator will operate in active/standby mode based on control signal. 1: Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    #[must_use]
    pub fn reg_act_hv(&mut self) -> REG_ACT_HV_W<CAL_CTL3_SPEC> {
        REG_ACT_HV_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - FDIV_TRIM_HV\\[1:0\\]: Assuming oscillator frequency of 8MHz in standby. Following are the clock frequencies seen by doubler 00: F = 1MHz 01: F = 0.5MHz 10: F = 2MHz 11: F = 4MHz"]
    #[inline(always)]
    #[must_use]
    pub fn fdiv_trim_hv(&mut self) -> FDIV_TRIM_HV_W<CAL_CTL3_SPEC> {
        FDIV_TRIM_HV_W::new(self, 10)
    }
    #[doc = "Bit 12 - 0: vdd &lt; 2.3V 1: vdd >= 2.3V '0' setting can used for vdd > 2.3V also, but with a current penalty."]
    #[inline(always)]
    #[must_use]
    pub fn vddhi_hv(&mut self) -> VDDHI_HV_W<CAL_CTL3_SPEC> {
        VDDHI_HV_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - Turbo pulse width trim (Typical) 00: 40 us 01: 20 us 10: 15 us 11: 8 us"]
    #[inline(always)]
    #[must_use]
    pub fn turbo_pulsew_hv(&mut self) -> TURBO_PULSEW_HV_W<CAL_CTL3_SPEC> {
        TURBO_PULSEW_HV_W::new(self, 13)
    }
    #[doc = "Bit 15 - 0: Normal (Automatic change over from HI to LO) 1: Force enable LO Bandgap"]
    #[inline(always)]
    #[must_use]
    pub fn bglo_en_hv(&mut self) -> BGLO_EN_HV_W<CAL_CTL3_SPEC> {
        BGLO_EN_HV_W::new(self, 15)
    }
    #[doc = "Bit 16 - 0: Normal (Automatic change over from HI to LO) 1: Force enable HI Bandgap When both BGLO_EN_HV and BGHI_EN_HV are HIGH, only BGHI output is used and turbo_hv_n pulse is active"]
    #[inline(always)]
    #[must_use]
    pub fn bghi_en_hv(&mut self) -> BGHI_EN_HV_W<CAL_CTL3_SPEC> {
        BGHI_EN_HV_W::new(self, 16)
    }
    #[doc = "Bit 17 - 0: The internal logic controls the CL isolation 1: Forces CL bypass"]
    #[inline(always)]
    #[must_use]
    pub fn cl_iso_dis_hv(&mut self) -> CL_ISO_DIS_HV_W<CAL_CTL3_SPEC> {
        CL_ISO_DIS_HV_W::new(self, 17)
    }
    #[doc = "Bit 18 - 0: r_grant handshake disabled, r_grant always 1. 1: r_grand handshake enabled"]
    #[inline(always)]
    #[must_use]
    pub fn r_grant_en_hv(&mut self) -> R_GRANT_EN_HV_W<CAL_CTL3_SPEC> {
        R_GRANT_EN_HV_W::new(self, 18)
    }
    #[doc = "Bit 19 - LP&lt;-->ULP switch for trim signals: 0: LP 1: ULP"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ulp_sw_hv(&mut self) -> LP_ULP_SW_HV_W<CAL_CTL3_SPEC> {
        LP_ULP_SW_HV_W::new(self, 19)
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
#[doc = "Cal control osc trim bits, idac, sdac, itim\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAL_CTL3_SPEC;
impl crate::RegisterSpec for CAL_CTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl3::R`](R) reader structure"]
impl crate::Readable for CAL_CTL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl3::W`](W) writer structure"]
impl crate::Writable for CAL_CTL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_CTL3 to value 0x2004"]
impl crate::Resettable for CAL_CTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0x2004;
}
