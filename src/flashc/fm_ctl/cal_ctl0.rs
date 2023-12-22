#[doc = "Register `CAL_CTL0` reader"]
pub type R = crate::R<CAL_CTL0_SPEC>;
#[doc = "Register `CAL_CTL0` writer"]
pub type W = crate::W<CAL_CTL0_SPEC>;
#[doc = "Field `VCT_TRIM_LO_HV` reader - LO Bandgap Voltage Temperature Compensation trim control."]
pub type VCT_TRIM_LO_HV_R = crate::FieldReader;
#[doc = "Field `VCT_TRIM_LO_HV` writer - LO Bandgap Voltage Temperature Compensation trim control."]
pub type VCT_TRIM_LO_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CDAC_LO_HV` reader - LO Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
pub type CDAC_LO_HV_R = crate::FieldReader;
#[doc = "Field `CDAC_LO_HV` writer - LO Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
pub type CDAC_LO_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VBG_TRIM_LO_HV` reader - LO Bandgap Voltage trim control."]
pub type VBG_TRIM_LO_HV_R = crate::FieldReader;
#[doc = "Field `VBG_TRIM_LO_HV` writer - LO Bandgap Voltage trim control."]
pub type VBG_TRIM_LO_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VBG_TC_TRIM_LO_HV` reader - LO Bandgap Voltage Temperature Compensation trim control"]
pub type VBG_TC_TRIM_LO_HV_R = crate::FieldReader;
#[doc = "Field `VBG_TC_TRIM_LO_HV` writer - LO Bandgap Voltage Temperature Compensation trim control"]
pub type VBG_TC_TRIM_LO_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ICREF_TC_TRIM_LO_HV` reader - LO Bandgap Current Temperature Compensation trim control"]
pub type ICREF_TC_TRIM_LO_HV_R = crate::FieldReader;
#[doc = "Field `ICREF_TC_TRIM_LO_HV` writer - LO Bandgap Current Temperature Compensation trim control"]
pub type ICREF_TC_TRIM_LO_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IPREF_TRIMA_LO_HV` reader - Adds 100-150nA boost on IPREF_LO"]
pub type IPREF_TRIMA_LO_HV_R = crate::BitReader;
#[doc = "Field `IPREF_TRIMA_LO_HV` writer - Adds 100-150nA boost on IPREF_LO"]
pub type IPREF_TRIMA_LO_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - LO Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub fn vct_trim_lo_hv(&self) -> VCT_TRIM_LO_HV_R {
        VCT_TRIM_LO_HV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - LO Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    pub fn cdac_lo_hv(&self) -> CDAC_LO_HV_R {
        CDAC_LO_HV_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - LO Bandgap Voltage trim control."]
    #[inline(always)]
    pub fn vbg_trim_lo_hv(&self) -> VBG_TRIM_LO_HV_R {
        VBG_TRIM_LO_HV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - LO Bandgap Voltage Temperature Compensation trim control"]
    #[inline(always)]
    pub fn vbg_tc_trim_lo_hv(&self) -> VBG_TC_TRIM_LO_HV_R {
        VBG_TC_TRIM_LO_HV_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    pub fn icref_tc_trim_lo_hv(&self) -> ICREF_TC_TRIM_LO_HV_R {
        ICREF_TC_TRIM_LO_HV_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Adds 100-150nA boost on IPREF_LO"]
    #[inline(always)]
    pub fn ipref_trima_lo_hv(&self) -> IPREF_TRIMA_LO_HV_R {
        IPREF_TRIMA_LO_HV_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - LO Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    #[must_use]
    pub fn vct_trim_lo_hv(&mut self) -> VCT_TRIM_LO_HV_W<CAL_CTL0_SPEC> {
        VCT_TRIM_LO_HV_W::new(self, 0)
    }
    #[doc = "Bits 5:7 - LO Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    #[must_use]
    pub fn cdac_lo_hv(&mut self) -> CDAC_LO_HV_W<CAL_CTL0_SPEC> {
        CDAC_LO_HV_W::new(self, 5)
    }
    #[doc = "Bits 8:12 - LO Bandgap Voltage trim control."]
    #[inline(always)]
    #[must_use]
    pub fn vbg_trim_lo_hv(&mut self) -> VBG_TRIM_LO_HV_W<CAL_CTL0_SPEC> {
        VBG_TRIM_LO_HV_W::new(self, 8)
    }
    #[doc = "Bits 13:15 - LO Bandgap Voltage Temperature Compensation trim control"]
    #[inline(always)]
    #[must_use]
    pub fn vbg_tc_trim_lo_hv(&mut self) -> VBG_TC_TRIM_LO_HV_W<CAL_CTL0_SPEC> {
        VBG_TC_TRIM_LO_HV_W::new(self, 13)
    }
    #[doc = "Bits 16:18 - LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    #[must_use]
    pub fn icref_tc_trim_lo_hv(&mut self) -> ICREF_TC_TRIM_LO_HV_W<CAL_CTL0_SPEC> {
        ICREF_TC_TRIM_LO_HV_W::new(self, 16)
    }
    #[doc = "Bit 19 - Adds 100-150nA boost on IPREF_LO"]
    #[inline(always)]
    #[must_use]
    pub fn ipref_trima_lo_hv(&mut self) -> IPREF_TRIMA_LO_HV_W<CAL_CTL0_SPEC> {
        IPREF_TRIMA_LO_HV_W::new(self, 19)
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
#[doc = "Cal control BG LO trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAL_CTL0_SPEC;
impl crate::RegisterSpec for CAL_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl0::R`](R) reader structure"]
impl crate::Readable for CAL_CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl0::W`](W) writer structure"]
impl crate::Writable for CAL_CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_CTL0 to value 0x0003_8f8f"]
impl crate::Resettable for CAL_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_8f8f;
}
