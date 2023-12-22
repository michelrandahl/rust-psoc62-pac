#[doc = "Register `CQTDLBA` reader"]
pub type R = crate::R<CQTDLBA_SPEC>;
#[doc = "Register `CQTDLBA` writer"]
pub type W = crate::W<CQTDLBA_SPEC>;
#[doc = "Field `TDLBA` reader - This register stores the LSB bits (31:0) of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * (Task Descriptor size + Transfer Descriptor size) as configured by the host driver. This address is set on 1 KB boundary. The lower 10 bits of this register are set to 0 by the software and are ignored by CQE."]
pub type TDLBA_R = crate::FieldReader<u32>;
#[doc = "Field `TDLBA` writer - This register stores the LSB bits (31:0) of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * (Task Descriptor size + Transfer Descriptor size) as configured by the host driver. This address is set on 1 KB boundary. The lower 10 bits of this register are set to 0 by the software and are ignored by CQE."]
pub type TDLBA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the LSB bits (31:0) of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * (Task Descriptor size + Transfer Descriptor size) as configured by the host driver. This address is set on 1 KB boundary. The lower 10 bits of this register are set to 0 by the software and are ignored by CQE."]
    #[inline(always)]
    pub fn tdlba(&self) -> TDLBA_R {
        TDLBA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the LSB bits (31:0) of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * (Task Descriptor size + Transfer Descriptor size) as configured by the host driver. This address is set on 1 KB boundary. The lower 10 bits of this register are set to 0 by the software and are ignored by CQE."]
    #[inline(always)]
    #[must_use]
    pub fn tdlba(&mut self) -> TDLBA_W<CQTDLBA_SPEC> {
        TDLBA_W::new(self, 0)
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
#[doc = "Command Queuing Task Descriptor List Base Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqtdlba::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqtdlba::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQTDLBA_SPEC;
impl crate::RegisterSpec for CQTDLBA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqtdlba::R`](R) reader structure"]
impl crate::Readable for CQTDLBA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cqtdlba::W`](W) writer structure"]
impl crate::Writable for CQTDLBA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CQTDLBA to value 0"]
impl crate::Resettable for CQTDLBA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
