#[doc = "Register `CH_CTL` reader"]
pub type R = crate::R<CH_CTL_SPEC>;
#[doc = "Register `CH_CTL` writer"]
pub type W = crate::W<CH_CTL_SPEC>;
#[doc = "Field `P` reader - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
pub type P_R = crate::BitReader;
#[doc = "Field `P` writer - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
pub type P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS` reader - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
pub type NS_R = crate::BitReader;
#[doc = "Field `NS` writer - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
pub type NS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B` reader - Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
pub type B_R = crate::BitReader;
#[doc = "Field `B` writer - Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
pub type B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC` reader - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the 'write data' is ignored and instead the context is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel uses the PC field for the protection context."]
pub type PC_R = crate::FieldReader;
#[doc = "Field `PC` writer - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the 'write data' is ignored and instead the context is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel uses the PC field for the protection context."]
pub type PC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIO` reader - Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority group with pending channels is identified. Second, within this priority group, round robin arbitration is applied. Round robin arbitration (within a priority group) gives the highest priority to the lower channel indices (within the priority group)."]
pub type PRIO_R = crate::FieldReader;
#[doc = "Field `PRIO` writer - Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority group with pending channels is identified. Second, within this priority group, round robin arbitration is applied. Round robin arbitration (within a priority group) gives the highest priority to the lower channel indices (within the priority group)."]
pub type PRIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PREEMPTABLE` reader - Specifies if the channel is preemptable. '0': Not preemptable. '1': Preemptable. This field allows higher priority pending channels (from a higher priority group; i.e. an active channel can NOT be preempted by a pending channel in the same priority group) to preempt the active channel in between 'single transfers' (a 1D transfer consists out of X_COUNT single transfers; a 2D transfer consists out of X_COUNT*Y_COUNT single transfers). Preemption will NOT affect the pending status of channel. As a result, after completion of a higher priority activated channel, the current channel may be reactivated."]
pub type PREEMPTABLE_R = crate::BitReader;
#[doc = "Field `PREEMPTABLE` writer - Specifies if the channel is preemptable. '0': Not preemptable. '1': Preemptable. This field allows higher priority pending channels (from a higher priority group; i.e. an active channel can NOT be preempted by a pending channel in the same priority group) to preempt the active channel in between 'single transfers' (a 1D transfer consists out of X_COUNT single transfers; a 2D transfer consists out of X_COUNT*Y_COUNT single transfers). Preemption will NOT affect the pending status of channel. As a result, after completion of a higher priority activated channel, the current channel may be reactivated."]
pub type PREEMPTABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' on an error interrupt cause (the specific error is specified by CH_STATUS.INTR_CAUSE)."]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' on an error interrupt cause (the specific error is specified by CH_STATUS.INTR_CAUSE)."]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the 'write data' is ignored and instead the context is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel uses the PC field for the protection context."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority group with pending channels is identified. Second, within this priority group, round robin arbitration is applied. Round robin arbitration (within a priority group) gives the highest priority to the lower channel indices (within the priority group)."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Specifies if the channel is preemptable. '0': Not preemptable. '1': Preemptable. This field allows higher priority pending channels (from a higher priority group; i.e. an active channel can NOT be preempted by a pending channel in the same priority group) to preempt the active channel in between 'single transfers' (a 1D transfer consists out of X_COUNT single transfers; a 2D transfer consists out of X_COUNT*Y_COUNT single transfers). Preemption will NOT affect the pending status of channel. As a result, after completion of a higher priority activated channel, the current channel may be reactivated."]
    #[inline(always)]
    pub fn preemptable(&self) -> PREEMPTABLE_R {
        PREEMPTABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' on an error interrupt cause (the specific error is specified by CH_STATUS.INTR_CAUSE)."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<CH_CTL_SPEC> {
        P_W::new(self, 0)
    }
    #[doc = "Bit 1 - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NS_W<CH_CTL_SPEC> {
        NS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<CH_CTL_SPEC> {
        B_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the 'write data' is ignored and instead the context is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel uses the PC field for the protection context."]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PC_W<CH_CTL_SPEC> {
        PC_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority group with pending channels is identified. Second, within this priority group, round robin arbitration is applied. Round robin arbitration (within a priority group) gives the highest priority to the lower channel indices (within the priority group)."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<CH_CTL_SPEC> {
        PRIO_W::new(self, 8)
    }
    #[doc = "Bit 11 - Specifies if the channel is preemptable. '0': Not preemptable. '1': Preemptable. This field allows higher priority pending channels (from a higher priority group; i.e. an active channel can NOT be preempted by a pending channel in the same priority group) to preempt the active channel in between 'single transfers' (a 1D transfer consists out of X_COUNT single transfers; a 2D transfer consists out of X_COUNT*Y_COUNT single transfers). Preemption will NOT affect the pending status of channel. As a result, after completion of a higher priority activated channel, the current channel may be reactivated."]
    #[inline(always)]
    #[must_use]
    pub fn preemptable(&mut self) -> PREEMPTABLE_W<CH_CTL_SPEC> {
        PREEMPTABLE_W::new(self, 11)
    }
    #[doc = "Bit 31 - Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' on an error interrupt cause (the specific error is specified by CH_STATUS.INTR_CAUSE)."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<CH_CTL_SPEC> {
        ENABLED_W::new(self, 31)
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
#[doc = "Channel control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_CTL_SPEC;
impl crate::RegisterSpec for CH_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_ctl::R`](R) reader structure"]
impl crate::Readable for CH_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_ctl::W`](W) writer structure"]
impl crate::Writable for CH_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH_CTL to value 0x02"]
impl crate::Resettable for CH_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
