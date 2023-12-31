#[doc = "Register `SIE_EP3_CNT1` reader"]
pub type R = crate::R<SIE_EP3_CNT1_SPEC>;
#[doc = "Register `SIE_EP3_CNT1` writer"]
pub type W = crate::W<SIE_EP3_CNT1_SPEC>;
#[doc = "Field `DATA_COUNT` reader - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
pub type DATA_COUNT_R = crate::FieldReader;
#[doc = "Field `DATA_COUNT` writer - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
pub type DATA_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn data_count(&self) -> DATA_COUNT_R {
        DATA_COUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    #[must_use]
    pub fn data_count(&mut self) -> DATA_COUNT_W<SIE_EP3_CNT1_SPEC> {
        DATA_COUNT_W::new(self, 0)
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
#[doc = "Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep3_cnt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep3_cnt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIE_EP3_CNT1_SPEC;
impl crate::RegisterSpec for SIE_EP3_CNT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sie_ep3_cnt1::R`](R) reader structure"]
impl crate::Readable for SIE_EP3_CNT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sie_ep3_cnt1::W`](W) writer structure"]
impl crate::Writable for SIE_EP3_CNT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIE_EP3_CNT1 to value 0"]
impl crate::Resettable for SIE_EP3_CNT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
