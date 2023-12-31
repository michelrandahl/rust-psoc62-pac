#[doc = "Register `CLK_ILO_CONFIG` reader"]
pub type R = crate::R<CLK_ILO_CONFIG_SPEC>;
#[doc = "Register `CLK_ILO_CONFIG` writer"]
pub type W = crate::W<CLK_ILO_CONFIG_SPEC>;
#[doc = "Field `ILO_BACKUP` reader - If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
pub type ILO_BACKUP_R = crate::BitReader;
#[doc = "Field `ILO_BACKUP` writer - If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
pub type ILO_BACKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
    #[inline(always)]
    pub fn ilo_backup(&self) -> ILO_BACKUP_R {
        ILO_BACKUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
    #[inline(always)]
    #[must_use]
    pub fn ilo_backup(&mut self) -> ILO_BACKUP_W<CLK_ILO_CONFIG_SPEC> {
        ILO_BACKUP_W::new(self, 0)
    }
    #[doc = "Bit 31 - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CLK_ILO_CONFIG_SPEC> {
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
#[doc = "ILO Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ilo_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ilo_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_ILO_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_ILO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ilo_config::R`](R) reader structure"]
impl crate::Readable for CLK_ILO_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_ilo_config::W`](W) writer structure"]
impl crate::Writable for CLK_ILO_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_ILO_CONFIG to value 0x8000_0000"]
impl crate::Resettable for CLK_ILO_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
