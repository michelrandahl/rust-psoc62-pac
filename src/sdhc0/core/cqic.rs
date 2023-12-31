#[doc = "Register `CQIC` reader"]
pub type R = crate::R<CQIC_SPEC>;
#[doc = "Register `CQIC` writer"]
pub type W = crate::W<CQIC_SPEC>;
#[doc = "Field `TOUT_VAL` reader - Interrupt Coalescing Timeout Value Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when the first data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field, it generates an interrupt and stops. The timer's unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. - 0x0: Timer is disabled. Timeout-based interrupt is not generated - 0x1: Timeout on 01x1024 cycles of timer clock frequency - 0x2: Timeout on 02x1024 cycles of timer clock frequency - ........ - 0x7f: Timeout on 127x1024 cycles of timer clock frequency In order to write to this field, the TOUT_VAL_WEN bit must be set at the same write operation."]
pub type TOUT_VAL_R = crate::FieldReader;
#[doc = "Field `TOUT_VAL` writer - Interrupt Coalescing Timeout Value Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when the first data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field, it generates an interrupt and stops. The timer's unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. - 0x0: Timer is disabled. Timeout-based interrupt is not generated - 0x1: Timeout on 01x1024 cycles of timer clock frequency - 0x2: Timeout on 02x1024 cycles of timer clock frequency - ........ - 0x7f: Timeout on 127x1024 cycles of timer clock frequency In order to write to this field, the TOUT_VAL_WEN bit must be set at the same write operation."]
pub type TOUT_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TOUT_VAL_WEN` writer - When software writes 1 to this bit, the value TOUT_VAL is updated with the contents written on the same cycle. Values: - 0x1 (WEN_SET): Sets TOUT_VAL_WEN - 0x0 (WEN_CLR): clears TOUT_VAL_WEN"]
pub type TOUT_VAL_WEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTC_TH` writer - Interrupt Coalescing Counter Threshold filed Software uses this field to configure the number of task completions (only tasks with INT=0 in the Task Descriptor), which are required in order to generate an interrupt. Counter Operation: As data transfer tasks with INT=0 complete, they are counted by CQE. The counter is reset by software during the interrupt service routine. The counter stops counting when it reaches the value configured in INTC_TH, and generates interrupt. - 0x0: Interrupt coalescing feature disabled - 0x1: Interrupt coalescing interrupt generated after 1 task when INT=0 completes - 0x2: Interrupt coalescing interrupt generated after 2 tasks when INT=0 completes - ........ - 0x1f: Interrupt coalescing interrupt generated after 31 tasks when INT=0 completes To write to this field, the INTC_TH_WEN bit must be set during the same write operation."]
pub type INTC_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INTC_TH_WEN` writer - Interrupt Coalescing Counter Threshold Write Enable When software writes 1 to this bit, the value INTC_TH is updated with the contents written on the same cycle. Values: - 0x1 (WEN_SET): Sets INTC_TH_WEN - 0x0 (WEN_CLR): Clears INTC_TH_WEN"]
pub type INTC_TH_WEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTC_RST` writer - Counter and Timer Reset When host driver writes 1, the interrupt coalescing timer and counter are reset. Values: - 0x1 (ASSERT_INTC_RESET): Interrupt coalescing timer and counter are reset - 0x0 (NO_EFFECT): No Effect"]
pub type INTC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTC_STAT` reader - Interrupt Coalescing Status Bit This bit indicates to the software whether any tasks (with INT=0) have completed and counted towards interrupt coalescing (that is, this is set if and only if INTC counter > 0). Values: - 0x1 (INTC_ATLEAST1_COMP): At least one INT0 task completion has been counted (INTC counter > 0) - 0x0 (INTC_NO_TASK_COMP): INT0 Task completions have not occurred since last counter reset (INTC counter == 0)"]
pub type INTC_STAT_R = crate::BitReader;
#[doc = "Field `INTC_EN` reader - Interrupt Coalescing Enable Bit Values: - 0x1 (ENABLE_INT_COALESCING): Interrupt coalescing mechanism is active. Interrupts are counted and timed, and coalesced interrupts are generated - 0x0 (DISABLE_INT_COALESCING): Interrupt coalescing mechanism is disabled (Default)."]
pub type INTC_EN_R = crate::BitReader;
#[doc = "Field `INTC_EN` writer - Interrupt Coalescing Enable Bit Values: - 0x1 (ENABLE_INT_COALESCING): Interrupt coalescing mechanism is active. Interrupts are counted and timed, and coalesced interrupts are generated - 0x0 (DISABLE_INT_COALESCING): Interrupt coalescing mechanism is disabled (Default)."]
pub type INTC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Interrupt Coalescing Timeout Value Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when the first data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field, it generates an interrupt and stops. The timer's unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. - 0x0: Timer is disabled. Timeout-based interrupt is not generated - 0x1: Timeout on 01x1024 cycles of timer clock frequency - 0x2: Timeout on 02x1024 cycles of timer clock frequency - ........ - 0x7f: Timeout on 127x1024 cycles of timer clock frequency In order to write to this field, the TOUT_VAL_WEN bit must be set at the same write operation."]
    #[inline(always)]
    pub fn tout_val(&self) -> TOUT_VAL_R {
        TOUT_VAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 20 - Interrupt Coalescing Status Bit This bit indicates to the software whether any tasks (with INT=0) have completed and counted towards interrupt coalescing (that is, this is set if and only if INTC counter > 0). Values: - 0x1 (INTC_ATLEAST1_COMP): At least one INT0 task completion has been counted (INTC counter > 0) - 0x0 (INTC_NO_TASK_COMP): INT0 Task completions have not occurred since last counter reset (INTC counter == 0)"]
    #[inline(always)]
    pub fn intc_stat(&self) -> INTC_STAT_R {
        INTC_STAT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable Bit Values: - 0x1 (ENABLE_INT_COALESCING): Interrupt coalescing mechanism is active. Interrupts are counted and timed, and coalesced interrupts are generated - 0x0 (DISABLE_INT_COALESCING): Interrupt coalescing mechanism is disabled (Default)."]
    #[inline(always)]
    pub fn intc_en(&self) -> INTC_EN_R {
        INTC_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Interrupt Coalescing Timeout Value Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when the first data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field, it generates an interrupt and stops. The timer's unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. - 0x0: Timer is disabled. Timeout-based interrupt is not generated - 0x1: Timeout on 01x1024 cycles of timer clock frequency - 0x2: Timeout on 02x1024 cycles of timer clock frequency - ........ - 0x7f: Timeout on 127x1024 cycles of timer clock frequency In order to write to this field, the TOUT_VAL_WEN bit must be set at the same write operation."]
    #[inline(always)]
    #[must_use]
    pub fn tout_val(&mut self) -> TOUT_VAL_W<CQIC_SPEC> {
        TOUT_VAL_W::new(self, 0)
    }
    #[doc = "Bit 7 - When software writes 1 to this bit, the value TOUT_VAL is updated with the contents written on the same cycle. Values: - 0x1 (WEN_SET): Sets TOUT_VAL_WEN - 0x0 (WEN_CLR): clears TOUT_VAL_WEN"]
    #[inline(always)]
    #[must_use]
    pub fn tout_val_wen(&mut self) -> TOUT_VAL_WEN_W<CQIC_SPEC> {
        TOUT_VAL_WEN_W::new(self, 7)
    }
    #[doc = "Bits 8:12 - Interrupt Coalescing Counter Threshold filed Software uses this field to configure the number of task completions (only tasks with INT=0 in the Task Descriptor), which are required in order to generate an interrupt. Counter Operation: As data transfer tasks with INT=0 complete, they are counted by CQE. The counter is reset by software during the interrupt service routine. The counter stops counting when it reaches the value configured in INTC_TH, and generates interrupt. - 0x0: Interrupt coalescing feature disabled - 0x1: Interrupt coalescing interrupt generated after 1 task when INT=0 completes - 0x2: Interrupt coalescing interrupt generated after 2 tasks when INT=0 completes - ........ - 0x1f: Interrupt coalescing interrupt generated after 31 tasks when INT=0 completes To write to this field, the INTC_TH_WEN bit must be set during the same write operation."]
    #[inline(always)]
    #[must_use]
    pub fn intc_th(&mut self) -> INTC_TH_W<CQIC_SPEC> {
        INTC_TH_W::new(self, 8)
    }
    #[doc = "Bit 15 - Interrupt Coalescing Counter Threshold Write Enable When software writes 1 to this bit, the value INTC_TH is updated with the contents written on the same cycle. Values: - 0x1 (WEN_SET): Sets INTC_TH_WEN - 0x0 (WEN_CLR): Clears INTC_TH_WEN"]
    #[inline(always)]
    #[must_use]
    pub fn intc_th_wen(&mut self) -> INTC_TH_WEN_W<CQIC_SPEC> {
        INTC_TH_WEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Counter and Timer Reset When host driver writes 1, the interrupt coalescing timer and counter are reset. Values: - 0x1 (ASSERT_INTC_RESET): Interrupt coalescing timer and counter are reset - 0x0 (NO_EFFECT): No Effect"]
    #[inline(always)]
    #[must_use]
    pub fn intc_rst(&mut self) -> INTC_RST_W<CQIC_SPEC> {
        INTC_RST_W::new(self, 16)
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable Bit Values: - 0x1 (ENABLE_INT_COALESCING): Interrupt coalescing mechanism is active. Interrupts are counted and timed, and coalesced interrupts are generated - 0x0 (DISABLE_INT_COALESCING): Interrupt coalescing mechanism is disabled (Default)."]
    #[inline(always)]
    #[must_use]
    pub fn intc_en(&mut self) -> INTC_EN_W<CQIC_SPEC> {
        INTC_EN_W::new(self, 31)
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
#[doc = "Command Queuing Interrupt Coalescing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqic::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqic::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQIC_SPEC;
impl crate::RegisterSpec for CQIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqic::R`](R) reader structure"]
impl crate::Readable for CQIC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cqic::W`](W) writer structure"]
impl crate::Writable for CQIC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CQIC to value 0"]
impl crate::Resettable for CQIC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
