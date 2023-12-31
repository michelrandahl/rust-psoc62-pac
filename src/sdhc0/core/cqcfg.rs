#[doc = "Register `CQCFG` reader"]
pub type R = crate::R<CQCFG_SPEC>;
#[doc = "Register `CQCFG` writer"]
pub type W = crate::W<CQCFG_SPEC>;
#[doc = "Field `CQ_EN` reader - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
pub type CQ_EN_R = crate::BitReader;
#[doc = "Field `CQ_EN` writer - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
pub type CQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_GENERAL_EN` reader - N/A"]
pub type CR_GENERAL_EN_R = crate::BitReader;
#[doc = "Field `CR_GENERAL_EN` writer - N/A"]
pub type CR_GENERAL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_DESC_SIZE` reader - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
pub type TASK_DESC_SIZE_R = crate::BitReader;
#[doc = "Field `TASK_DESC_SIZE` writer - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
pub type TASK_DESC_SIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMD_EN` reader - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
pub type DCMD_EN_R = crate::BitReader;
#[doc = "Field `DCMD_EN` writer - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
pub type DCMD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
    #[inline(always)]
    pub fn cq_en(&self) -> CQ_EN_R {
        CQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn cr_general_en(&self) -> CR_GENERAL_EN_R {
        CR_GENERAL_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
    #[inline(always)]
    pub fn task_desc_size(&self) -> TASK_DESC_SIZE_R {
        TASK_DESC_SIZE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
    #[inline(always)]
    pub fn dcmd_en(&self) -> DCMD_EN_R {
        DCMD_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
    #[inline(always)]
    #[must_use]
    pub fn cq_en(&mut self) -> CQ_EN_W<CQCFG_SPEC> {
        CQ_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cr_general_en(&mut self) -> CR_GENERAL_EN_W<CQCFG_SPEC> {
        CR_GENERAL_EN_W::new(self, 1)
    }
    #[doc = "Bit 8 - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
    #[inline(always)]
    #[must_use]
    pub fn task_desc_size(&mut self) -> TASK_DESC_SIZE_W<CQCFG_SPEC> {
        TASK_DESC_SIZE_W::new(self, 8)
    }
    #[doc = "Bit 12 - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
    #[inline(always)]
    #[must_use]
    pub fn dcmd_en(&mut self) -> DCMD_EN_W<CQCFG_SPEC> {
        DCMD_EN_W::new(self, 12)
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
#[doc = "Command Queuing Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQCFG_SPEC;
impl crate::RegisterSpec for CQCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcfg::R`](R) reader structure"]
impl crate::Readable for CQCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cqcfg::W`](W) writer structure"]
impl crate::Writable for CQCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CQCFG to value 0"]
impl crate::Resettable for CQCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
