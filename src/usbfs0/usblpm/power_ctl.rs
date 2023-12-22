#[doc = "Register `POWER_CTL` reader"]
pub type R = crate::R<POWER_CTL_SPEC>;
#[doc = "Register `POWER_CTL` writer"]
pub type W = crate::W<POWER_CTL_SPEC>;
#[doc = "Field `SUSPEND` reader - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
pub type SUSPEND_R = crate::BitReader;
#[doc = "Field `SUSPEND` writer - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
pub type SUSPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_UP_EN` reader - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
pub type DP_UP_EN_R = crate::BitReader;
#[doc = "Field `DP_UP_EN` writer - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
pub type DP_UP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_BIG` reader - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
pub type DP_BIG_R = crate::BitReader;
#[doc = "Field `DP_BIG` writer - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
pub type DP_BIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_DOWN_EN` reader - Enables the ~15k pull down on the DP."]
pub type DP_DOWN_EN_R = crate::BitReader;
#[doc = "Field `DP_DOWN_EN` writer - Enables the ~15k pull down on the DP."]
pub type DP_DOWN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_UP_EN` reader - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
pub type DM_UP_EN_R = crate::BitReader;
#[doc = "Field `DM_UP_EN` writer - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
pub type DM_UP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_BIG` reader - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
pub type DM_BIG_R = crate::BitReader;
#[doc = "Field `DM_BIG` writer - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
pub type DM_BIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_DOWN_EN` reader - Enables the ~15k pull down on the DP."]
pub type DM_DOWN_EN_R = crate::BitReader;
#[doc = "Field `DM_DOWN_EN` writer - Enables the ~15k pull down on the DP."]
pub type DM_DOWN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_DPO` reader - Enables the single ended receiver on D+."]
pub type ENABLE_DPO_R = crate::BitReader;
#[doc = "Field `ENABLE_DPO` writer - Enables the single ended receiver on D+."]
pub type ENABLE_DPO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_DMO` reader - Enables the signle ended receiver on D-."]
pub type ENABLE_DMO_R = crate::BitReader;
#[doc = "Field `ENABLE_DMO` writer - Enables the signle ended receiver on D-."]
pub type ENABLE_DMO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn dp_up_en(&self) -> DP_UP_EN_R {
        DP_UP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
    #[inline(always)]
    pub fn dp_big(&self) -> DP_BIG_R {
        DP_BIG_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn dp_down_en(&self) -> DP_DOWN_EN_R {
        DP_DOWN_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn dm_up_en(&self) -> DM_UP_EN_R {
        DM_UP_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
    #[inline(always)]
    pub fn dm_big(&self) -> DM_BIG_R {
        DM_BIG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn dm_down_en(&self) -> DM_DOWN_EN_R {
        DM_DOWN_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - Enables the single ended receiver on D+."]
    #[inline(always)]
    pub fn enable_dpo(&self) -> ENABLE_DPO_R {
        ENABLE_DPO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enables the signle ended receiver on D-."]
    #[inline(always)]
    pub fn enable_dmo(&self) -> ENABLE_DMO_R {
        ENABLE_DMO_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SUSPEND_W<POWER_CTL_SPEC> {
        SUSPEND_W::new(self, 2)
    }
    #[doc = "Bit 16 - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    #[must_use]
    pub fn dp_up_en(&mut self) -> DP_UP_EN_W<POWER_CTL_SPEC> {
        DP_UP_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
    #[inline(always)]
    #[must_use]
    pub fn dp_big(&mut self) -> DP_BIG_W<POWER_CTL_SPEC> {
        DP_BIG_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    #[must_use]
    pub fn dp_down_en(&mut self) -> DP_DOWN_EN_W<POWER_CTL_SPEC> {
        DP_DOWN_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    #[must_use]
    pub fn dm_up_en(&mut self) -> DM_UP_EN_W<POWER_CTL_SPEC> {
        DM_UP_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
    #[inline(always)]
    #[must_use]
    pub fn dm_big(&mut self) -> DM_BIG_W<POWER_CTL_SPEC> {
        DM_BIG_W::new(self, 20)
    }
    #[doc = "Bit 21 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    #[must_use]
    pub fn dm_down_en(&mut self) -> DM_DOWN_EN_W<POWER_CTL_SPEC> {
        DM_DOWN_EN_W::new(self, 21)
    }
    #[doc = "Bit 28 - Enables the single ended receiver on D+."]
    #[inline(always)]
    #[must_use]
    pub fn enable_dpo(&mut self) -> ENABLE_DPO_W<POWER_CTL_SPEC> {
        ENABLE_DPO_W::new(self, 28)
    }
    #[doc = "Bit 29 - Enables the signle ended receiver on D-."]
    #[inline(always)]
    #[must_use]
    pub fn enable_dmo(&mut self) -> ENABLE_DMO_W<POWER_CTL_SPEC> {
        ENABLE_DMO_W::new(self, 29)
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
#[doc = "Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_CTL_SPEC;
impl crate::RegisterSpec for POWER_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_ctl::R`](R) reader structure"]
impl crate::Readable for POWER_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_ctl::W`](W) writer structure"]
impl crate::Writable for POWER_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER_CTL to value 0"]
impl crate::Resettable for POWER_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
