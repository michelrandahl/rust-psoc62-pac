#[doc = "Register `PROTECTION` reader"]
pub type R = crate::R<PROTECTION_SPEC>;
#[doc = "Register `PROTECTION` writer"]
pub type W = crate::W<PROTECTION_SPEC>;
#[doc = "Field `STATE` reader - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
pub type STATE_R = crate::FieldReader;
#[doc = "Field `STATE` writer - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
pub type STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> STATE_W<PROTECTION_SPEC> {
        STATE_W::new(self, 0)
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
#[doc = "Protection status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`protection::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`protection::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROTECTION_SPEC;
impl crate::RegisterSpec for PROTECTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`protection::R`](R) reader structure"]
impl crate::Readable for PROTECTION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`protection::W`](W) writer structure"]
impl crate::Writable for PROTECTION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PROTECTION to value 0"]
impl crate::Resettable for PROTECTION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
