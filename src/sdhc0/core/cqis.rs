#[doc = "Register `CQIS` reader"]
pub type R = crate::R<CQIS_SPEC>;
#[doc = "Register `CQIS` writer"]
pub type W = crate::W<CQIS_SPEC>;
#[doc = "Field `HAC` reader - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: - 0x1 (SET): HAC Interrupt is set - 0x0 (NOTSET): HAC Interrupt is not set"]
pub type HAC_R = crate::BitReader;
#[doc = "Field `HAC` writer - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: - 0x1 (SET): HAC Interrupt is set - 0x0 (NOTSET): HAC Interrupt is not set"]
pub type HAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` reader - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: - A task is completed and the INT bit is set in its Task Descriptor - Interrupt caused by Interrupt Coalescing logic due to timeout - Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit Values: - 0x1 (SET): TCC Interrupt is set - 0x0 (NOTSET): TCC Interrupt is not set"]
pub type TCC_R = crate::BitReader;
#[doc = "Field `TCC` writer - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: - A task is completed and the INT bit is set in its Task Descriptor - Interrupt caused by Interrupt Coalescing logic due to timeout - Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit Values: - 0x1 (SET): TCC Interrupt is set - 0x0 (NOTSET): TCC Interrupt is not set"]
pub type TCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED` reader - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: - 0x1 (SET): RED Interrupt is set - 0x0 (NOTSET): RED Interrupt is not set"]
pub type RED_R = crate::BitReader;
#[doc = "Field `RED` writer - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: - 0x1 (SET): RED Interrupt is set - 0x0 (NOTSET): RED Interrupt is not set"]
pub type RED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCL` reader - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: - 0x1 (SET): TCL Interrupt is set - 0x0 (NOTSET): TCL Interrupt is not set"]
pub type TCL_R = crate::BitReader;
#[doc = "Field `TCL` writer - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: - 0x1 (SET): TCL Interrupt is set - 0x0 (NOTSET): TCL Interrupt is not set"]
pub type TCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCE` reader - N/A"]
pub type GCE_R = crate::BitReader;
#[doc = "Field `GCE` writer - N/A"]
pub type GCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICCE` reader - N/A"]
pub type ICCE_R = crate::BitReader;
#[doc = "Field `ICCE` writer - N/A"]
pub type ICCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: - 0x1 (SET): HAC Interrupt is set - 0x0 (NOTSET): HAC Interrupt is not set"]
    #[inline(always)]
    pub fn hac(&self) -> HAC_R {
        HAC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: - A task is completed and the INT bit is set in its Task Descriptor - Interrupt caused by Interrupt Coalescing logic due to timeout - Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit Values: - 0x1 (SET): TCC Interrupt is set - 0x0 (NOTSET): TCC Interrupt is not set"]
    #[inline(always)]
    pub fn tcc(&self) -> TCC_R {
        TCC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: - 0x1 (SET): RED Interrupt is set - 0x0 (NOTSET): RED Interrupt is not set"]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: - 0x1 (SET): TCL Interrupt is set - 0x0 (NOTSET): TCL Interrupt is not set"]
    #[inline(always)]
    pub fn tcl(&self) -> TCL_R {
        TCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn gce(&self) -> GCE_R {
        GCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn icce(&self) -> ICCE_R {
        ICCE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: - 0x1 (SET): HAC Interrupt is set - 0x0 (NOTSET): HAC Interrupt is not set"]
    #[inline(always)]
    #[must_use]
    pub fn hac(&mut self) -> HAC_W<CQIS_SPEC> {
        HAC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: - A task is completed and the INT bit is set in its Task Descriptor - Interrupt caused by Interrupt Coalescing logic due to timeout - Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit Values: - 0x1 (SET): TCC Interrupt is set - 0x0 (NOTSET): TCC Interrupt is not set"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TCC_W<CQIS_SPEC> {
        TCC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: - 0x1 (SET): RED Interrupt is set - 0x0 (NOTSET): RED Interrupt is not set"]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<CQIS_SPEC> {
        RED_W::new(self, 2)
    }
    #[doc = "Bit 3 - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: - 0x1 (SET): TCL Interrupt is set - 0x0 (NOTSET): TCL Interrupt is not set"]
    #[inline(always)]
    #[must_use]
    pub fn tcl(&mut self) -> TCL_W<CQIS_SPEC> {
        TCL_W::new(self, 3)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn gce(&mut self) -> GCE_W<CQIS_SPEC> {
        GCE_W::new(self, 4)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn icce(&mut self) -> ICCE_W<CQIS_SPEC> {
        ICCE_W::new(self, 5)
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
#[doc = "Command Queuing Interrupt Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQIS_SPEC;
impl crate::RegisterSpec for CQIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqis::R`](R) reader structure"]
impl crate::Readable for CQIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cqis::W`](W) writer structure"]
impl crate::Writable for CQIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CQIS to value 0"]
impl crate::Resettable for CQIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
