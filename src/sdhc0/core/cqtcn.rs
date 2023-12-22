#[doc = "Register `CQTCN` reader"]
pub type R = crate::R<CQTCN_SPEC>;
#[doc = "Register `CQTCN` writer"]
pub type W = crate::W<CQTCN_SPEC>;
#[doc = "Field `TCN` reader - Task Completion Notification Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Task-N has completed execution (with success or errors) - Bit-N(0): Task-N has not completed, could be pending or not submitted. On task completion, software may read this register to know tasks that have completed. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
pub type TCN_R = crate::FieldReader<u32>;
#[doc = "Field `TCN` writer - Task Completion Notification Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Task-N has completed execution (with success or errors) - Bit-N(0): Task-N has not completed, could be pending or not submitted. On task completion, software may read this register to know tasks that have completed. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
pub type TCN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Task Completion Notification Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Task-N has completed execution (with success or errors) - Bit-N(0): Task-N has not completed, could be pending or not submitted. On task completion, software may read this register to know tasks that have completed. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
    #[inline(always)]
    pub fn tcn(&self) -> TCN_R {
        TCN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Task Completion Notification Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Task-N has completed execution (with success or errors) - Bit-N(0): Task-N has not completed, could be pending or not submitted. On task completion, software may read this register to know tasks that have completed. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
    #[inline(always)]
    #[must_use]
    pub fn tcn(&mut self) -> TCN_W<CQTCN_SPEC> {
        TCN_W::new(self, 0)
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
#[doc = "Command Queuing TaskClear Notification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqtcn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqtcn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQTCN_SPEC;
impl crate::RegisterSpec for CQTCN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqtcn::R`](R) reader structure"]
impl crate::Readable for CQTCN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cqtcn::W`](W) writer structure"]
impl crate::Writable for CQTCN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CQTCN to value 0"]
impl crate::Resettable for CQTCN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
