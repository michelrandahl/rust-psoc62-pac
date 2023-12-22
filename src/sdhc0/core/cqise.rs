#[doc = "Register `CQISE` reader"]
pub type R = crate::R<CQISE_SPEC>;
#[doc = "Register `CQISE` writer"]
pub type W = crate::W<CQISE_SPEC>;
#[doc = "Field `HAC_STE` reader - Halt complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
pub type HAC_STE_R = crate::BitReader;
#[doc = "Field `HAC_STE` writer - Halt complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
pub type HAC_STE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC_STE` reader - Task complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
pub type TCC_STE_R = crate::BitReader;
#[doc = "Field `TCC_STE` writer - Task complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
pub type TCC_STE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_STE` reader - Response error detected interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
pub type RED_STE_R = crate::BitReader;
#[doc = "Field `RED_STE` writer - Response error detected interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
pub type RED_STE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCL_STE` reader - Task cleared interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
pub type TCL_STE_R = crate::BitReader;
#[doc = "Field `TCL_STE` writer - Task cleared interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
pub type TCL_STE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCE_STE` reader - General Crypto Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.GCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.GCE is disabled"]
pub type GCE_STE_R = crate::BitReader;
#[doc = "Field `GCE_STE` writer - General Crypto Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.GCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.GCE is disabled"]
pub type GCE_STE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICCE_STE` reader - Invalid Crypto Configuration Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.ICCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.ICCE is disabled"]
pub type ICCE_STE_R = crate::BitReader;
#[doc = "Field `ICCE_STE` writer - Invalid Crypto Configuration Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.ICCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.ICCE is disabled"]
pub type ICCE_STE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Halt complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
    #[inline(always)]
    pub fn hac_ste(&self) -> HAC_STE_R {
        HAC_STE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Task complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
    #[inline(always)]
    pub fn tcc_ste(&self) -> TCC_STE_R {
        TCC_STE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Response error detected interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
    #[inline(always)]
    pub fn red_ste(&self) -> RED_STE_R {
        RED_STE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Task cleared interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
    #[inline(always)]
    pub fn tcl_ste(&self) -> TCL_STE_R {
        TCL_STE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - General Crypto Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.GCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.GCE is disabled"]
    #[inline(always)]
    pub fn gce_ste(&self) -> GCE_STE_R {
        GCE_STE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invalid Crypto Configuration Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.ICCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.ICCE is disabled"]
    #[inline(always)]
    pub fn icce_ste(&self) -> ICCE_STE_R {
        ICCE_STE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn hac_ste(&mut self) -> HAC_STE_W<CQISE_SPEC> {
        HAC_STE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Task complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn tcc_ste(&mut self) -> TCC_STE_W<CQISE_SPEC> {
        TCC_STE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Response error detected interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn red_ste(&mut self) -> RED_STE_W<CQISE_SPEC> {
        RED_STE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Task cleared interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn tcl_ste(&mut self) -> TCL_STE_W<CQISE_SPEC> {
        TCL_STE_W::new(self, 3)
    }
    #[doc = "Bit 4 - General Crypto Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.GCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.GCE is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn gce_ste(&mut self) -> GCE_STE_W<CQISE_SPEC> {
        GCE_STE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Invalid Crypto Configuration Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.ICCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.ICCE is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn icce_ste(&mut self) -> ICCE_STE_W<CQISE_SPEC> {
        ICCE_STE_W::new(self, 5)
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
#[doc = "Command Queuing Interrupt Status Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqise::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqise::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQISE_SPEC;
impl crate::RegisterSpec for CQISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqise::R`](R) reader structure"]
impl crate::Readable for CQISE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cqise::W`](W) writer structure"]
impl crate::Writable for CQISE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CQISE to value 0"]
impl crate::Resettable for CQISE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
