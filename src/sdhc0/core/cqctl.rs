#[doc = "Register `CQCTL` reader"]
pub type R = crate::R<CQCTL_SPEC>;
#[doc = "Register `CQCTL` writer"]
pub type W = crate::W<CQCTL_SPEC>;
#[doc = "Field `HALT` reader - Halt request and resume Values: - 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. - 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity."]
pub type HALT_R = crate::BitReader;
#[doc = "Field `HALT` writer - Halt request and resume Values: - 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. - 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity."]
pub type HALT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_ALL_TASKS` reader - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: - 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller - 0x0 (NO_EFFECT): Programming 0 has no effect"]
pub type CLR_ALL_TASKS_R = crate::BitReader;
#[doc = "Field `CLR_ALL_TASKS` writer - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: - 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller - 0x0 (NO_EFFECT): Programming 0 has no effect"]
pub type CLR_ALL_TASKS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Halt request and resume Values: - 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. - 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity."]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: - 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller - 0x0 (NO_EFFECT): Programming 0 has no effect"]
    #[inline(always)]
    pub fn clr_all_tasks(&self) -> CLR_ALL_TASKS_R {
        CLR_ALL_TASKS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt request and resume Values: - 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. - 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity."]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<CQCTL_SPEC> {
        HALT_W::new(self, 0)
    }
    #[doc = "Bit 8 - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: - 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller - 0x0 (NO_EFFECT): Programming 0 has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn clr_all_tasks(&mut self) -> CLR_ALL_TASKS_W<CQCTL_SPEC> {
        CLR_ALL_TASKS_W::new(self, 8)
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
#[doc = "Command Queuing Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQCTL_SPEC;
impl crate::RegisterSpec for CQCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqctl::R`](R) reader structure"]
impl crate::Readable for CQCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cqctl::W`](W) writer structure"]
impl crate::Writable for CQCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CQCTL to value 0"]
impl crate::Resettable for CQCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
