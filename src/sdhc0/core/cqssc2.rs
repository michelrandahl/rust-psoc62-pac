#[doc = "Register `CQSSC2` reader"]
pub type R = crate::R<CQSSC2_SPEC>;
#[doc = "Register `CQSSC2` writer"]
pub type W = crate::W<CQSSC2_SPEC>;
#[doc = "Field `SQSCMD_RCA` reader - This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_STATUS (CMD13) command argument. CQE copies this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS (CMD13) command."]
pub type SQSCMD_RCA_R = crate::FieldReader<u16>;
#[doc = "Field `SQSCMD_RCA` writer - This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_STATUS (CMD13) command argument. CQE copies this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS (CMD13) command."]
pub type SQSCMD_RCA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_STATUS (CMD13) command argument. CQE copies this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS (CMD13) command."]
    #[inline(always)]
    pub fn sqscmd_rca(&self) -> SQSCMD_RCA_R {
        SQSCMD_RCA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_STATUS (CMD13) command argument. CQE copies this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS (CMD13) command."]
    #[inline(always)]
    #[must_use]
    pub fn sqscmd_rca(&mut self) -> SQSCMD_RCA_W<CQSSC2_SPEC> {
        SQSCMD_RCA_W::new(self, 0)
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
#[doc = "CQ Send Status Configuration 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqssc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqssc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQSSC2_SPEC;
impl crate::RegisterSpec for CQSSC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqssc2::R`](R) reader structure"]
impl crate::Readable for CQSSC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cqssc2::W`](W) writer structure"]
impl crate::Writable for CQSSC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CQSSC2 to value 0"]
impl crate::Resettable for CQSSC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
