#[doc = "Register `CAL_CTL4` reader"]
pub type R = crate::R<CAL_CTL4_SPEC>;
#[doc = "Register `CAL_CTL4` writer"]
pub type W = crate::W<CAL_CTL4_SPEC>;
#[doc = "Field `VLIM_TRIM_ULP_HV` reader - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
pub type VLIM_TRIM_ULP_HV_R = crate::FieldReader;
#[doc = "Field `VLIM_TRIM_ULP_HV` writer - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
pub type VLIM_TRIM_ULP_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDAC_ULP_HV` reader - Sets the sense current reference offset value. Refer to trim tables for details."]
pub type IDAC_ULP_HV_R = crate::FieldReader;
#[doc = "Field `IDAC_ULP_HV` writer - Sets the sense current reference offset value. Refer to trim tables for details."]
pub type IDAC_ULP_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDAC_ULP_HV` reader - Sets the sense current reference temp slope. Refer to trim tables for details."]
pub type SDAC_ULP_HV_R = crate::FieldReader;
#[doc = "Field `SDAC_ULP_HV` writer - Sets the sense current reference temp slope. Refer to trim tables for details."]
pub type SDAC_ULP_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ITIM_ULP_HV` reader - Trimming of timing current"]
pub type ITIM_ULP_HV_R = crate::FieldReader;
#[doc = "Field `ITIM_ULP_HV` writer - Trimming of timing current"]
pub type ITIM_ULP_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FM_READY_DEL_ULP_HV` reader - 00: Default : delay 1ns 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
pub type FM_READY_DEL_ULP_HV_R = crate::FieldReader;
#[doc = "Field `FM_READY_DEL_ULP_HV` writer - 00: Default : delay 1ns 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
pub type FM_READY_DEL_ULP_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPARE451_ULP_HV` reader - N/A"]
pub type SPARE451_ULP_HV_R = crate::BitReader;
#[doc = "Field `SPARE451_ULP_HV` writer - N/A"]
pub type SPARE451_ULP_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READY_RESTART_N_HV` reader - Toggle: 1-->0, ready goes low, ready will remain low as long as the bit is low. Toggle the bit back to 1 to activate the ready logic. To be used by API only."]
pub type READY_RESTART_N_HV_R = crate::BitReader;
#[doc = "Field `READY_RESTART_N_HV` writer - Toggle: 1-->0, ready goes low, ready will remain low as long as the bit is low. Toggle the bit back to 1 to activate the ready logic. To be used by API only."]
pub type READY_RESTART_N_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBST_S_DIS_HV` reader - 0: VBST_S voltage for each sector to allow VBST level to be dropped to VCC during Erase in the selected sector, reducing coupling to GBL. 1: VBST_S voltage for each sector stays at VBST level during Erase in the selected sector."]
pub type VBST_S_DIS_HV_R = crate::BitReader;
#[doc = "Field `VBST_S_DIS_HV` writer - 0: VBST_S voltage for each sector to allow VBST level to be dropped to VCC during Erase in the selected sector, reducing coupling to GBL. 1: VBST_S voltage for each sector stays at VBST level during Erase in the selected sector."]
pub type VBST_S_DIS_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_HVPULSE_HV` reader - 0: HV Pulse controlled by FW 1: HV Pulse controlled by Hardware"]
pub type AUTO_HVPULSE_HV_R = crate::BitReader;
#[doc = "Field `AUTO_HVPULSE_HV` writer - 0: HV Pulse controlled by FW 1: HV Pulse controlled by Hardware"]
pub type AUTO_HVPULSE_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UGB_EN_HV` reader - UGB enable in TM control"]
pub type UGB_EN_HV_R = crate::BitReader;
#[doc = "Field `UGB_EN_HV` writer - UGB enable in TM control"]
pub type UGB_EN_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_ulp_hv(&self) -> VLIM_TRIM_ULP_HV_R {
        VLIM_TRIM_ULP_HV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    pub fn idac_ulp_hv(&self) -> IDAC_ULP_HV_R {
        IDAC_ULP_HV_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    pub fn sdac_ulp_hv(&self) -> SDAC_ULP_HV_R {
        SDAC_ULP_HV_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Trimming of timing current"]
    #[inline(always)]
    pub fn itim_ulp_hv(&self) -> ITIM_ULP_HV_R {
        ITIM_ULP_HV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - 00: Default : delay 1ns 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    pub fn fm_ready_del_ulp_hv(&self) -> FM_READY_DEL_ULP_HV_R {
        FM_READY_DEL_ULP_HV_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn spare451_ulp_hv(&self) -> SPARE451_ULP_HV_R {
        SPARE451_ULP_HV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Toggle: 1-->0, ready goes low, ready will remain low as long as the bit is low. Toggle the bit back to 1 to activate the ready logic. To be used by API only."]
    #[inline(always)]
    pub fn ready_restart_n_hv(&self) -> READY_RESTART_N_HV_R {
        READY_RESTART_N_HV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: VBST_S voltage for each sector to allow VBST level to be dropped to VCC during Erase in the selected sector, reducing coupling to GBL. 1: VBST_S voltage for each sector stays at VBST level during Erase in the selected sector."]
    #[inline(always)]
    pub fn vbst_s_dis_hv(&self) -> VBST_S_DIS_HV_R {
        VBST_S_DIS_HV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 0: HV Pulse controlled by FW 1: HV Pulse controlled by Hardware"]
    #[inline(always)]
    pub fn auto_hvpulse_hv(&self) -> AUTO_HVPULSE_HV_R {
        AUTO_HVPULSE_HV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UGB enable in TM control"]
    #[inline(always)]
    pub fn ugb_en_hv(&self) -> UGB_EN_HV_R {
        UGB_EN_HV_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    #[must_use]
    pub fn vlim_trim_ulp_hv(&mut self) -> VLIM_TRIM_ULP_HV_W<CAL_CTL4_SPEC> {
        VLIM_TRIM_ULP_HV_W::new(self, 0)
    }
    #[doc = "Bits 2:5 - Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    #[must_use]
    pub fn idac_ulp_hv(&mut self) -> IDAC_ULP_HV_W<CAL_CTL4_SPEC> {
        IDAC_ULP_HV_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    #[must_use]
    pub fn sdac_ulp_hv(&mut self) -> SDAC_ULP_HV_W<CAL_CTL4_SPEC> {
        SDAC_ULP_HV_W::new(self, 6)
    }
    #[doc = "Bits 8:12 - Trimming of timing current"]
    #[inline(always)]
    #[must_use]
    pub fn itim_ulp_hv(&mut self) -> ITIM_ULP_HV_W<CAL_CTL4_SPEC> {
        ITIM_ULP_HV_W::new(self, 8)
    }
    #[doc = "Bits 13:14 - 00: Default : delay 1ns 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    #[must_use]
    pub fn fm_ready_del_ulp_hv(&mut self) -> FM_READY_DEL_ULP_HV_W<CAL_CTL4_SPEC> {
        FM_READY_DEL_ULP_HV_W::new(self, 13)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn spare451_ulp_hv(&mut self) -> SPARE451_ULP_HV_W<CAL_CTL4_SPEC> {
        SPARE451_ULP_HV_W::new(self, 15)
    }
    #[doc = "Bit 16 - Toggle: 1-->0, ready goes low, ready will remain low as long as the bit is low. Toggle the bit back to 1 to activate the ready logic. To be used by API only."]
    #[inline(always)]
    #[must_use]
    pub fn ready_restart_n_hv(&mut self) -> READY_RESTART_N_HV_W<CAL_CTL4_SPEC> {
        READY_RESTART_N_HV_W::new(self, 16)
    }
    #[doc = "Bit 17 - 0: VBST_S voltage for each sector to allow VBST level to be dropped to VCC during Erase in the selected sector, reducing coupling to GBL. 1: VBST_S voltage for each sector stays at VBST level during Erase in the selected sector."]
    #[inline(always)]
    #[must_use]
    pub fn vbst_s_dis_hv(&mut self) -> VBST_S_DIS_HV_W<CAL_CTL4_SPEC> {
        VBST_S_DIS_HV_W::new(self, 17)
    }
    #[doc = "Bit 18 - 0: HV Pulse controlled by FW 1: HV Pulse controlled by Hardware"]
    #[inline(always)]
    #[must_use]
    pub fn auto_hvpulse_hv(&mut self) -> AUTO_HVPULSE_HV_W<CAL_CTL4_SPEC> {
        AUTO_HVPULSE_HV_W::new(self, 18)
    }
    #[doc = "Bit 19 - UGB enable in TM control"]
    #[inline(always)]
    #[must_use]
    pub fn ugb_en_hv(&mut self) -> UGB_EN_HV_W<CAL_CTL4_SPEC> {
        UGB_EN_HV_W::new(self, 19)
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
#[doc = "Cal Control Vlim, SA, fdiv, reg_act\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAL_CTL4_SPEC;
impl crate::RegisterSpec for CAL_CTL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl4::R`](R) reader structure"]
impl crate::Readable for CAL_CTL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl4::W`](W) writer structure"]
impl crate::Writable for CAL_CTL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_CTL4 to value 0x0001_2ae0"]
impl crate::Resettable for CAL_CTL4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_2ae0;
}
