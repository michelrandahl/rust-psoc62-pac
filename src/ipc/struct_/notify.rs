#[doc = "Register `NOTIFY` writer"]
pub type W = crate::W<NOTIFY_SPEC>;
#[doc = "Field `INTR_NOTIFY` writer - This field allows for the generation of notification events to the IPC interrupt structures. The IPC notification cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_NOTIFY\\[\\]
is set to '1'. SW writes a '1' to the bit fields to generate a notify event. Due to the transient nature of this event, SW always reads a '0' from this field."]
pub type INTR_NOTIFY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - This field allows for the generation of notification events to the IPC interrupt structures. The IPC notification cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_NOTIFY\\[\\]
is set to '1'. SW writes a '1' to the bit fields to generate a notify event. Due to the transient nature of this event, SW always reads a '0' from this field."]
    #[inline(always)]
    #[must_use]
    pub fn intr_notify(&mut self) -> INTR_NOTIFY_W<NOTIFY_SPEC> {
        INTR_NOTIFY_W::new(self, 0)
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
#[doc = "IPC notification\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`notify::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NOTIFY_SPEC;
impl crate::RegisterSpec for NOTIFY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`notify::W`](W) writer structure"]
impl crate::Writable for NOTIFY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NOTIFY to value 0"]
impl crate::Resettable for NOTIFY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
