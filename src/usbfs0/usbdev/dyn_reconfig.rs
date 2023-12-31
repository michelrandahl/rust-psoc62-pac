#[doc = "Register `DYN_RECONFIG` reader"]
pub type R = crate::R<DYN_RECONFIG_SPEC>;
#[doc = "Register `DYN_RECONFIG` writer"]
pub type W = crate::W<DYN_RECONFIG_SPEC>;
#[doc = "Field `DYN_CONFIG_EN` reader - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
pub type DYN_CONFIG_EN_R = crate::BitReader;
#[doc = "Field `DYN_CONFIG_EN` writer - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
pub type DYN_CONFIG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DYN_RECONFIG_EPNO` reader - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
pub type DYN_RECONFIG_EPNO_R = crate::FieldReader;
#[doc = "Field `DYN_RECONFIG_EPNO` writer - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
pub type DYN_RECONFIG_EPNO_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DYN_RECONFIG_RDY_STS` reader - This bit indicates the ready status for the dynamic reconfiguration, when set to 1, indicates the block is ready for reconfiguration."]
pub type DYN_RECONFIG_RDY_STS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
    #[inline(always)]
    pub fn dyn_config_en(&self) -> DYN_CONFIG_EN_R {
        DYN_CONFIG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
    #[inline(always)]
    pub fn dyn_reconfig_epno(&self) -> DYN_RECONFIG_EPNO_R {
        DYN_RECONFIG_EPNO_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - This bit indicates the ready status for the dynamic reconfiguration, when set to 1, indicates the block is ready for reconfiguration."]
    #[inline(always)]
    pub fn dyn_reconfig_rdy_sts(&self) -> DYN_RECONFIG_RDY_STS_R {
        DYN_RECONFIG_RDY_STS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
    #[inline(always)]
    #[must_use]
    pub fn dyn_config_en(&mut self) -> DYN_CONFIG_EN_W<DYN_RECONFIG_SPEC> {
        DYN_CONFIG_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn dyn_reconfig_epno(&mut self) -> DYN_RECONFIG_EPNO_W<DYN_RECONFIG_SPEC> {
        DYN_RECONFIG_EPNO_W::new(self, 1)
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
#[doc = "USB Dynamic reconfiguration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dyn_reconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dyn_reconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DYN_RECONFIG_SPEC;
impl crate::RegisterSpec for DYN_RECONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dyn_reconfig::R`](R) reader structure"]
impl crate::Readable for DYN_RECONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dyn_reconfig::W`](W) writer structure"]
impl crate::Writable for DYN_RECONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DYN_RECONFIG to value 0"]
impl crate::Resettable for DYN_RECONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
