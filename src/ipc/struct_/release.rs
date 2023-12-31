#[doc = "Register `RELEASE` writer"]
pub type W = crate::W<RELEASE_SPEC>;
#[doc = "Field `INTR_RELEASE` writer - Writing this field releases a lock and allows for the generation of release events to the IPC interrupt structures, but only when the lock is acquired (LOCK_STATUS.ACQUIRED is '1'). The IPC release cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_RELEASE\\[\\]
is set to '1'. SW writes a '1' to the bit fields to generate a release event. Due to the transient nature of this event, SW always reads a '0' from this field."]
pub type INTR_RELEASE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Writing this field releases a lock and allows for the generation of release events to the IPC interrupt structures, but only when the lock is acquired (LOCK_STATUS.ACQUIRED is '1'). The IPC release cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_RELEASE\\[\\]
is set to '1'. SW writes a '1' to the bit fields to generate a release event. Due to the transient nature of this event, SW always reads a '0' from this field."]
    #[inline(always)]
    #[must_use]
    pub fn intr_release(&mut self) -> INTR_RELEASE_W<RELEASE_SPEC> {
        INTR_RELEASE_W::new(self, 0)
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
#[doc = "IPC release\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`release::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RELEASE_SPEC;
impl crate::RegisterSpec for RELEASE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`release::W`](W) writer structure"]
impl crate::Writable for RELEASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RELEASE to value 0"]
impl crate::Resettable for RELEASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
