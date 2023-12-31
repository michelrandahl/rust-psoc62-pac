#[doc = "Register `CQRMEM` reader"]
pub type R = crate::R<CQRMEM_SPEC>;
#[doc = "Register `CQRMEM` writer"]
pub type W = crate::W<CQRMEM_SPEC>;
#[doc = "Field `RESP_ERR_MASK` reader - The bits of this field are bit mapped to the device response. This bit is used as an interrupt mask on the device status filed that is received in R1/R1b responses. - 1: When a R1/R1b response is received, with a bit i in the device status set, a RED interrupt is generated. - 0: When a R1/R1b response is received, bit i in the device status is ignored. The reset value of this register is set to trigger an interrupt on all 'Error' type bits in the device status. Note: Responses to CMD13 (SQS) encode the QSR so that they are ignored by this logic."]
pub type RESP_ERR_MASK_R = crate::FieldReader<u32>;
#[doc = "Field `RESP_ERR_MASK` writer - The bits of this field are bit mapped to the device response. This bit is used as an interrupt mask on the device status filed that is received in R1/R1b responses. - 1: When a R1/R1b response is received, with a bit i in the device status set, a RED interrupt is generated. - 0: When a R1/R1b response is received, bit i in the device status is ignored. The reset value of this register is set to trigger an interrupt on all 'Error' type bits in the device status. Note: Responses to CMD13 (SQS) encode the QSR so that they are ignored by this logic."]
pub type RESP_ERR_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits of this field are bit mapped to the device response. This bit is used as an interrupt mask on the device status filed that is received in R1/R1b responses. - 1: When a R1/R1b response is received, with a bit i in the device status set, a RED interrupt is generated. - 0: When a R1/R1b response is received, bit i in the device status is ignored. The reset value of this register is set to trigger an interrupt on all 'Error' type bits in the device status. Note: Responses to CMD13 (SQS) encode the QSR so that they are ignored by this logic."]
    #[inline(always)]
    pub fn resp_err_mask(&self) -> RESP_ERR_MASK_R {
        RESP_ERR_MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits of this field are bit mapped to the device response. This bit is used as an interrupt mask on the device status filed that is received in R1/R1b responses. - 1: When a R1/R1b response is received, with a bit i in the device status set, a RED interrupt is generated. - 0: When a R1/R1b response is received, bit i in the device status is ignored. The reset value of this register is set to trigger an interrupt on all 'Error' type bits in the device status. Note: Responses to CMD13 (SQS) encode the QSR so that they are ignored by this logic."]
    #[inline(always)]
    #[must_use]
    pub fn resp_err_mask(&mut self) -> RESP_ERR_MASK_W<CQRMEM_SPEC> {
        RESP_ERR_MASK_W::new(self, 0)
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
#[doc = "Command response mode error mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqrmem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqrmem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQRMEM_SPEC;
impl crate::RegisterSpec for CQRMEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqrmem::R`](R) reader structure"]
impl crate::Readable for CQRMEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cqrmem::W`](W) writer structure"]
impl crate::Writable for CQRMEM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CQRMEM to value 0xfdf9_a080"]
impl crate::Resettable for CQRMEM_SPEC {
    const RESET_VALUE: Self::Ux = 0xfdf9_a080;
}
